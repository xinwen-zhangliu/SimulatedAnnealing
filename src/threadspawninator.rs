use rand::{rngs::StdRng, Rng, SeedableRng};
use std::sync::mpsc;
use std::thread;

use crate::{testCases::Cases, tsp::Solution};

pub struct TSI {
    num_of_threads: usize,
    //best_path_overall: Vec<usize>,
    best_eval_overall: f64,
    best_solution: Option<Solution>,

}

type Type = Solution;

impl TSI {
    pub fn new(num: usize) -> Self {
        Self {
            num_of_threads: num_cpus::get(),
            //best_path_overall: vec![0.0f64; num],
            best_eval_overall: f64::NEG_INFINITY,
            
            best_solution: None,
        }
    }

    pub fn spawn_threads(&mut self) {
        let (tx, rx) = mpsc::channel();
        let (sol_tx, sol_rx) = mpsc::channel();
        for _i in 1..self.num_of_threads {
            let tx = tx.clone();
            let sol_tx = sol_tx.clone();
            let handle = thread::spawn(move || {
                dbg!("thread spawned");
                for _i in 1..3 {}
                let mut r = rand::thread_rng();
                let mut sol: Type = Solution::new(
                    0.002,
                    800000.0,
                    0.95,
                    &Cases::new().l40,
                    2000,
                    r.gen(),
                    r.gen(),
                );
                
                tx.send(sol.threshold_acceptance()).unwrap_or_else(|error| {
                    panic!("Error sending solution through channel");
                });
                sol_tx.send(sol).unwrap();
            });
            //.join()
            //.unwrap();
        }
        let mut counter = 0;
        for tuple in rx{
            println!("{:?}", tuple);
            counter+=1;
            
        }
        dbg!(counter);
        for received in sol_rx {
//            if received.is_some(){
                if received.get_best_eval() < self.best_eval_overall {
                self.best_eval_overall = received.get_best_eval();
                self.best_solution = Some(received);
                // self.best_solution.unwrap_or_else(|| {
                //     if self.best_solution.kind() != Some(Solution) {
                        
                //     }
                // });
            }
//            }
            
        }

        let neighbor_seed = self.best_solution.as_ref().unwrap().get_neighbor_seed();
        let initial_sol_seed = self.best_solution.as_ref().unwrap().get_initial_sol_seed();
        println!("best_eval_overall  {:.15}", self.best_eval_overall);
        println!("path:{:?} ", self.best_solution.as_ref().unwrap().get_best_path() );
        println!("neighbor seed : {}, initial_sol_seed : {}", neighbor_seed, initial_sol_seed)
    }
}
