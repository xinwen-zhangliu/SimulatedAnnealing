use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::sa::SimAnn;

trait TA {
    fn calcula_lote() -> (f64, f64);
    fn threshold_acceptance();
}

pub struct Solution {
    cities: Vec<usize>,
    epsilon: f64,
    phi: f64,
    temp: f64,
    sa: SimAnn,
    L: u32,
    best_path: Vec<usize>,
    best_eval: f64,
}

impl Solution {
    pub fn new(epsilon: f64, initial_temp: f64, phi: f64, cities: &Vec<usize>, L: u32) -> Self {
        let len = cities.len();
        Self {
            cities: cities.clone(),
            epsilon,
            phi,
            temp: initial_temp,
            sa: SimAnn::new(len, &cities),
            L,
            best_path: cities.clone(),
            best_eval: 0.0,
        }
    }

    fn calculate_batch(&mut self) -> f64 {
        let mut counter = 0;

        let mut r = 0.0;
        while counter < self.L {
            let mut s2 = self.cities.clone();
            self.sa.get_neighbor(&mut s2[..]);
            let new_cost = self.sa.get_cost(&mut s2);
            let last_cost = self.sa.get_cost(&mut self.cities) + self.temp;

            if new_cost < last_cost {
                println!("E:{}", self.sa.get_cost(&mut s2));
                self.cities = s2.clone();
                counter += 1;
                r += self.sa.get_cost(&mut s2);

                if new_cost <= (self.sa.get_cost(&mut self.best_path)) {
                    self.best_path = s2.clone();
                    self.best_eval = self.sa.get_cost(&mut self.best_path);
                }
            }
        }
        r / f64::try_from(2000).unwrap()
    }

    pub fn threshold_acceptance(&mut self) {
        self.sa.prepare();
        self.cities = self.sa.get_initial_solution(&mut self.cities, 8);

        let mut batch_average: f64 = 0.0;
        while self.temp > self.epsilon {
            let mut q: f64 = f64::INFINITY;
            while batch_average < q {
                q = batch_average;
                batch_average = self.calculate_batch();
            }

            self.temp = self.phi * self.temp;
            let epsilon = q - batch_average;
            if epsilon.abs() < 50.0 && batch_average > 1000.0 {
                self.temp *= 10000.0;
            }
            println!("T:{}", self.temp);
        }

        // --
        println!("{}", self.best_eval);
        let best_sol = self
            .best_path
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",");
        println!("[{}]", best_sol);
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

            let previous_cost = self.sa.get_cost(&mut self.cities);

            let value = self.cities[i];
            let index = n % usize::try_from(self.cities.len()).unwrap();
            self.cities[i] = self.cities[index as usize];
            self.cities[index as usize] = value;

            let new_cost = self.sa.get_cost(&mut self.cities);
            if previous_cost < new_cost {
                dbg!("undoing");
                self.sa.undo(&mut self.cities);
            } else {
                dbg!("calculating batch", self.cities.to_vec(), new_cost);
                // while batch_average < q {
                //     q = batch_average;
                //     batch_average = self.calculate_batch();
                // }
            }
        }

         // --
        println!("{}", self.best_eval);
        let best_sol = self
            .best_path
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",");
        println!("[{}]", best_sol);
    }
}
