use crate::path::Path;

#[allow(non_snake_case)]

pub struct SimAnn {
    cities: Vec<usize>,
    epsilon: f64,
    epsilon_p: f64,
    phi: f64,
    temp: f64,
    path: Path,
    L: u32,
    best_path: Vec<usize>,
    best_eval: f64,
    initial_sol_seed: u64,
    neighbor_seed: u64,
}

impl SimAnn {
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
            path: Path::new(len, &cities, neighbor_seed),
            L,
            best_path: cities.clone(),
            best_eval: f64::INFINITY,
            neighbor_seed,
            initial_sol_seed,
            epsilon_p: 0.001,
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
            let last_cost = self.path.get_cost() + self.temp;
            self.path.get_neighbor(&mut self.cities);
            let new_cost = self.path.get_cost();

            if new_cost < last_cost {
                //println!("E:{:.20}", new_cost);
                counter += 1;
                r += new_cost;

                if new_cost <= self.best_eval && new_cost < 1.0 {
                    self.best_path = self.cities.clone();
                    self.best_eval = new_cost;
                }
            } else {
                self.path.undo(&mut self.cities);
            }
        }
        r / f64::try_from(self.L).unwrap()
    }

    ///
    /// Runs the simulated annealing algorithm with threshold acceptance over the subset
    /// of cities passed when initializing instance
    ///
    /// #Return types
    /// Returns a tuple with the first element being the evaluation of the best solution found,
    /// then the path oc cities, the seed used for finding neighbors and the seed used for the
    /// initial solution.
    pub fn threshold_acceptance(&mut self) -> (f64, Vec<usize>, u64, u64) {
        
        
        self.path.prepare();
        self.cities = self
            .path
            .get_initial_solution(&mut self.cities, self.initial_sol_seed);
        //prepare initial temp
        self.temp = self.initial_temp( &mut self.cities.clone(),8.0, 0.92 );
        println!("IT:{}", self.temp);
        //
        
        self.path.add_initial_distance();

        let mut batch_average: f64 = 0.0;
        while self.temp > self.epsilon {
            let mut q: f64 = f64::INFINITY;
            while batch_average < q {
                q = batch_average;
                batch_average = self.calculate_batch();
            }

            self.temp = self.phi * self.temp;
            //println!("T:{}", self.temp);
        }
  
        let mut cities = self.best_path.clone();
        let mut sweep = self.sweep(&mut cities);
        let mut best_sweep = (self.best_eval, cities); 
        loop {
            let mut new_sweep = self.sweep(&mut sweep.1.clone());

            if &new_sweep.0 < &best_sweep.0 {
                best_sweep = new_sweep.clone();
            }
            //new_sweep = self.sweep(&mut sweep.1.clone());
            if &new_sweep.0 == &sweep.0 {
                break;
            }
            sweep = new_sweep;
        }
        self.best_eval = sweep.0;
        self.best_path = sweep.1;

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

    pub fn sweep(&mut self, cities: &mut [usize]) -> (f64, Vec<usize>) {
        let mut path: Path = Path::new(cities.len(), &cities.to_vec(), 123);
        path.prepare();
        let norm = path.get_normalizer();
       
        let mut best_cost: f64 = path.add_dist(cities) / norm;

        let mut best_path = cities.to_vec();

        let swap = |x: &mut [usize], i: usize, j: usize| {
            let value = x[i];
            x[i] = x[j];
            x[j] = value;
        };

        for i in 0..cities.len() - 1 {
            for j in (i + 1)..cities.len() {
                
                swap(cities, i, j);
                
                let new_cost: f64 = path.add_dist(cities) / norm;

                if new_cost < best_cost {
                    best_cost = new_cost;
                    best_path = cities.to_vec().clone();
                }

                swap(cities, i, j);
            }
        }

        (best_cost, best_path)
    }

    fn initial_temp(&mut self, s: &mut [usize], mut t: f64, p: f64) -> f64 {
        let mut percentage = self.accepted_percentage(s, t);
        let t1: f64;
        let t2: f64;

        if (p - percentage).abs() <= self.epsilon_p {
            return t;
        }

        if percentage < p {
            while percentage < p {
                t *= 2.0;
                percentage = self.accepted_percentage(s, t)
            }
            t1 = t / 2.0;
            t2 = t;
        } else {
            while percentage > p {
                t *= 0.5;
                percentage = self.accepted_percentage(s, t);
            }
            t1 = t;
            t2 = t * 2.0;
        }

        self.binary_search(s, t1, t2, p)
    }

    fn accepted_percentage(&mut self, s: &mut [usize], t: f64) -> f64 {
        let n = 1000;
        let mut counter = 0;
        let mut new_s = s.to_vec();
        let mut previous_cost: f64;
        for _i in 0..n {
            previous_cost = self.path.get_cost();
            self.path.get_neighbor(s);

            if self.path.get_cost() <= previous_cost + t {
                counter += 1;
            } else {
                self.path.undo(s);
            }
        }
        f64::try_from(counter).unwrap() / f64::try_from(n).unwrap()
    }

    fn binary_search(&mut self, s: &mut [usize], t1: f64, t2: f64, p: f64) -> f64 {
        let tm: f64 = (t1 + t2) / 2.0;
        if (t2 - t1) < self.epsilon_p {
            return tm;
        }

        let percentage = self.accepted_percentage(s, tm);

        if (p - percentage).abs() < self.epsilon_p {
            return tm;
        }

        if percentage > p {
            return self.binary_search(s, t1, tm, p);
        } else {
            return self.binary_search(s, tm, t2, p);
        }
    }
}
