use std::fmt::{self, Display, Formatter};
use std::sync::{atomic::AtomicBool, atomic::Ordering};

use rayon::prelude::*;

use crate::circuit::{Gate, PullApplyOutput, PushApplicable, PushApplyOutput};
use crate::config::Config;
use crate::types::{AtomicBasisIdx, BasisIdx, Complex, Real};
use crate::utility;

use super::super::expected_cost;
use super::state::{DenseStateTable, SparseStateTable, State};

pub enum ExpandMethod {
    Sparse,
    PushDense,
    PullDense,
}

impl Display for ExpandMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ExpandMethod::Sparse => write!(f, "push sparse"),
            ExpandMethod::PushDense => write!(f, "push dense"),
            ExpandMethod::PullDense => write!(f, "pull dense"),
        }
    }
}

pub struct ExpandResult<B: BasisIdx, AB: AtomicBasisIdx<B>> {
    pub state: State<B, AB>,
    pub num_nonzeros: usize,
    pub num_gate_apps: usize,
    pub method: ExpandMethod,
}

pub fn expand<B: BasisIdx, AB: AtomicBasisIdx<B>>(
    gates: Vec<&Gate<B>>,
    config: &Config,
    num_qubits: usize,
    prev_num_nonzeros: usize,
    state: State<B, AB>,
) -> ExpandResult<B, AB> {
    let (expected_density, expected_num_nonzeros) =
        expected_cost(num_qubits, state.num_nonzeros(), prev_num_nonzeros);

    let all_gates_pullable = gates.iter().all(|gate| gate.is_pullable());

    assert!(config.dense_threshold <= config.pull_threshold);

    if expected_density < config.dense_threshold {
        expand_sparse(gates, num_qubits, config, expected_num_nonzeros, &state)
    } else if expected_density >= config.pull_threshold && all_gates_pullable {
        expand_pull_dense(gates, num_qubits, state)
    } else {
        expand_push_dense(gates, num_qubits, state)
    }
}

enum SuccessorsResult<B: BasisIdx> {
    AllSucceeded,
    SomeFailed(Vec<(B, Complex, usize)>),
}

fn apply_gates1<B: BasisIdx, AB: AtomicBasisIdx<B>>(
    gatenum: usize,
    gates: &[&Gate<B>],
    table: &SparseStateTable<B, AB>,
    bidx: B,
    weight: Complex,
    is_full: &AtomicBool,
    apps: usize,
    maxload: Real,
) -> (usize, SuccessorsResult<B>) {
    if utility::is_zero(weight) {
        return (apps, SuccessorsResult::AllSucceeded);
    }
    if gatenum >= gates.len() {
        if !is_full.load(Ordering::Relaxed) {
            match table.try_put(bidx.clone(), weight, maxload) {
                Ok(()) => return (apps, SuccessorsResult::AllSucceeded),
                Err(()) => (),
            }
        }
        if !is_full.load(Ordering::Relaxed) {
            is_full.store(true, Ordering::SeqCst);
        }
        return (
            apps,
            SuccessorsResult::SomeFailed(vec![(bidx, weight, gatenum)]),
        );
    }
    match gates[gatenum].push_apply(bidx, weight) {
        PushApplyOutput::Nonbranching(new_bidx, new_weight) => apply_gates1(
            gatenum + 1,
            gates,
            table,
            new_bidx,
            new_weight,
            is_full,
            apps + 1,
            maxload,
        ),
        PushApplyOutput::Branching((new_bidx1, new_weight1), (new_bidx2, new_weight2)) => {
            apply_gates2(
                gatenum + 1,
                gates,
                table,
                new_bidx1,
                new_weight1,
                new_bidx2,
                new_weight2,
                is_full,
                apps + 1,
                maxload,
            )
        }
    }
}

fn apply_gates2<B: BasisIdx, AB: AtomicBasisIdx<B>>(
    gatenum: usize,
    gates: &[&Gate<B>],
    table: &SparseStateTable<B, AB>,
    bidx1: B,
    weight1: Complex,
    bidx2: B,
    weight2: Complex,
    is_full: &AtomicBool,
    apps: usize,
    maxload: Real,
) -> (usize, SuccessorsResult<B>) {
    match apply_gates1(
        gatenum, gates, table, bidx1, weight1, is_full, apps, maxload,
    ) {
        (apps, SuccessorsResult::AllSucceeded) => apply_gates1(
            gatenum, gates, table, bidx2, weight2, is_full, apps, maxload,
        ),
        (apps, SuccessorsResult::SomeFailed(v)) => {
            let mut v2 = v.clone();
            v2.push((bidx2, weight2, gatenum));
            (apps, SuccessorsResult::SomeFailed(v2))
        }
    }
}

