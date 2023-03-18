extern crate clap;
use clap::{arg, value_parser, ArgAction, Command};
use simulated_annealing::testCases::Cases;
use simulated_annealing::threadspawninator::TSI;
use sqlite::Result;
use std::fs;
use std::path::PathBuf;

const EPSILON_DEFAULT: f64 = 0.0001;
const PHI_DEFAULT: f64 = 0.98;

fn main() -> Result<()> {
    let mut cities = Cases::new().l40;
    let mut epsilon = EPSILON_DEFAULT;
    let mut phi = PHI_DEFAULT;
    let mut batch_size = 1000;
    let mut number_of_iterations = 120;
    let mut neighbor_seed: u64 = 0;
    let mut initial_sol_seed: u64 = 0;

    /*
    Command line arguments
    */
    let matches = Command::new("Simulated Annealing for Travelling Salesman Problem")
        .next_line_help(true)
        .arg(
            arg!(
                -f --file <PATH>
                    "Use cities included in the file
\nNOTE there must be a comma break between each city ")
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(-e --epsilon <VALUE>
                 "The algorithm will stop when the temperature reaches this number.")
            .required(false)
            .action(ArgAction::Set)
            .value_parser(value_parser!(f64)),
        )
        .arg(
            arg!(-p --phi <VALUE>
                 "The rate at which the temperature decreases.")
            .required(false)
            .action(ArgAction::Set)
            .value_parser(value_parser!(f64)),
        )
        .arg(
            arg!(-b --batch <VALUE>
                 "The batch size.")
            .required(false)
            .action(ArgAction::Set)
            .value_parser(value_parser!(u32)),
        )
        .arg(
            arg!(-n --niter <VALUE>
                 "The total number of simulated annealing instances to run.")
            .required(false)
            .action(ArgAction::Set)
            .value_parser(value_parser!(usize)),
        )
        .arg(
            arg!( --neigh <VALUE>
                 "The seed used for the neighbor randomizer.")
            .required(false)
            .action(ArgAction::Set)
            .value_parser(value_parser!(u64)),
        )
        .arg(
            arg!( --init <VALUE>
                 "The seed used for the randomizer that generates the initial solution.")
            .required(false)
            .action(ArgAction::Set)
            .value_parser(value_parser!(u64)),
        )
        .arg(
            arg!( --cities40
                 "Run the heuristic with the default 40 cities test case.")
            .required(false)
            .action(ArgAction::SetTrue),
        )
        .arg(
            arg!( --cities150
                 "Run the heuristic with the default 150 cities  case.")
            .required(false)
            .action(ArgAction::SetTrue),
        )
        .get_matches();
    #[allow(unused_variables)]
    if let Some(cities_path) = matches.get_one::<PathBuf>("file") {
        let path : String= cities_path
            .clone()
            .into_os_string()
            .into_string()
            .unwrap();
        println!("Path: {}", &path);
        let number: usize = 1093;
        let contents = fs::read_to_string(&path).expect("Should have been able to read the file");
        cities = contents
            .split(",")
            .map(|x| x.parse::<usize>().unwrap_or(1093))
            .filter(|x| x < &number)
            .collect();
    }
    if let Some(c40) = matches.get_one::<bool>("cities40") {
        if *c40 {
            cities = Cases::new().l40;
        }
    }

    if let Some(c150) = matches.get_one::<bool>("cities150") {
        if *c150 {
            cities = Cases::new().l150;
        }
    }

    if let Some(some_epsilon) = matches.get_one::<f64>("epsilon") {
        println!("Value for epsilon: {}", some_epsilon);
        epsilon = *some_epsilon;
    }
    if let Some(some_phi) = matches.get_one::<f64>("phi") {
        println!("Value for phi: {}", some_phi);
        phi = *some_phi;
    }
    if let Some(some_batch_size) = matches.get_one::<u32>("batch") {
        println!("Value for batch size: {}", some_batch_size);
        batch_size = *some_batch_size;
    }
    if let Some(some_number_of_iterations) = matches.get_one::<usize>("niter") {
        println!(
            "Value for number of iterations: {}",
            some_number_of_iterations
        );
        number_of_iterations = *some_number_of_iterations;
    }
    if let Some(some_neighbor_seed) = matches.get_one::<u64>("neigh") {
        println!("Seed for neighbors: {}", some_neighbor_seed);
        neighbor_seed = *some_neighbor_seed;
    }
    if let Some(some_initial_sol_seed) = matches.get_one::<u64>("init") {
        println!("Seed for initial solution: {}", some_initial_sol_seed);
        initial_sol_seed = *some_initial_sol_seed;
    }

    let mut thread = TSI::new();
    thread.spawn_threads(
        number_of_iterations,
        epsilon,
        phi,
        batch_size,
        neighbor_seed,
        initial_sol_seed,
        cities,
    );

    Ok(())
}
