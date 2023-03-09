use std::thread;

use crate::tsp::Solution;

struct TSI{
    num_of_threads : usize,
    best_solution_overall : Vec<f64>,
    best_eval_overall : f64,
    solutions : Vec<Solution>,
}


impl TSI{
    pub fn spawn_threads(&self){
        for i in 1..self.num_of_threads {
            let handle = thread::spawn(|| {
                
            }).join().unwrap();
            
        }
        
    }
}
