use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::sa::SimAnn;

#[allow(non_snake_case)]

pub struct Solution {
    cities: Vec<usize>,
    epsilon: f64,
    phi: f64,
    temp: f64,
    sa: SimAnn,
    L: u32,
    best_path: Vec<usize>,
    best_eval: f64,
    initial_sol_seed: u64,
    neighbor_seed: u64,
}

impl Solution {
    pub fn new(
        epsilon: f64,
        initial_temp: f64,
        phi: f64,
        cities: &Vec<usize>,
        L: u32,
        initial_sol_seed: u64,
        neighbor_seed: u64,
    ) -> Self {
        let len = cities.len();
        Self {
            cities: cities.clone(),
            epsilon,
            phi,
            temp: initial_temp,
            sa: SimAnn::new(len, &cities, neighbor_seed),
            L,
            best_path: cities.clone(),
            best_eval: f64::INFINITY,
            neighbor_seed,
            initial_sol_seed,
        }
    }

    pub fn get_best_eval(&self) -> f64 {
        self.best_eval
    }

    pub fn get_best_path(&self) -> Vec<usize> {
        self.best_path.clone()
    }

    pub fn get_neighbor_seed(&self) -> u64 {
        self.neighbor_seed
    }

    pub fn get_initial_sol_seed(&self) -> u64 {
        self.initial_sol_seed
    }

    fn calculate_batch(&mut self) -> f64 {
        let mut counter = 0;

        let mut r = 0.0;
        while counter < self.L {
            let last_cost = self.sa.get_cost() + self.temp;
            self.sa.get_neighbor(&mut self.cities);
            let new_cost = self.sa.get_cost();

            if new_cost < last_cost {
                println!("E:{:.20}", new_cost);
                counter += 1;
                r += new_cost;

                if new_cost <= self.best_eval && new_cost < 1.0 {
                    self.best_path = self.cities.clone();
                    self.best_eval = new_cost;
                }
            } else {
                self.sa.undo(&mut self.cities);
            }
        }
        r / f64::try_from(self.L).unwrap()
    }

    pub fn threshold_acceptance(&mut self) -> (f64, Vec<usize>, u64, u64) {
        self.sa.prepare();
        self.cities = self
            .sa
            .get_initial_solution(&mut self.cities, self.initial_sol_seed);
        self.sa.add_initial_distance();

        let mut batch_average: f64 = 0.0;
        while self.temp > self.epsilon {
            let mut q: f64 = f64::INFINITY;
            while batch_average < q {
                q = batch_average;
                batch_average = self.calculate_batch();
            }

            self.temp = self.phi * self.temp;
            println!("T:{}", self.temp);
        }

        println!("S:{:.20}", self.best_eval);
        println!("P:{:?}", self.best_path);
        println!("N:{}", self.neighbor_seed);
        println!("I:{}", self.initial_sol_seed);
        (
            self.best_eval,
            self.best_path.clone(),
            self.neighbor_seed,
            self.initial_sol_seed,
        )
    }

    pub fn sweep(&mut self) {
        let norm = self.sa.get_normalizer();
        for i in 0..self.best_path.len() {
            let value = self.best_path[i];
            for j in 1..self.best_path.len(){
                self.best_path[i] = self.best_path[i];
                
                
            }
        }
    }

    
}
