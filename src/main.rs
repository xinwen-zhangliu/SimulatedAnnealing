//use rusqlite::NO_PARAMS; deprecated
//use std::collections::HashMap;
use sqlite::{Connection, Result};
use std::env;
use sqlite::State;



#[derive(Debug)]
struct City{
    id : i64 ,
    lat : f64,
    long : f64,
}



struct Connec{
    c1 : u16,
    c2 : u16,
    distance : f32,
}

// struct World{
//     all : [[u16; 1092]; 1092],
// }


fn main() -> Result<()>{
    // use cargo run -- num cities

    // let args: Vec<String> = env::args().collect();

    // let numOfCities = &args[1].parse::<i32>().unwrap();
    // let citiesList = &args[2];

  

    let conn = Connection::open("db/citiesDB.db")?;


    
    let mut all_cities :  Vec<City> = Vec::new();
    let mut i = 0;
    for row in conn.prepare(
        "SELECT * FROM cities;",
    ).unwrap()
        .into_iter()
        .map(|row| row.unwrap()){
            let city = City {
                id : row.read::<i64, _>("id"),
                lat : row.read::<f64, _>("latitude"),
                long : row.read::<f64, _>("longitude"),
            };
            all_cities.push(city);
        }

    // let allCities = stmtCities.query_map([], |row| {
    //     Ok( City {
    //         id : row.get(0)?,
    //         latitud : row.get(4)?,
    //         longitud : row.get(5)?, 
    //     })
    // })?;

    let query = "SELECT * FROM connections;";
  
    
    for row in conn
        .prepare(query)
        .unwrap()
        .into_iter()
        .map(|row| row.unwrap()){
            println!("city1 = {}", row.read::<i64, _>("id_city_1"));
            println!("city2 = {}", row.read::<i64, _>("id_city_2"));

        }



    // for city in allCities {
    //     println!("{:?}", city);
    // }




    Ok(())
}
