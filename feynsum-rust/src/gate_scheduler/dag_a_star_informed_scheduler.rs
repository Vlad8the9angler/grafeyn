//IDEA: represent a circuit as a directed acylic graph(DAG)
//use a*-search with  a heuristic function to find the optimal scheduling
//WARNING: scheduler may be slower than the greedy version 
//IMPORTANT NOTE:  I had some issues testing the scheduler and adapting it to the current scheme, 
// the role of it is mainly a visualisation of the idea of a forward looking scheduler
use super::{utility, GateScheduler};
use crate::circuit::{self, Circuit};
use crate::types::{BasisIdx, GateIndex, QubitIndex};
use std::collections::{BinaryHeap, HashMap, HashSet};


pub struct DAGScheduler{
    frontier: Vec<GateIndex>,
    num_gates: usize,
    num_qubits: usize,
    gate_touches: Vec<&'a [QubitIndex]>,
    gate_is_branching: Vec<bool>,
    max_branching_stride: usize,
    informed: bool,
    dependency_graph: HashMap<usize, Vec<usize>>
}

impl <'a> GateScheduler for DAGScheduler <'a>{
    fn pick_next_gates(&mut self) -> Vec<usize> {
        let mut result:Vec<usize>;
        if self.informed{
            result = self.a_star_next();
        }else{
            result = self.greedy_next();            
        }
        result
    }   
}

impl <'a> DAGScheduler <'a>{
    pub fn new(num_gates: usize,
        num_qubits: usize,
        gate_touches: Vec<&'a [QubitIndex]>,
        gate_is_branching: Vec<bool>,
        informed: bool,)->Self{
        let sched = Self{
            frontier: (0..num_qubits)
                .map(|qi| next_touch(num_gates, &gate_touches, qi, 0))
                .collect(),
            dependency_graph:HashMap::new(),
            num_gates: num_gates,
            num_qubits: num_qubits,
            gate_touches: gate_touches,
            gate_is_branching: gate_is_branching,
            max_branching_stride:2,
            informed: informed,
        };
        sched.build_dependency_graph();
        sched
    }


    fn build_dependency_graph(&mut self){
        self.dependency_graph = HashMap::new();
        for g in 0..self.num_gates{
            let preds = self.predecesors(g);
            self.dependency_graph.insert(g, preds);
        
        }
    }
    fn greedy_heuristic(&mut self, g_index:usize)-> i32{
        if self.gate_is_branching[g_index] == true {
            0
        } else{
            1
        }
    }
    //a simple a* heuristic
    fn a_star_heuristic(&mut self,  g_index:usize)-> i32{
        let mut branch:i32 = 0;
        if self.gate_is_branching[g_index] == true{
            branch = 1;
        }
        let depth = self.predecessors(g_index).len();
        let oportunities = self.successor(g_index).len();
        let heuristic:i32 = branch - oportunities as i32 + depth as i32;
        
        heuristic
    }
    //implements a* scheduler
    fn a_star_next(&mut self)-> Vec<usize>{
        let mut a_star:Vec<usize> = Vec::new();
        let mut bf: i8 = 0;
        while  bf < 2{
            let current: Vec<usize> = self.current_gates();
            if current.len() == 0{
                return a_star;
            }
            let mut best_gate = current[0];
            let mut min_heur: i32 = self.a_star_heuristic(best_gate);
            for j in 1..current.len(){
                let cur_heur = self.a_star_heuristic(current[j]);
                if min_heur > cur_heur {
                    best_gate = current[j];
                    min_heur = cur_heur;
                }
            } 
            a_star.push(best_gate);
            //note:branching gates may be added before non_branching ones 
            //if they significantly increase the number of succesors
            if self.gate_is_branching[best_gate]{
                bf+=1;
            }
        }
        a_star
    }

    //implements greedy scheduler
    fn greedy_next(&mut self)-> Vec<usize>{
        let mut greedy:Vec<usize> = Vec::new();
        
        let mut bf: i8 = 0;
        while bf < 2{
            let current = self.current_gates();
            //if no more gates may be added return  the kernel
            if current.len() == 0{
                return greedy;
            }
            let mut next: usize;
            let mut non_bracnhing:bool = false;
            for i in 0..current.len(){
                if self.gate_is_branching[current[i]] == false{
                    next = current[i];
                    non_bracnhing = true;
                    break; 
                }
            }
            //if all the gates are branching
            if !non_bracnhing{
                next =  current[0];
                bf+=1;
            } 
            greedy.push(next);
        }
        greedy
    }
    
    //function determining whether a gate touches a qubit
    fn touches_qubit(&mut self, gate_index:GateIndex,  qubit_index: QubitIndex )-> bool{
        self.gate_touches[gate_index].contains(&qubit_index)
    }
    fn predecesors_init(&mut self, gate_index:GateIndex) -> Vec<QubitIndex>{
        //find all the elements required by the gate
        let mut required_qubits: Vec<QubitIndex> = Vec::new();
        for q in 0..self.num_qubits{
            if self.touches_qubit(gate_index, q){
                required_qubits.push(q);
            }
        }
        let predecessors: Vec<GateIndex> =  Vec::new();

        predecessors
    }
    //gets the currently ready gates, i.e the gates that are ready to be executed
    fn current_gates(&mut self)-> Vec<GateIndex>{
        self.frontier.clone()
    }
    //gets the successor gates of the given gates
    fn successor(&mut self,  gate_index:GateIndex) -> Vec<QubitIndex>{
        let mut successors = Vec::new();
        for (&gate, dependencies) in &self.dependency_graph {
            if dependencies.contains(&gate_index) {
                successors.push(gate);
            }
        }
        successors
    }
    //predecessors of a gate
    fn predecessors(&mut self, gate_index: GateIndex) -> Vec<GateIndex> {
        let mut predecessors = HashSet::new(); 
    
        for &qubit in self.gate_touches[gate_index] {
        
            for prev_gate in (0..gate_index).rev() {
                if self.touches_qubit(prev_gate, qubit) {
                    predecessors.insert(prev_gate);
                    break;
                }
            }
        }
        predecessors.into_iter().collect()
    }
    
}