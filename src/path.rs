use crate::reader::Reader;
use crate::City;
use libm::{atan2, cos, pow, sin, sqrt};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::convert::TryFrom;
use std::env;
use std::f64::consts::PI;

#[allow(non_snake_case)]

pub struct Path {
    initial_solution: Vec<usize>,
    num_of_cities: usize,
    n1: usize,
    n2: usize,
    sum_of_distances: f64,
    normalizer: f64,
    all_cities: Vec<City>,
    all_connections: Vec<Vec<f64>>,
    max_distance: f64,
    r: StdRng,
}

impl Path {
    pub fn new(num: usize, cities: &Vec<usize>, seed: u64) -> Self {
        // let db_path = std::env::var_os("CARGO_MANIFEST_DIR").unwrap();
        // let path_str = db_path.into_string().unwrap();
        // let reader : Reader = Reader::new(&path_str);
        Self {
            initial_solution: cities.clone(),
            num_of_cities: num,
            n1: 0,
            n2: 0,
            sum_of_distances: 0.0,
            normalizer: 0.0,
            all_cities: vec![
                City {
                    id: 0,
                    lat: 0.0,
                    long: 0.0,
                };
                num as usize
            ],
            all_connections: vec![vec![0.0f64; 1092]; 1092],
            max_distance: 0.0,
            r: StdRng::seed_from_u64(seed),
        }
    }

    pub fn prepare(&mut self) {
        let db_path = env::var("CARGO_MANIFEST_DIR").unwrap();
        let path_str = db_path + "/db/citiesDB.db";

        let reader: Reader = Reader::new(&path_str);

        self.get_cities_connections(&reader);
        self.normalizer(&reader);
        self.fill_distances();
    }

    pub fn set_n1_n2(&mut self, n1 : usize, n2 : usize){
        self.n1 = n1;
        self.n2 = n2;
    }
    
    pub fn get_cost(&self) -> f64 {
        self.sum_of_distances / self.normalizer
    }

    pub fn get_max_distance(&self) -> f64 {
        self.max_distance
    }

    pub fn get_normalizer(&self) -> f64 {
        self.normalizer
    }
    pub fn set_initial_solution(&mut self, arr: &mut [usize]) {
        self.initial_solution = arr.to_vec();
    }

    pub fn get_sum_of_distances(&self) -> f64 {
        self.sum_of_distances
    }

    fn normalizer(&mut self, reader: &Reader) {
        let mut arr: Vec<f64> = vec![0.0f64; self.num_of_cities as usize];
        arr = reader.get_distances_ordered(&self.initial_solution);
        self.max_distance = arr[0];
        let mut norm: f64 = 0.0;
        let mut range: usize = self.num_of_cities as usize;
        if arr.len() < self.num_of_cities as usize {
            range = arr.len();
        }

        for i in 0..range {
            if i == self.num_of_cities as usize - 1 {
                break;
            } else {
                norm += arr[i];
            }
        }
        self.normalizer = norm;
    }

    fn get_cities_connections(&mut self, reader: &Reader) {
        self.all_connections = reader.read_connections();
        self.all_cities = reader.read_cities();
    }

    pub fn add_dist(&mut self, cities: &mut [usize]) -> f64 {
        let mut sum: f64 = 0.0;
        for i in 1..self.num_of_cities as usize {
            let row: usize = usize::try_from(cities[i - 1]).unwrap() - 1;
            let column: usize = usize::try_from(cities[i]).unwrap() - 1;
            sum += self.all_connections[row][column];
           
        }
        sum
    }

    pub fn add_initial_distance(&mut self) {
        let mut sum: f64 = 0.0;
        for i in 1..self.num_of_cities as usize {
            sum += self.all_connections[self.initial_solution[i - 1] - 1]
                [self.initial_solution[i] - 1];
        }
        self.sum_of_distances = sum;
    }

    ///Fills all the unknown distances between two cities and saves then in self.all_connections
    pub fn fill_distances(&mut self) {
        for i in 0..1092 {
            for j in (i + 1)..1092 {
                if self.all_connections[i][j] == 0.0 || self.all_connections[j][i] == 0.0 {
                    let dist = self.get_unknown_distance(self.all_cities[i], self.all_cities[j]);

                    self.all_connections[i][j] = dist;
                    self.all_connections[j][i] = dist;
                }
            }
        }
    }

    /// Calculates the natural distance between two cities
    ///
    /// # Arguments
    ///
    /// * `city1` - A City instance
    /// * `city21 - A City instance
    pub fn get_unknown_distance(&mut self, city1: City, city2: City) -> f64 {
        self.get_nat_distance(city1, city2) * self.max_distance
    }