pub fn expand_sparse<B: BasisIdx, AB: AtomicBasisIdx<B>>(
    gates: Vec<&Gate<B>>,
    num_qubits: usize,
    config: &Config,
    expected_num_nonzeros: usize,
    state: &State<B, AB>,
) -> ExpandResult<B, AB> {
    let mut table = SparseStateTable::new(num_qubits, config.maxload, expected_num_nonzeros);
    let n: usize = match state {
        State::Sparse(prev_table) => prev_table.num_nonzeros(),
        State::Dense(prev_table) => prev_table.capacity(),
        State::Never(_, _) => unreachable!(),
    };
    let block_size = std::cmp::max(100, std::cmp::min(n / 1000, config.block_size));
    let num_blocks = (n as f64 / block_size as f64).ceil() as usize;
    let block_start = |b: usize| block_size * b;
    let block_stop = |b: usize| std::cmp::min(n, block_size + block_start(b));
    let mut blocks: Vec<(usize, usize, Vec<(B, Complex, usize)>)> = (0..num_blocks)
        .into_par_iter()
        .map(|b| (b, block_start(b), vec![]))
        .collect();
    let get: Box<dyn Fn(usize) -> (B, Complex) + Sync> = match state {
        State::Sparse(prev_table) => {
            let nonzeros = prev_table.nonzeros();
            Box::new(move |i: usize| nonzeros[i].clone())
        }
        State::Dense(prev_table) => Box::new(|i: usize| {
            let v = prev_table.array[i].load(Ordering::Relaxed);
            let weight = utility::unpack_complex(v);
            (B::from_idx(i), weight)
        }),
        State::Never(_, _) => unreachable!(),
    };

    while !blocks.is_empty() {
        let is_full: AtomicBool = AtomicBool::new(false);
        blocks = blocks
            .par_iter()
            // We process a given block b in two steps:
            .map(|(b, s, ps)| {
                // (1) we clear any gate applications postponed from resizing the state vector
                let mut s2 = *s;
                let mut ps2 = ps.clone();
                loop {
                    if is_full.load(Ordering::Relaxed) {
                        return (*b, s2, ps2);
                    }
                    match ps2.pop() {
                        None => {
                            break;
                        }
                        Some((idx, weight, gatenum)) => {
                            match apply_gates1(
                                gatenum,
                                &gates,
                                &table,
                                idx,
                                weight,
                                &is_full,
                                0,
                                config.maxload,
                            ) {
                                (_, SuccessorsResult::AllSucceeded) => {}
                                (_, SuccessorsResult::SomeFailed(fs)) => {
                                    ps2.extend(fs);
                                    return (*b, s2, ps2);
                                }
                            }
                        }
                    }
                }
                // (2) we apply remaining gate applications in the block
                for i in *s..block_stop(*b) {
                    if is_full.load(Ordering::Relaxed) {
                        s2 = i;
                        break;
                    }
                    let (idx, weight) = get(i);
                    match apply_gates1(0, &gates, &table, idx, weight, &is_full, 0, config.maxload)
                    {
                        (_, SuccessorsResult::AllSucceeded) => {}
                        (_, SuccessorsResult::SomeFailed(fs)) => {
                            s2 = i + 1;
                            ps2.extend(fs);
                            break;
                        }
                    }
                    if i + 1 == block_stop(*b) {
                        s2 = i + 1
                    }
                }
                (*b, s2, ps2)
            })
            // We keep any block that is not fully processed.
            .filter(|(b, s, ps)| s < &block_stop(*b) || !ps.is_empty())
            .collect();
        if !blocks.is_empty() {
            let mut table2 = table.increase_capacity_by_factor(1.5);
            std::mem::swap(&mut table, &mut table2);
        }
    }
    let num_nonzeros = table.num_nonzeros();
    let num_gate_apps = 0;
    ExpandResult {
        state: State::Sparse(table),
        num_nonzeros,
        num_gate_apps,
        method: ExpandMethod::Sparse,
    }
}

