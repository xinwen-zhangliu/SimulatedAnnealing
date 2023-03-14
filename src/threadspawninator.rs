use rand::Rng;
use std::thread;

use std::thread::JoinHandle;

use crate::{sa::SimAnn, testCases::Cases};

pub struct TSI {
    num_of_threads: usize,
    //best_path_overall: Vec<usize>,
    best_eval_overall: f64,
    best_solution: Option<SimAnn>,
}

type Type = SimAnn;

impl TSI {
    pub fn new() -> Self {
        Self {
            //num_of_threads: num_cpus::get(),
            //best_path_overall: vec![0.0f64; num],
            num_of_threads: 24,
            best_eval_overall: f64::NEG_INFINITY,

            best_solution: None,
        }
    }

    pub fn spawn_threads(&mut self, num: usize) {
        let mut tuples = self.spawner(5);
        // let mut new_tuples = tuples.clone();
        // //new_tuples = tuples.into_iter().filter(|x| {x.0 < 0.264}).collect();
        // tuples.iter().for_each(|t| {
        //     if t.0 < 0.264 {
        //         return;
        //         println!("{:?}", t);
        //     } else {
        //         tuples = self.spawner(40);
        //     };
        // });

        for t in tuples{
            println!("{:?}", t);

        }
    }

    fn spawner(&mut self, num: usize) -> Vec<(f64, Vec<usize>, u64, u64)> {
        let mut v: Vec<Option<JoinHandle<()>>> = Vec::new();
        let (send_finished_thread, receive_finished_thread) = std::sync::mpsc::channel();
        let (tx, rx) = std::sync::mpsc::channel();
        let average = num / self.num_of_threads;
        for i in 0..self.num_of_threads {
            let send_finished_thread = send_finished_thread.clone();
            let tx = tx.clone();

            let join_handle = thread::spawn(move || {
                let mut r = rand::thread_rng();
                let mut sol: SimAnn = SimAnn::new(
                    0.002,
                    800000.0,
                    0.95,
                    &Cases::new().l40,
                    2000,
                    r.gen(),
                    r.gen(),
                );

                let mut counter = 0;
                //let wanted_solution = 0.264;
                let mut tuple = sol.threshold_acceptance();
                let mut new_tuple = (f64::INFINITY, vec![0usize; tuple.1.len()], 0, 0);

                while counter < average {
                    counter += 1;
                    let mut sol: SimAnn = SimAnn::new(
                        0.002,
                        800000.0,
                        0.95,
                        &Cases::new().l40,
                        2000,
                        r.gen(),
                        r.gen(),
                    );

                    new_tuple = sol.threshold_acceptance();
                    if new_tuple.0 < tuple.0 {
                        tuple = new_tuple;
                    }
                }
                // while tuple.0 > wanted_solution {
                //     let mut sol: SimAnn = SimAnn::new(
                //         0.002,
                //         800000.0,
                //         0.95,
                //         &Cases::new().l40,
                //         2000,
                //         r.gen(),
                //         r.gen(),
                //     );
                //     let new_tuple = sol.threshold_acceptance();
                //     if new_tuple.0 < tuple.0 {
                //         tuple = new_tuple;
                //     }
                // }

                send_finished_thread.send(tuple).unwrap();
                tx.send(i).unwrap();
            });
            v.push(Some(join_handle));
        }

        let mut tuples = vec![(0.0f64, vec![0usize; num], 0u64, 0u64); 24];
        loop {
            // Check if all threads are finished otherwise they will block each other
            //until the previous one finishes
            let num_left = v.iter().filter(|th| th.is_some()).count();
            if num_left == 0 {
                break;
            }

            // Wait until a thread is finished, then join it
            let i = rx.recv().unwrap();
            tuples[i] = receive_finished_thread.recv().unwrap();
            let join_handle = std::mem::take(&mut v[i]).unwrap();
            //println!("Joining {} ...", i);
            join_handle.join().unwrap();
            //println!("{} joined.", i);
        }
        tuples
    }
}
