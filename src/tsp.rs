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
    initial_sol_seed : u64,
    neighbor_seed : u64
}

impl Solution {
    pub fn new(epsilon: f64, initial_temp: f64, phi: f64, cities: &Vec<usize>, L: u32, initial_sol_seed : u64, neighbor_seed : u64 ) -> Self {
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
            initial_sol_seed
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
                //println!("E:{:.20}", new_cost);
                counter += 1;
                r += new_cost;

                if new_cost <= self.best_eval {
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
        self.cities = self.sa.get_initial_solution(&mut self.cities, self.initial_sol_seed);
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
        (self.best_eval, self.best_path.clone(), self.neighbor_seed, self.initial_sol_seed)
    }

    pub fn hill_descent(&mut self, seed: u64) {
        self.sa.prepare();
        let mut r = StdRng::seed_from_u64(seed);
        let mut batch_average: f64 = 0.0;
        for i in 0..self.cities.len() as usize {
            let mut n: usize = r.gen();
            let mut q: f64 = f64::INFINITY;
            while n == i {
                n = r.gen();
            }

            let previous_cost = self.sa.get_cost();
            let value = self.cities[i];
            let index = n % usize::try_from(self.cities.len()).unwrap();
            self.cities[i] = self.cities[index as usize];
            self.cities[index as usize] = value;

            let new_cost = self.sa.get_cost();
            if previous_cost < new_cost {
                self.sa.undo(&mut self.cities);
            } else {
                // while batch_average < q {
                //     q = batch_average;
                //     batch_average = self.calculate_batch();
                // }
            }
        }

        // --
        println!("{}", self.best_eval);
        println!("{:?}", self.best_path);
    }
}
