use crate::city::City;
use crate::reader::Reader;
use crate::testCases::Cases;
use libm::{atan2, cos, pow, sin, sqrt};
use std::convert::TryFrom;
use std::f64::consts::PI;

//use simulated_annealing::sa::SimAnn::{to_rad};

pub struct SimAnn {
    temperature: f64,
    initial_solution: Vec<u16>,
    num_of_cities: u16,
    n1: u16,
    n2: u16,

    sum_of_distances: f64,
    normalizer: f64,

    all_cities: Vec<City>,
    all_connections: Vec<Vec<f64>>,
    max_distance: f64,
}

impl SimAnn {
    pub fn new(num: u16, list_of_cities: Vec<u16>) -> Self {
        Self {
            initial_solution: list_of_cities,
            temperature: 0.0,
            num_of_cities: num,
            n1: 0,
            n2: 0,
            sum_of_distances: 0.0,
            normalizer: 0.0,
            all_cities: vec![City {
                id: 0,
                lat: 0.0,
                long: 0.0,
            }],
            all_connections: vec![vec![0.0f64; 1092]; 1092],
            max_distance: 0.0,
        }
    }

    pub fn prepare(&mut self) {
        //read database and get all the cities and connections
       
        let reader: Reader = Reader::new( "db/citiesDB.db");
        self.get_cities_connections(&reader);
        self.normalizer(&reader);
        self.sum_of_distances = self.add_distances();

        println!("cost : {}", (self.sum_of_distances / self.normalizer));
        println!("dist_max : {}", self.max_distance);
        println!("norm : {}", self.normalizer);
    }

    pub fn get_cost(&self) -> f64{
        self.sum_of_distances / self.normalizer
    }

    pub fn get_max_distance(&self) -> f64{
        self.max_distance
    }

    pub fn get_normalizer(&self) -> f64 {
        self.normalizer
    }
    
    fn normalizer(&mut self, reader : &Reader) {
         
        let mut arr: Vec<f64> = vec![0.0f64];
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
            if self.all_connections[usize::try_from(self.initial_solution[i - 1]).unwrap() - 1]
                [usize::try_from(self.initial_solution[i]).unwrap() - 1]
                == 0.0
            {
                let dist = self.get_unknown_distance(
                    self.all_cities[(self.initial_solution[i - 1] as usize) - 1],
                    self.all_cities[(self.initial_solution[i] as usize) - 1],
                );
                // println!(
                //     "{}, {}",
                //     self.initial_solution[i - 1] - 1,
                //     self.initial_solution[i] - 1
                // );
                sum += dist;
                self.all_connections[usize::try_from(self.initial_solution[i - 1]).unwrap() - 1]
                    [usize::try_from(self.initial_solution[i]).unwrap() - 1] = dist;
                //println!("d : {}", dist);
            } else {
                let dist = self.all_connections
                    [usize::try_from(self.initial_solution[i - 1]).unwrap() - 1]
                    [usize::try_from(self.initial_solution[i]).unwrap() - 1];
                sum += dist;
                // println!(
                //     "{}, {}, {}",
                //     self.initial_solution[i - 1],
                //     self.initial_solution[i],
                //     dist
                // );
            }

            // println!(
            //     "dw : {}",
            //     self.all_connections[usize::try_from(self.initial_solution[i - 1]).unwrap()]
            //         [usize::try_from(self.initial_solution[i]).unwrap()]
            // );
        }
        sum
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

    fn initial_solution(&self) {}
}
