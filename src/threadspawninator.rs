use rand::{rngs::StdRng, Rng, SeedableRng};
use std::thread;
use std::time::Duration;
use std::{sync::mpsc, thread::JoinHandle};

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

    pub fn spawn_threads(&mut self, num: usize) {
        let mut v: Vec<Option<JoinHandle<()>>> = Vec::new();
        let (send_finished_thread, receive_finished_thread) = std::sync::mpsc::channel();
        let (tx, rx) = std::sync::mpsc::channel();
        for i in 0..self.num_of_threads {
            let send_finished_thread = send_finished_thread.clone();
            let tx = tx.clone();
            let join_handle = thread::spawn(move || {
                let mut r = rand::thread_rng();
                let mut sol: Solution = Solution::new(
                    0.002,
                    2000000.0,
                    0.95,
                    &Cases::new().l150,
                    5000,
                    r.gen(),
                    r.gen(),
                );
                // Signal that we are finished.
                // This will wake up the main thread.
                send_finished_thread
                    .send(sol.threshold_acceptance())
                    .unwrap();
                tx.send(i).unwrap();
            });
            v.push(Some(join_handle));
        }

        let counter = 0;
        let mut tuples = vec![(0.0f64, vec![0usize; num], 0u64, 0u64); 36];
        loop {
            // Check if all threads are finished
            let num_left = v.iter().filter(|th| th.is_some()).count();
            if num_left == 0 {
                break;
            }

            // Wait until a thread is finished, then join it
            let i = rx.recv().unwrap();
            tuples[i] = receive_finished_thread.recv().unwrap();
            let join_handle = std::mem::take(&mut v[i]).unwrap();
            println!("Joining {} ...", i);
            join_handle.join().unwrap();
            println!("{} joined.", i);
        }

        println!("All joined.");
        for t in tuples {
            println!("{:?}", t);
        }
        //println!("{:?}", tuples);


    }
}
