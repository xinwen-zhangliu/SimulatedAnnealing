use sqlite::Result;
use std::env;

use simulated_annealing::sa::SimAnn;
use simulated_annealing::testCases::Cases;
use simulated_annealing::tsp::Solution;
use simulated_annealing::threadspawninator::TSI;
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

    //extern crate num_cpus;
    let num : usize = num_cpus::get();
    dbg!(num);

    

    
    let cases: Cases = Cases::new();
    let cities: Vec<usize> = cases.l40;
//    let mut sa: SimAnn = SimAnn::new(cities.len().try_into().unwrap(), &cities, );
  //  sa.prepare();

    // let dist =sa.add_dist(&mut vec![54,483,186,980,327,164,984,491,492,489,4,817,
    //               978,6,5,165,3,333,981,820,332,982,816,7,653,
    //               654,490,2,656,657,168,1,163,172,496,815,329,
    //                       493,979,331]);
    
    // let norm = sa.get_normalizer();

    // let mut thread = TSI::new(40);
    // thread.spawn_threads(40);

    let mut sol: Solution = Solution::new(
                    0.002,
                    800000.0,
                    0.95,
                    &Cases::new().l40,
                    2000,
                    13603085585784548072,
                    11058656509428391188,
                );
//11058656509428391188, 13603085585784548072

     

        sol.threshold_acceptance();
    Ok(())
}
