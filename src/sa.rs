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
    norm: f64,

    all_cities: Vec<City>,
    all_connections: Box<Vec<Vec<f64>>>,
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
            norm: 0.0,
            all_cities: vec![City {
                id: 0,
                lat: 0.0,
                long: 0.0,
            }],
            all_connections: Box::new(vec![vec![0.0f64; 1092]; 1092]),
            max_distance: 0.0,
        }
    }

    pub fn prepare(&mut self) {
        //read database and get all the cities and connections
        let case: Cases = Cases::new();
        let reader: Reader = Reader::new(case.l150, "db/citiesDB.db");
        let mut arr: Box<Vec<f64>> = Box::new(vec![0.0f64]);

        arr = Box::new(*reader.get_distances_ordered());
        self.max_distance = arr[0];
        let mut count: usize = 0;
        let mut sum: f64 = 0.0;
        let mut norm: f64 = 0.0;
        for dist in &*arr {
            sum += dist;
            println!("{}", dist);
            count += 1;
            if arr.len() > self.num_of_cities.into() {
                if count == self.num_of_cities.into() {
                    break;
                } else {
                    norm += dist;
                }
            }
        }
        println!("count : {}, norm  : {}", count, norm);
        //self.sum_of_distances = sum;
        self.all_cities = reader.read_cities();
        //self.all_cities = (&self.all_cities[..]).to_vec();
        // for (pos, e) in self.all_cities.iter().enumerate() {
        //     println!("{}: {:?}", pos, e);
        // }
        self.get_connections(reader);

        self.add_distances();
        println!("cost : {}", (self.sum_of_distances / norm));
        println!("dist_max : {}" , self.max_distance);
        println!("norm : {}" , norm);
        //println!("dist : {}", 4129508.339517763 * 180088219.480000019073486);
    }

    fn normalizer(&self, arr: &[f64]) {
        //get the list of distances from high-to-low

        //L if L.length < num_of_cities-1

        //else take the first num_of_cities-1 distances

        //add everything
    }

    fn add_distances(&mut self) {
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
        self.sum_of_distances = sum;
        //println!(" total_dist : {}", sum);
        //let dist1 = self.get_unknown_distance(self.all_cities[492], self.all_cities[495]);
        //println!("(493, 496){}", dist1)
    }

    fn get_connections(&mut self, reader: Reader) {
        self.all_connections = reader.read_connections();
    }

    fn get_unknown_distance(&mut self, city1: City, city2: City) -> f64 {
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
        let mut nat_distance = R * C;

        // println!(
        //     "{:?} , {:?}, {}, {}",
        //     city1,
        //     city2,
        //     nat_distance,
        //     nat_distance * self.max_distance
        // );

        return nat_distance * self.max_distance ;
    }

    fn to_rad(num: f64) -> f64 {
        num * PI / 180.0
    }

    fn initial_solution(&self) {}
}
