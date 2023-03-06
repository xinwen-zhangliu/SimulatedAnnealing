use crate::city::City;
use crate::reader::Reader;
use crate::testCases::Cases;
use libm::{atan2, cos, pow, sin, sqrt};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::convert::TryFrom;
use std::f64::consts::PI;

//use simulated_annealing::sa::SimAnn::{to_rad};

pub struct SimAnn {
    initial_solution: Vec<usize>,
    num_of_cities: usize,
    n1: usize,
    n2: usize,

    sum_of_distances: f64,
    normalizer: f64,

    all_cities: Vec<City>,
    all_connections: Vec<Vec<f64>>, //vec![vec![f64;1092];1092],
    max_distance: f64,
    r: StdRng,
}

impl SimAnn {
    pub fn new(num: usize, cities: &Vec<usize>) -> Self {
        let new: Vec<usize> = cities.to_vec();
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
            r: StdRng::seed_from_u64(7),
        }
    }

    pub fn prepare(&mut self) {
        let reader: Reader = Reader::new("db/citiesDB.db");

        self.get_cities_connections(&reader);
        self.normalizer(&reader);
        //self.sum_of_distances = self.add_initial_dist();
        self.fill_distances();
    }

    pub fn get_cost(&mut self, cities: &mut Vec<usize>) -> f64 {
        self.update_sum(cities);
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

    fn add_dist(&mut self, cities: &mut Vec<u16>) -> f64 {
        let mut sum: f64 = 0.0;
        for i in 1..self.num_of_cities as usize {
            let mut row: usize = usize::try_from(cities[i - 1]).unwrap() - 1;
            let mut column: usize = usize::try_from(cities[i]).unwrap() - 1;
            let dist = self.all_connections[row][column];
            sum += dist;
        }
        self.sum_of_distances = sum;
        sum
    }

    pub fn add_initial_dist(&mut self) -> f64 {
        dbg!("adding initial distances", &self.initial_solution);
        let mut sum: f64 = 0.0;
        for i in 1..self.num_of_cities as usize {
            let mut row: usize = usize::try_from(self.initial_solution[i - 1]).unwrap() - 1;
            let mut column: usize = usize::try_from(self.initial_solution[i]).unwrap() - 1;
            let dist = self.all_connections[row][column];
            sum += dist;
        }
        dbg!(sum);
        self.sum_of_distances = sum;
        sum
    }

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

    pub fn get_initial_solution(&mut self, cities: &mut Vec<usize>) -> Vec<usize> {
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
        self.initial_solution = cities.to_vec();
        self.add_initial_dist();
        //self.sum_of_distances = self.add_dist();
        dbg!(cities.to_vec());
        cities.to_vec()
    }

    pub fn get_neighbor(&mut self, cities: &mut [usize]) {
        self.n1 = self.r.gen::<usize>() % self.num_of_cities;
        self.n2 = self.r.gen::<usize>() % self.num_of_cities;
        while self.n1 == self.n2 {
            self.n1 = self.r.gen::<usize>() % self.num_of_cities;
            self.n2 = self.r.gen::<usize>() % self.num_of_cities;
        }

        // self.swap(self.n1, self.n2);
        let value = cities[self.n1 as usize];
        cities[self.n1 as usize] = cities[self.n2 as usize];
        cities[self.n2 as usize] = value;
    }

    pub fn swap(&self, i1: usize, i2: usize, cities: &mut [u16]) {
        let value = cities[self.n1 as usize];
        cities[self.n1 as usize] = cities[self.n2 as usize];
        cities[self.n2 as usize] = value;
    }

    fn update_sum(&mut self, cities: &[usize]) {
        // if self.n1-1 > 0 {
        //     self.sum_of_distances +=  self.all_connections[cities[self.n1-1]][cities[self.n1]];
        // }

        // if self.n1+1 < cities.len()-1 {

        // }

        // let (mut id1, mut id2, mut id3, mut id4, mut id5, mut id6): (
        //     usize,
        //     usize,
        //     usize,
        //     usize,
        //     usize,
        //     usize,
        // ) = (0, 0, 0, 0, 0, 0);

        //case where the dwaped items are beside each other

        if self.n1 > self.n2 {
                    let value = self.n1;
                    self.n1 = self.n2;
                    self.n2 = value;
        }

        

        
        // let mut next_to = false;
        // if self.n1 != 0 {
        //     if  self.n1 - 1 == self.n2 {
        //         if self.n1 > self.n2 {
        //             let value = self.n1;
        //             self.n1 = self.n2;
        //             self.n2 = value;
        //         }
        //         next_to = true;
        //     }
        // }else if self.n1+1 == self.n2{
        //     if self.n1 > self.n2 {
        //             let value = self.n1;
        //             self.n1 = self.n2;
        //             self.n2 = value;
        //         }
        //         next_to = true;
            
        // }

        let mut id: [usize; 6] = [1093; 6];

        //let (mut p1, mut p2, mut p3, mut p4): (f64, f64, f64, f64) = (0.0, 0.0, 0.0, 0.0);

        if (self.n1 > 0 && self.n1 < (cities.len() - 1)) {
            id[0] = cities[self.n1 - 1] - 1;
            id[1] = cities[self.n1] - 1;
            id[2] = cities[self.n1 + 1] - 1;
        }

        if (self.n2 > 0 && self.n2 < (cities.len() - 1)) {
            id[3] = cities[self.n2 - 1] - 1;
            id[4] = cities[self.n2] - 1;
            id[5] = cities[self.n2 + 1] - 1;
        }

        if self.n1 == self.num_of_cities - 1 {
            id[0] = cities[self.n1 - 1] - 1;
            id[1] = cities[self.n1] - 1;
        }

        if self.n1 == 0 {
            id[1] = cities[self.n1] - 1;
            id[2] = cities[self.n1 + 1] - 1;
        }

        if self.n2 == self.num_of_cities - 1 {
            id[3] = cities[self.n2 - 1] - 1;
            id[4] = cities[self.n2] - 1;
        }

        if self.n2 == 0 {
            id[4] = cities[self.n2] - 1;
            id[5] = cities[self.n2 + 1] - 1;
        }

        if id[1] == id[3] &&  id[2] ==id[4]{
            id[2] = 1093;
            id[3] = 1093;
        }

        let mut id_minus = id.clone();
        let value = id_minus[1];
        id_minus[1] = id_minus[4];
        id_minus[4] = value;

        for i in 0..5 {
            if id[i] == 1093 || id[i + 1] == 1093 {
                continue;
            } else if i == 2 {
                continue;
            }

            self.sum_of_distances += self.all_connections[id[i]][id[i + 1]];
        }

        for i in 0..5 {
            if id[i] == 1093 || id[i + 1] == 1093 {
                continue;
            } else if i == 2 {
                continue;
            }

            self.sum_of_distances -= self.all_connections[id_minus[i]][id_minus[i + 1]];
        }
        //dbg!(self.all_connections[981][]);

        //dbg!(cities, id, self.sum_of_distances);
    }
}
