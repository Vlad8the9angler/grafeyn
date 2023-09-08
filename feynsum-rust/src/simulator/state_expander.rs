use std::cmp;

use crate::circuit::{Gate, MaybeBranchingOutput, PushApplicable};
use crate::config::Config;
use crate::types::{BasisIdx, Complex};
use crate::utility::is_zero;

use super::state::State;
use super::table::SparseStateTable;
use super::SimulatorError;

pub struct ExpandResult {
    pub state: State,
    pub num_nonzero: usize,
    pub num_gate_apps: usize,
}

pub fn expand(
    gates: Vec<&Gate>,
    config: &Config,
    num_qubits: usize,
    prev_num_nonzero: usize,
    state: State,
) -> Result<ExpandResult, SimulatorError> {
    let max_num_states = 1 << num_qubits;

    let num_nonzero = state.num_nonzero();

    let rate = f64::max(1.0, num_nonzero as f64 / prev_num_nonzero as f64);
    let expected = cmp::min(max_num_states, (rate * num_nonzero as f64) as i64);
    let expected_density = expected as f64 / max_num_states as f64;
    let current_density = num_nonzero as f64 / max_num_states as f64;
    let expected_cost = expected_density.max(current_density);

    if expected_cost >= config.dense_threshold {
        expand_dense(gates, num_qubits, state)
    } else {
        expand_sparse(gates, state)
    }
}

fn expand_sparse(gates: Vec<&Gate>, state: State) -> Result<ExpandResult, SimulatorError> {
    let prev_entries = state.compactify();

    let mut table = SparseStateTable::new();

    let mut num_gate_apps = 0;

    for (bidx, weight) in prev_entries {
        num_gate_apps += apply_gates(&gates, &mut table, bidx, weight)?;
    }

    let num_nonzero = table.num_nonzero();

    Ok(ExpandResult {
        state: State::Sparse(table), // TODO: use DenseStateTable if necessary
        num_nonzero,
        num_gate_apps,
    })
}

fn apply_gates(
    gates: &[&Gate],
    table: &mut SparseStateTable,
    bidx: BasisIdx,
    weight: Complex,
) -> Result<usize, SimulatorError> {
    if is_zero(weight) {
        return Ok(0);
    }
    if gates.is_empty() {
        table.put(bidx, weight);
        return Ok(0);
    }

    match gates[0].push_apply(bidx, weight)? {
        MaybeBranchingOutput::OuptutOne((new_bidx, new_weight)) => {
            Ok(1 + apply_gates(&gates[1..], table, new_bidx, new_weight)?)
        }
        MaybeBranchingOutput::OutputTwo((new_bidx1, new_weight1), (new_bidx2, new_weight2)) => {
            let num_gate_apps_1 = apply_gates(&gates[1..], table, new_bidx1, new_weight1)?;
            let num_gate_apps_2 = apply_gates(&gates[1..], table, new_bidx2, new_weight2)?;
            Ok(1 + num_gate_apps_1 + num_gate_apps_2)
        }
    }
}

fn expand_dense(gates: Vec<&Gate>, state: State) -> Result<ExpandResult, SimulatorError> {
    unimplemented!()
}
