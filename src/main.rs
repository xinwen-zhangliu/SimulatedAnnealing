//use rusqlite::NO_PARAMS; deprecated
//use std::collections::HashMap;
//use sqlite::State;
use sqlite::{Connection, Result};
//use std::convert::TryFrom;
use std::env;


use simulated_annealing::sa::SimAnn;
use simulated_annealing::testCases::Cases;
use simulated_annealing::tsp::Solution;

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

   
    
    let cases: Cases = Cases::new();
    let cities : Vec<usize>= cases.l40;
    let mut sa: SimAnn = SimAnn::new(cities.len().try_into().unwrap(), &cities);
    sa.prepare();
    
    
    let mut sol : Solution = Solution::new(0.002, 788999.0, 0.95, &cities, 700);
    sol.threshold_acceptance();


    // let new_cities : Vec<u16> = vec! [332,982,816,7,653,490,654,820,981,3,333,165,4,489,817,978,6,5,163,172,2,656,657,168,1,815,496,329,493,979,331,984,491,492,164,327,980,186,483,54];
    //     let mut sol : Solution = Solution::new(0.002, 788999.0, 0.95, &new_cities, 700);
    // sol.hill_descent(7);
    

    Ok(())
}
