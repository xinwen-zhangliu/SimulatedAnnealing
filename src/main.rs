//use rusqlite::NO_PARAMS; deprecated
//use std::collections::HashMap;
//use sqlite::State;
use sqlite::{Connection, Result};
use std::convert::TryFrom;
use std::env;

use simulated_annealing::city::City;
//use city::City;
//#[path="city.rs"]
//pub mod city;
use simulated_annealing::sa::SimAnn;
use simulated_annealing::testCases::Cases;

fn main() -> Result<()> {
    /*
      Command line arguments
    */
    // use cargo run -- num cities

    //let args: Vec<String> = env::args().collect();
    //let num_of_cities = &args[1].parse::<i32>().unwrap();
    //we get a slice of previous vector
    //let citiesList = &args[2..num_of_cities-1];
    // if !args.is_empty() {
    //     for s in args{
    //         println!("{}", s);
    //     }
    // }

    let conn = Connection::open("../db/citiesDB.db")?;
    let num: u16 = 150;
    let cases: Cases = Cases::new();
    let mut sa: SimAnn = SimAnn::new(num, cases.l150);
    sa.prepare();

   
    let mut all_cities: Vec<City> = Vec::new();
    for row in conn
        .prepare("SELECT * FROM cities;")
        .unwrap()
        .into_iter()
        .map(|row| row.unwrap())
    {
        let city = City {
            id: row.read::<i64, _>("id"),
            lat: row.read::<f64, _>("latitude"),
            long: row.read::<f64, _>("longitude"),
        };
        all_cities.push(city);
    }
    

    // for (pos, e) in all_cities.iter().enumerate() {
    //     println!("{}: {:?}", pos, e);
    // }

    let query = "SELECT * FROM connections;";

    let mut all_connections = vec![vec![0.0f64; 1092]; 1092];
    for row in conn
        .prepare(query)
        .unwrap()
        .into_iter()
        .map(|row| row.unwrap())
    {
        let city1 = row.read::<i64, _>("id_city_1");
        let city2 = row.read::<i64, _>("id_city_2");
        let distance = row.read::<f64, _>("distance");
        let c1 = usize::try_from(city1).unwrap();
        let c2 = usize::try_from(city2).unwrap();
        all_connections[c1 - 1][c2 - 1] = distance;
    }

        //println!("{}, {} : {}", 1090, 1092, all_connections[1090-1][1092-1]);

    // for city in allCities {
    //     println!("{:?}", city);
    // }

    Ok(())
}
