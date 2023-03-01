use crate::city::City;
use crate::reader::Reader;
use crate::testCases::Cases;
use libm::{atan2, cos, pow, sin, sqrt};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::convert::TryFrom;
use std::f64::consts::PI;

//use simulated_annealing::sa::SimAnn::{to_rad};

pub struct Solution {
    pub cities: Vec<u16>,
    pub cost: f64,
}

pub struct SimAnn {
    temperature: f64,
    initial_solution: Vec<u16>,
    new_solution: Vec<u16>,
    num_of_cities: u16,
    n1: u16,
    n2: u16,

    sum_of_distances: f64,
    normalizer: f64,

    all_cities: Vec<City>,
    all_connections: Vec<Vec<f64>>, //vec![vec![f64;1092];1092],
    max_distance: f64,
    r: StdRng,
    //solution: Solution,
}

impl SimAnn {
    pub fn new(num: u16, list_of_cities: Vec<u16>) -> Self {
        dbg!("siman");
        Self {
            //reader : Reader::new( "../db/citiesDB.db"),
            initial_solution: list_of_cities,
            temperature: 0.0,
            new_solution: vec![0; num as usize],
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
            r: StdRng::seed_from_u64(7),
            // solution: Solution {
            //     cities: list_of_cities,
            //     cost: 0.0,
            // },
        }
    }

    pub fn prepare(&mut self) {
        let reader: Reader = Reader::new("db/citiesDB.db");

        self.get_cities_connections(&reader);
        self.normalizer(&reader);
        self.sum_of_distances = self.add_distances();

        println!("cost : {}", (self.sum_of_distances / self.normalizer));
        println!("dist_max : {}", self.max_distance);
        println!("norm : {}", self.normalizer);
        println!(
            "1, 7 , db : {:.15} , fn : {:.15}",
            self.all_connections[0][6],
            self.get_nat_distance(self.all_cities[0], self.all_cities[6])
        );
        self.get_initial_solution();
        self.get_neighbor();
    }

    pub fn get_cost(&self,) -> f64 {
        self.sum_of_distances / self.normalizer
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
        let mut count: usize = 0;
        let mut sum: f64 = 0.0;
        let mut norm: f64 = 0.0;
        for dist in &*arr {
            sum += dist;
            //println!("{:.15}", dist);
            count += 1;
            if arr.len() > self.num_of_cities.into() {
                if count == self.num_of_cities.into() {
                    break;
                } else {
                    norm += dist;
                }
            }
        }
        self.normalizer = norm;
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
                dbg!(i,dist);
            } else {
                let dist = self.all_connections[row][column];
                sum += dist;
                dbg!(i, dist);
            }
        }
        sum
    }


    fn fill_distances(&mut self){
        for i in 1..1092 as usize {
            // let mut row: usize = usize::try_from(self.initial_solution[i - 1]).unwrap() - 1;
            // let mut column: usize = usize::try_from(self.initial_solution[i]).unwrap() - 1;

            // if self.initial_solution[i - 1] > self.initial_solution[i] {
            //     row = usize::try_from(self.initial_solution[i]).unwrap() - 1;
            //     column = usize::try_from(self.initial_solution[i - 1]).unwrap() - 1;
            // }
            for j in 1..1092 as usize {
                if self.all_connections[i][j] == 0.0 {
                let dist = self.get_unknown_distance(self.all_cities[i], self.all_cities[j]);
               
                self.all_connections[i][j] = dist;
                self.all_connections[j][i] = dist;
            } 
            }
            
        }
    }

    fn get_cities_connections(&mut self, reader: &Reader) {
        self.all_connections = reader.read_connections();
        self.all_cities = reader.read_cities();
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

    fn get_initial_solution(&mut self) {
        let mut r = StdRng::seed_from_u64(42);
        for i in 0..self.num_of_cities as usize {
            let n: u16 = r.gen();
            if n as usize != i {
                let value = self.initial_solution[i];
                let index = n % self.num_of_cities;
                self.initial_solution[i] = self.initial_solution[index as usize];
                self.initial_solution[index as usize] = value;
            }
        }
        let new_initial_solution = self
            .initial_solution
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",");
        dbg!(new_initial_solution);
    }

    fn get_neighbor(&mut self) {
        self.n1 = self.r.gen::<u16>() % self.num_of_cities;
        self.n2 = self.r.gen::<u16>() % self.num_of_cities;
        let value = self.initial_solution[self.n1 as usize];
        self.initial_solution[self.n1 as usize] = self.initial_solution[self.n2 as usize];
        self.initial_solution[self.n2 as usize] = value;
        let new_initial_solution = self
            .initial_solution
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",");
        dbg!(self.n1, self.n2, new_initial_solution);
        //delete this later
        self.update_sum();
    }

    fn ctrlz(&mut self) {
        let value = self.initial_solution[self.n1 as usize];
        self.initial_solution[self.n1 as usize] = self.initial_solution[self.n2 as usize];
        self.initial_solution[self.n2 as usize] = value;
    }

    fn calculate_batch(&mut self, temp: f64, solution: &Vec<u16>) -> (f64, f64) {
        let mut counter = 0;
        let mut sum: f64 = 0.0;
        let batch = 5;
        while counter < batch {
            self.get_neighbor();
        }
        (0.0, 0.0)
    }

    fn update_sum(&mut self) {
        self.fill_distances();
        let n1 = usize::try_from(self.n1).unwrap();
        let n2 = usize::try_from(self.n2).unwrap();
        
        let id1: usize = usize::try_from(self.initial_solution[n1 -1] - 1).unwrap();
        let id2: usize = usize::try_from(self.initial_solution[n1 ] - 1).unwrap();
        let id3: usize = usize::try_from(self.initial_solution[n1+1] - 1).unwrap();

        let id4 = usize::try_from(self.initial_solution[n2 -1]-1).unwrap();
        let id5 = usize::try_from(self.initial_solution[n2 ]-1).unwrap();
        let id6 = usize::try_from(self.initial_solution[n2+1]-1).unwrap();

        self.sum_of_distances +=
            self.all_connections[id1][id2]
            + self.all_connections[id2][id3]
            + self.all_connections[id4][id5]
            + self.all_connections[id5][id6];

         self.sum_of_distances -=
            self.all_connections[id1][id5]
            + self.all_connections[id5][id3]
            + self.all_connections[id4][id2]
            + self.all_connections[id2][id6];
        dbg!(id1, id2, id3, id4, id5, id6);
        dbg!( self.all_connections[id1][id2]
            , self.all_connections[id2][id3]
            , self.all_connections[id4][id5]
              , self.all_connections[id5][id6]);
        dbg!(self.all_connections[id1][id5]
             , self.all_connections[id5][id3]
            , self.all_connections[id4][id2]
            , self.all_connections[id2][id6]);
        dbg!(self.sum_of_distances);
        
        
        dbg!(self.add_distances());
    }

    fn threshold_acceptance() {}
}