    ///Calculates the natural distance between two cities
    pub fn get_nat_distance(&self, city1: City, city2: City) -> f64 {
        let A = pow(
            sin((Self::to_rad(city2.lat) - Self::to_rad(city1.lat)) / 2.0),
            2.0,
        ) + (cos(Self::to_rad(city1.lat))
            * cos(Self::to_rad(city2.lat))
            * pow(
                sin((Self::to_rad(city2.long) - Self::to_rad(city1.long)) / 2.0),
                2.0,
            ));

        let C = 2.0 * atan2(sqrt(A), sqrt(1.0 - A));
        let R = 6373000.0;
        R * C
    }

    ///Converts degrees to radians
    fn to_rad(num: f64) -> f64 {
        num * PI / 180.0
    }

    ///Modifies the passed vector and randomly generates from a seed an initial solution
    pub fn get_initial_solution(&mut self, cities: &mut Vec<usize>, seed: u64) -> Vec<usize> {
        let mut r = StdRng::seed_from_u64(seed);
        for i in 0..self.num_of_cities as usize {
            let n: u16 = r.gen();
            if n as usize != i {
                let value = cities[i];
                let index = n % u16::try_from(self.num_of_cities).unwrap();
                cities[i] = cities[index as usize];
                cities[index as usize] = value;
            }
        }
        self.initial_solution = cities.to_vec();
        cities.to_vec()
    }

    ///Modifies the passed slice and swaps two cities creating a neighboring solution
    pub fn get_neighbor(&mut self, cities: &mut [usize]) {
        self.n1 = self.r.gen::<usize>() % self.num_of_cities;
        self.n2 = self.r.gen::<usize>() % self.num_of_cities;
        while self.n1 == self.n2 {
            self.n1 = self.r.gen::<usize>() % self.num_of_cities;
            self.n2 = self.r.gen::<usize>() % self.num_of_cities;
        }

        //get the original distance between neighbors before swapping
        let previous_distances: f64 = self.get_sum(cities);

        //swapping
        let value = cities[self.n1 as usize];
        cities[self.n1 as usize] = cities[self.n2 as usize];
        cities[self.n2 as usize] = value;

        //getting the new distances after swapping
        let new_distances: f64 = self.get_sum(cities);

        //adding and substracitng the distances to set the new updated sum of distances
        self.sum_of_distances = self.sum_of_distances - previous_distances + new_distances;
    }

    pub fn swap(&mut self, cities : &mut [usize]){
        let previous_distances: f64 = self.get_sum(cities);
        let value = cities[self.n1 as usize];
        cities[self.n1 as usize] = cities[self.n2 as usize];
        cities[self.n2 as usize] = value;
        let new_distances: f64 = self.get_sum(cities);
        self.sum_of_distances = self.sum_of_distances - previous_distances + new_distances;
    }

    ///Returns the sum of distances between two swapped cities
    fn get_sum(&mut self, cities: &mut [usize]) -> f64 {
        let mut sum: f64 = 0.0;
        let mut id: [usize; 6] = [1093; 6];
        if self.n2 < self.n1 {
            let value = self.n1;
            self.n1 = self.n2;
            self.n2 = value;
        }

        if self.n1 > 0 && self.n1 < (cities.len() - 1) {
            id[0] = cities[self.n1 - 1] - 1;
            id[1] = cities[self.n1] - 1;
            id[2] = cities[self.n1 + 1] - 1;
        }

        if self.n2 > 0 && self.n2 < (cities.len() - 1) {
            id[3] = cities[self.n2 - 1] - 1;
            id[4] = cities[self.n2] - 1;
            id[5] = cities[self.n2 + 1] - 1;
        }

        if self.n1 == 0 {
            id[1] = cities[self.n1] - 1;
            id[2] = cities[self.n1 + 1] - 1;
        }

        if self.n1 + 1 == self.n2 {
            id[2] = 1093;
        }

        if self.n2 == cities.len() - 1 {
            id[3] = cities[self.n2 - 1] - 1;
            id[4] = cities[self.n2] - 1;
        }

        for i in 0..5 {
            if id[i] == 1093 || id[i + 1] == 1093 {
                continue;
            } else if i == 2 {
                continue;
            }
            sum = sum.mul_add(1.0, self.all_connections[id[i]][id[i + 1]]);
        }
        sum
    }

    ///
    ///Function that returns a vector of city ids to its previous state
    ///
    pub fn undo(&mut self, cities: &mut [usize]) {
        let previous = self.get_sum(cities);
        let value = cities[self.n1];
        cities[self.n1] = cities[self.n2];
        cities[self.n2] = value;
        let next = self.get_sum(cities);
        self.sum_of_distances = self.sum_of_distances - previous + next;
    }
}
