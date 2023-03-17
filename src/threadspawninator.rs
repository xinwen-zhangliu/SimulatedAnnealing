use itertools::Itertools;
use rand::Rng;
use std::thread::JoinHandle;
use std::{sync::mpsc, thread};
use crate::sa::SimAnn;

pub struct TSI {
    num_of_threads: usize,
}

impl TSI {
    pub fn new() -> Self {
        Self {
            num_of_threads: num_cpus::get(),
        }
    }

    pub fn spawn_threads(
        &mut self,
        num: usize,
        epsilon: f64,
        phi: f64,
        batch_size: u32,
        neighbor_seed: u64,
        initial_sol_seed: u64,
        cities: Vec<usize>,
    ) {
        let num_iter = num / self.num_of_threads;
        if initial_sol_seed != 0 &&  neighbor_seed != 0 {
            let mut sol: SimAnn = SimAnn::new(
                epsilon,
                phi,
                &cities,
                batch_size,
                initial_sol_seed,
                neighbor_seed,
            );
            sol.threshold_acceptance();
        } else {
            let mut tuples = self.spawner(num_iter, epsilon, phi, batch_size, cities);
            tuples.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
            for t in tuples {
                println!("{:?}", t);
            }
        }
    }

    fn spawner(
        &mut self,
        num: usize,
        epsilon: f64,
        phi: f64,
        batch_size: u32,
        cities: Vec<usize>,
    ) -> Vec<(f64, Vec<usize>, u64, u64)> {
        let mut v: Vec<Option<JoinHandle<()>>> = Vec::new();
        let (send_finished_thread, receive_finished_thread) = mpsc::channel();
        let (tx, rx) = mpsc::channel();
        let (ctx, crx) = mpsc::channel();

        for i in 0..self.num_of_threads {
            let send_finished_thread = send_finished_thread.clone();
            let tx = tx.clone();
            let ctx = ctx.clone();
            let cities = cities.clone();
            let join_handle = thread::spawn(move || {
                let mut r = rand::thread_rng();
                let mut counter = 1;
                let mut feasible_solutions = 0.0;
                let mut best_tuple = (f64::INFINITY, vec![0usize; 40], 0, 0);
                loop {
                    counter += 1;
                    let mut sol: SimAnn = SimAnn::new(
                        epsilon,
                        phi,
                        &cities,
                        batch_size,
                        r.gen(),
                        r.gen(),
                    );

                    let new_tuple = sol.threshold_acceptance();
                    if new_tuple.0 < 1.0 {
                        feasible_solutions += 1.0;
                    }
                    if new_tuple.0 < best_tuple.0 {
                        best_tuple = new_tuple;
                    }
                    if counter > num {
                        break;
                    }
                }

                send_finished_thread.send(best_tuple).unwrap();
                tx.send(i).unwrap();
                ctx.send(feasible_solutions).unwrap();
            });
            v.push(Some(join_handle));
        }

        let mut counter = 0.0;
        let mut tuples = vec![(0.0f64, vec![0usize; num], 0u64, 0u64); self.num_of_threads];
        loop {
            let num_left = v.iter().filter(|th| th.is_some()).count();
            if num_left == 0 {
                break;
            }

            let i = rx.recv().unwrap();
            tuples[i] = receive_finished_thread.recv().unwrap();
            counter += crx.recv().unwrap();
            let join_handle = std::mem::take(&mut v[i]).unwrap();
            join_handle.join().unwrap();
        }

        println!("AP:{}", counter / (num as f64 * 12.0));
        tuples
    }
}
