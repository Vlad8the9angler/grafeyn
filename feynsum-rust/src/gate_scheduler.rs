use std::collections::HashSet;
use std::fmt::Display;
use std::str::FromStr;

use log::info;

use crate::types::QubitIndex;

mod greedy_finish_qubit_gate_scheduler;
mod greedy_nonbranching_gate_scheduler;
mod naive_gate_scheduler;

pub use greedy_finish_qubit_gate_scheduler::GreedyFinishQubitGateScheduler;
pub use greedy_nonbranching_gate_scheduler::GreedyNonbranchingGateScheduler;
pub use naive_gate_scheduler::NaiveGateScheduler;

#[derive(Debug, Copy, Clone)]
pub enum GateSchedulingPolicy {
    Naive,
    GreedyNonbranching,
    GreedyFinishQubit,
}

impl FromStr for GateSchedulingPolicy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "naive" => Ok(GateSchedulingPolicy::Naive),
            "greedy-nonbranching" | "gnb" => Ok(GateSchedulingPolicy::GreedyNonbranching),
            "greedy-finish-qubit" | "gfq" => Ok(GateSchedulingPolicy::GreedyFinishQubit),
            _ => Err(format!(
                "unknown gate scheduling policy: {}; valid values are: naive, gnb",
                s
            )),
        }
    }
}

impl Display for GateSchedulingPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GateSchedulingPolicy::Naive => write!(f, "naive"),
            GateSchedulingPolicy::GreedyNonbranching => write!(f, "greedy-nonbranching"),
            GateSchedulingPolicy::GreedyFinishQubit => write!(f, "greedy-finish-qubit"),
        }
    }
}

pub trait GateScheduler {
    fn pick_next_gates(&mut self) -> Vec<usize>;
}

pub fn create_gate_scheduler<'a>(
    gate_scheduling_policy: &GateSchedulingPolicy,
    num_gates: usize,
    num_qubits: usize,
    gate_touches: Vec<&'a HashSet<QubitIndex>>,
    gate_is_branching: Vec<bool>,
) -> Box<dyn GateScheduler + 'a> {
    match gate_scheduling_policy {
        GateSchedulingPolicy::Naive => {
            info!("using naive gate scheduler");
            Box::new(NaiveGateScheduler::new(num_gates))
        }
        GateSchedulingPolicy::GreedyNonbranching => {
            info!("using greedy nonbranching gate scheduler");
            Box::new(GreedyNonbranchingGateScheduler::new(
                num_gates,
                num_qubits,
                gate_touches,
                gate_is_branching,
            ))
        }
        GateSchedulingPolicy::GreedyFinishQubit => {
            info!("using greedy nonbranching gate scheduler");
            Box::new(GreedyFinishQubitGateScheduler::new(
                num_gates,
                num_qubits,
                gate_touches,
            ))
        }
    }
}
