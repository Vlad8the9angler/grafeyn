use std::collections::HashSet;

use log::debug;

use super::GateScheduler;
use crate::types::{GateIndex, QubitIndex};

pub struct GreedyNonbranchingGateScheduler<'a> {
    frontier: Vec<GateIndex>,
    num_gates: usize,
    num_qubits: usize,
    gate_touches: Vec<&'a HashSet<QubitIndex>>,
    gate_is_branching: Vec<bool>,
    max_branching_stride: usize,
}

impl<'a> GateScheduler for GreedyNonbranchingGateScheduler<'a> {
    fn pick_next_gates(&mut self) -> Vec<GateIndex> {
        let mut num_branching_so_far = 0;
        let mut next_gates = Vec::<GateIndex>::new();

        while num_branching_so_far < self.max_branching_stride {
            next_gates.append(&mut self.visit_maximal_nonbranching_run());

            if let Some(next_gate) = self.visit_branching() {
                num_branching_so_far += 1;
                next_gates.push(next_gate);
            } else {
                break;
            }
        }

        // each gate in next_gates should be marked as already visited
        assert!(next_gates.iter().all(|gi| !self.okay_to_visit(*gi)));

        debug!("next gates: {:?}", next_gates);

        next_gates
    }
}

impl<'a> GreedyNonbranchingGateScheduler<'a> {
    pub fn new(
        num_gates: usize,
        num_qubits: usize,
        gate_touches: Vec<&'a HashSet<QubitIndex>>,
        gate_is_branching: Vec<bool>,
    ) -> Self {
        debug!(
            "initializing greedy nonbranching gate scheduler with {} gates and {} qubits",
            num_gates, num_qubits
        );
        let scheduler = Self {
            frontier: (0..num_qubits)
                .map(|qi| next_touch(num_gates, &gate_touches, qi, 0))
                .collect(),
            num_gates,
            num_qubits,
            gate_touches,
            gate_is_branching,
            max_branching_stride: 2,
        };

        assert_eq!(scheduler.frontier.len(), num_qubits);
        assert_eq!(scheduler.gate_touches.len(), num_gates);
        assert_eq!(scheduler.gate_is_branching.len(), num_gates);

        debug!("initial frontier: {:?}", scheduler.frontier);

        scheduler
    }

    fn visit_maximal_nonbranching_run(&mut self) -> Vec<GateIndex> {
        let mut non_branching_gates = Vec::new();

        loop {
            let mut selection = Vec::<GateIndex>::new();

            for qi in 0..self.num_qubits {
                loop {
                    let next_gi = self.frontier[qi];
                    if next_gi >= self.num_gates
                        || self.gate_is_branching[next_gi]
                        || !self.okay_to_visit(next_gi)
                    {
                        break;
                    } else {
                        assert!(self.okay_to_visit(next_gi));
                        self.visit(next_gi);
                        selection.push(next_gi);
                    }
                }
            }

            if selection.is_empty() {
                break;
            } else {
                non_branching_gates.append(&mut selection);
            }
        }
        non_branching_gates
    }

    fn visit_branching(&mut self) -> Option<GateIndex> {
        let result = self
            .frontier
            .iter()
            .filter(|gi| {
                *gi < &self.num_gates && self.gate_is_branching[**gi] && self.okay_to_visit(**gi)
            })
            .nth(0)
            .copied();

        if let Some(gi) = result {
            self.visit(gi);
        }

        result
    }

    fn visit(&mut self, gi: GateIndex) {
        debug!("visiting gate: {}", gi);
        assert!(self.okay_to_visit(gi));
        for qi in self.gate_touches[gi] {
            let next = next_touch(self.num_gates, &self.gate_touches, *qi, gi + 1);

            self.frontier[*qi] = next;
            debug!("updated frontier[{}] to {}", qi, self.frontier[*qi]);
        }
    }

    fn okay_to_visit(&self, gi: GateIndex) -> bool {
        gi < self.num_gates
            && self.gate_touches[gi]
                .iter()
                .all(|qi| self.frontier[*qi] == gi)
    }
}

fn next_touch(
    num_gates: usize,
    gate_touches: &[&HashSet<QubitIndex>],
    qi: QubitIndex,
    gi: GateIndex,
) -> GateIndex {
    if gi >= num_gates {
        num_gates
    } else if gate_touches[gi].contains(&qi) {
        gi
    } else {
        next_touch(num_gates, gate_touches, qi, gi + 1)
    }
}
