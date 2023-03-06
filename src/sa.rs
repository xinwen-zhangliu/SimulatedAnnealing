use crate::city::City;
use crate::reader::Reader;
use crate::testCases::Cases;
use libm::{atan2, cos, pow, sin, sqrt};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use rand_distr::Uniform;
use std::convert::TryFrom;
use std::f64::consts::PI;

//use simulated_annealing::sa::SimAnn::{to_rad};

pub struct SimAnn {
    initial_solution: Vec<u16>,
    num_of_cities: usize,
    n1: usize,
    n2: usize,

    sum_of_distances: f64,
    normalizer: f64,

    all_cities: Vec<City>,
    all_connections: Vec<Vec<f64>>, //vec![vec![f64;1092];1092],
    max_distance: f64,
    r: StdRng,
    //uni : Uniform,
}

impl SimAnn {
    pub fn new(num: usize, cities: &Vec<u16>) -> Self {
        let new: Vec<u16> = cities.to_vec();
        Self {
            initial_solution: new,
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
            //uni : Uniform::new(0, num ),
            r: StdRng::seed_from_u64(7),
        }
    }

    pub fn prepare(&mut self) {
        let reader: Reader = Reader::new("db/citiesDB.db");

        self.get_cities_connections(&reader);
        self.normalizer(&reader);
        //self.get_initial_solution();
        self.sum_of_distances = self.add_distances();
        self.fill_distances();

        // self.get_neighbor();
        // self.add_distances();
        // self.update_sum();
    }

    pub fn get_cost(&self, cities: &mut Vec<u16>) -> f64 {
        self.add_dist(cities) / self.normalizer
    }

    pub fn get_max_distance(&self) -> f64 {
        self.max_distance
    }

    pub fn get_normalizer(&self) -> f64 {
        self.normalizer
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

    fn add_dist(&self, cities: &mut Vec<u16>) -> f64 {
        let mut sum: f64 = 0.0;
        for i in 1..self.num_of_cities as usize {
            let mut row: usize = usize::try_from(cities[i - 1]).unwrap() - 1;
            let mut column: usize = usize::try_from(cities[i]).unwrap() - 1;
            let dist = self.all_connections[row][column];
            sum += dist;
        }
        sum
    }

    fn add_distances(&mut self) -> f64 {
        let mut sum: f64 = 0.0;
        for i in 1..self.num_of_cities as usize {
            let mut row: usize = usize::try_from(self.initial_solution[i - 1]).unwrap() - 1;
            let mut column: usize = usize::try_from(self.initial_solution[i]).unwrap() - 1;

            if self.initial_solution[i - 1] > self.initial_solution[i] {
                row = usize::try_from(self.initial_solution[i]).unwrap() - 1;
                column = usize::try_from(self.initial_solution[i - 1]).unwrap() - 1;
            }
            if self.all_connections[row][column] == 0.0 {
                let dist = self.get_unknown_distance(self.all_cities[row], self.all_cities[column]);
                sum += dist;
                self.all_connections[row][column] = dist;
                self.all_connections[column][row] = dist;
                // dbg!(row, column, dist);
            } else {
                let dist = self.all_connections[row][column];
                sum += dist;
                //dbg!(row, column, dist);
            }
        }
        sum
    }

    pub fn fill_distances(&mut self) {
        //dbg!(self.all_cities.len());
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

    pub fn get_unknown_distance(&mut self, city1: City, city2: City) -> f64 {
        self.get_nat_distance(city1, city2) * self.max_distance
    }

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

    fn to_rad(num: f64) -> f64 {
        num * PI / 180.0
    }

    pub fn get_initial_solution(&mut self, cities : &mut Vec<u16>) -> Vec<u16>  {
        let mut r = StdRng::seed_from_u64(42);
        for i in 0..self.num_of_cities as usize {
            let n: u16 = r.gen();
            if n as usize != i {
                let value = cities[i];
                let index = n % u16::try_from(self.num_of_cities).unwrap();
                cities[i] = cities[index as usize];
                cities[index as usize] = value;
            }
        }
        // let new_initial_solution = self
        //     .initial_solution
        //     .iter()
        //     .map(|x| x.to_string())
        //     .collect::<Vec<_>>()
        //     .join(",");
        // dbg!(new_initial_solution);
        // dbg!(self.add_distances());
        cities.to_vec()
    }

    pub fn get_neighbor(&mut self, cities: &mut [u16]){
        self.n1 = self.r.gen::<usize>() % self.num_of_cities;
        self.n2 = self.r.gen::<usize>() % self.num_of_cities;
        while self.n1 == self.n2 {
            self.n1 = self.r.gen::<usize>() % self.num_of_cities;
            self.n2 = self.r.gen::<usize>() % self.num_of_cities;
        }

        //dbg!(self.n1, self.n2);
        //let uni = Uniform::new(0, self.num_of_cities);
       // self.swap(self.n1, self.n2);
        let value = cities[self.n1 as usize];
            cities[self.n1 as usize] = cities[self.n2 as usize];
            cities[self.n2 as usize] = value;


        
       // cities.to_vec()
    }

    fn swap(&mut self , i1 : usize, i2 : usize, cities : &mut [u16] )    {
        let value = self.initial_solution[i1];
        self.initial_solution[i1] = self.initial_solution[i2];
        self.initial_solution[i2] = value;
        
    }

    fn update_sum(&mut self) {
        

        let id1: usize = usize::try_from(self.initial_solution[self.n1 - 1] - 1).unwrap();

        let id2: usize = usize::try_from(self.initial_solution[self.n1] - 1).unwrap();
        let id3: usize = usize::try_from(self.initial_solution[self.n1 + 1] - 1).unwrap();

        let id4 = usize::try_from(self.initial_solution[self.n2 - 1] - 1).unwrap();
        let id5 = usize::try_from(self.initial_solution[self.n2] - 1).unwrap();
        let id6 = usize::try_from(self.initial_solution[self.n2 + 1] - 1).unwrap();

        self.sum_of_distances += self.all_connections[id1][id2]
            + self.all_connections[id2][id3]
            + self.all_connections[id4][id5]
            + self.all_connections[id5][id6];

        self.sum_of_distances -= self.all_connections[id1][id5]
            + self.all_connections[id5][id3]
            + self.all_connections[id4][id2]
            + self.all_connections[id2][id6];

        // dbg!(id1, id2, id3, id4, id5, id6);
        // dbg!(
        //     self.all_connections[id1][id2],
        //     self.all_connections[id2][id3],
        //     self.all_connections[id4][id5],
        //     self.all_connections[id5][id6]
        // );
        // dbg!(
        //     self.all_connections[id1][id5],
        //     self.all_connections[id5][id3],
        //     self.all_connections[id4][id2],
        //     self.all_connections[id2][id6]
        // );

        // dbg!("------------------------------------------");
        // dbg!(self.sum_of_distances);
        // dbg!(self.add_distances());

        // let new : &Vec<u16>= &self.initial_solution;
        // dbg!(self.add_dist(&new));
    }
}