fn expand_push_dense<B: BasisIdx, AB: AtomicBasisIdx<B>>(
    gates: Vec<&Gate<B>>,
    num_qubits: usize,
    state: State<B, AB>,
) -> ExpandResult<B, AB> {
    let table = DenseStateTable::new(num_qubits);

    let num_gate_apps = match state {
        // FIXME: There should be better way to parallelize iteration over nonzeros of State::Sparse
        // FIXME: Refactor this iterator generation
        State::Sparse(prev_table) => prev_table
            .nonzeros()
            .into_par_iter()
            .map(|(bidx, weight)| apply_gates(&gates, &table, bidx, weight))
            .sum(),
        State::Dense(prev_table) => prev_table
            .array
            .into_par_iter()
            .enumerate()
            .map(|(idx, v)| {
                let weight = utility::unpack_complex(v.load(Ordering::Relaxed));
                apply_gates(&gates, &table, B::from_idx(idx), weight)
            })
            .sum(),
        _ => unreachable!(),
    };

    let num_nonzeros = table.num_nonzeros();

    ExpandResult {
        state: State::Dense(table),
        num_nonzeros,
        num_gate_apps,
        method: ExpandMethod::PushDense,
    }
}

fn expand_pull_dense<B: BasisIdx, AB: AtomicBasisIdx<B>>(
    gates: Vec<&Gate<B>>,
    num_qubits: usize,
    state: State<B, AB>,
) -> ExpandResult<B, AB> {
    let table = DenseStateTable::new(num_qubits);
    let capacity = 1 << num_qubits;

    let (num_gate_apps, num_nonzeros) = (0..capacity)
        .into_par_iter()
        .fold(
            || (0, 0),
            |acc, idx| {
                let bidx = B::from_idx(idx);
                let (weight, num_gate_apps_here) = apply_pull_gates(&gates, &state, bidx.clone());
                table.atomic_put(bidx, weight);
                (
                    acc.0 + num_gate_apps_here,
                    acc.1 + if utility::is_nonzero(weight) { 1 } else { 0 },
                )
            },
        )
        .reduce(|| (0, 0), |a, b| (a.0 + b.0, a.1 + b.1));

    ExpandResult {
        state: State::Dense(table),
        num_nonzeros,
        num_gate_apps,
        method: ExpandMethod::PullDense,
    }
}

fn apply_gates<B: BasisIdx>(
    gates: &[&Gate<B>],
    table: &DenseStateTable,
    bidx: B,
    weight: Complex,
) -> usize {
    if utility::is_zero(weight) {
        return 0;
    }
    if gates.is_empty() {
        table.atomic_put(bidx, weight);
        return 0;
    }

    match gates[0].push_apply(bidx, weight) {
        PushApplyOutput::Nonbranching(new_bidx, new_weight) => {
            1 + apply_gates(&gates[1..], table, new_bidx, new_weight)
        }
        PushApplyOutput::Branching((new_bidx1, new_weight1), (new_bidx2, new_weight2)) => {
            let num_gate_apps_1 = apply_gates(&gates[1..], table, new_bidx1, new_weight1);
            let num_gate_apps_2 = apply_gates(&gates[1..], table, new_bidx2, new_weight2);
            1 + num_gate_apps_1 + num_gate_apps_2
        }
    }
}

fn apply_pull_gates<B: BasisIdx, AB: AtomicBasisIdx<B>>(
    gates: &[&Gate<B>],
    prev_state: &State<B, AB>,
    bidx: B,
) -> (Complex, usize) {
    if gates.is_empty() {
        let weight = prev_state.get(&bidx).unwrap_or(Complex::new(0.0, 0.0));
        return (weight, 0);
    }

    match gates[0].pull_action.as_ref().unwrap()(bidx) {
        PullApplyOutput::Nonbranching(neighbor, multiplier) => {
            let (weight, num_gate_apps) = apply_pull_gates(&gates[1..], prev_state, neighbor);
            (weight * multiplier, 1 + num_gate_apps)
        }
        PullApplyOutput::Branching((neighbor1, multiplier1), (neighbor2, multiplier2)) => {
            let (weight1, num_gate_apps_1) = apply_pull_gates(&gates[1..], prev_state, neighbor1);
            let (weight2, num_gate_apps_2) = apply_pull_gates(&gates[1..], prev_state, neighbor2);

            (
                weight1 * multiplier1 + weight2 * multiplier2,
                1 + num_gate_apps_1 + num_gate_apps_2,
            )
        }
    }
}
