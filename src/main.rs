use sqlite::Result;
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

    extern crate num_cpus;
    let num = num_cpus::get();
    dbg!(num);

    let cases: Cases = Cases::new();
    let cities: Vec<usize> = cases.l150;
    let mut sa: SimAnn = SimAnn::new(cities.len().try_into().unwrap(), &cities);
    sa.prepare();

    let mut sol: Solution = Solution::new(0.002, 489943.0, 0.95, &cities, 2000);
    sol.threshold_acceptance();

    Ok(())
}
