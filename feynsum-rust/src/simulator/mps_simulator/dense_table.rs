use crate::types::{BasisIdx, Complex};
use crate::simulator::mps_simulator::state::Table;
use crate::utility;

#[derive(Debug)]
pub struct DenseStateTable {
    pub array: Vec<Complex>,
}

impl DenseStateTable {
    pub fn new(num_qubits: usize) -> Self {
        let capacity = 1 << num_qubits;

        Self {
            array: vec![Complex::new(0.0, 0.0); capacity],
        }
    }

    pub fn num_nonzeros(&self) -> usize {
        self.array
            .iter()
            .filter(|c| utility::is_nonzero(**c))
            .count()
    }

    pub fn get<B: BasisIdx>(&self, bidx: &B) -> Option<&Complex> {
        self.array.get(bidx.as_idx())
    }
}

impl<B: BasisIdx> Table<B> for DenseStateTable {
    fn put(&mut self, bidx: B, weight: Complex) {
        let idx = bidx.as_idx();

        self.array[idx] += weight;
    }
}
