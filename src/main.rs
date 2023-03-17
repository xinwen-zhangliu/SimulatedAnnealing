extern crate clap;
use clap::{arg, value_parser, ArgAction, Command};
use simulated_annealing::testCases::Cases;
use simulated_annealing::threadspawninator::TSI;
use sqlite::Result;
use std::path::PathBuf;
use std::fs;


const EPSILON_DEFAULT: f64 = 0.0001;
const PHI_DEFAULT: f64 = 0.98;

fn main() -> Result<()> {
    let mut cities = Cases::new().l150;
    let mut epsilon = EPSILON_DEFAULT;
    let mut phi = PHI_DEFAULT;
    let mut batch_size = 1000;
    let mut number_of_iterations = 240;
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
\nNOTE there must be a line break between each city ")
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
            arg!( --neigh-seed <VALUE>
                 "The seed used for the neighbor randomizer.")
            .required(false)
            .action(ArgAction::Set)
            .value_parser(value_parser!(u64)),
        )
        .arg(
            arg!( --init-sol-seed <VALUE>
                 "The seed used for the randomizer that generates the initial solution.")
            .required(false)
            .action(ArgAction::Set)
            .value_parser(value_parser!(u64)),
        )
        .arg(
            arg!( --cities40
                 "Run the heuristic with the 40 cities test case.")
            .required(false)
            .action(ArgAction::SetTrue),
        )
        .arg(
            arg!( --cities150
                 "Run the heuristic with the 150 cities test case.")
            .required(false)
            .action(ArgAction::SetTrue),
        )
        .get_matches();

    if let Some(cities_path) = matches.get_one::<PathBuf>("file") {
        let path = cities_path.clone().into_os_string().into_string().unwrap();
        println!("Path: {}", &path);
        let number: usize = 1093;
        let contents = fs::read_to_string(&path).expect("Should have been able to read the file");
        cities = contents
            .split("\n")
            .map(|x| x.parse::<usize>().unwrap_or(1093))
            .filter(|x| x < &number)
            .collect();
        //cities = &new_cities;
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

    // println!(
    //     "two: {:?}",
    //     matches
    //         .get_one::<String>("two")
    //         .expect("please enter a value")
    // );
    // println!(
    //     "one: {:?}",
    //     matches
    //         .get_one::<String>("one")
    //         .expect("please enter a value ")
    // );

    //  best solution for 40 cities
    // let mut sol: SimAnn = SimAnn::new(
    //     0.002,
    //     800000.0,
    //     0.95,
    //     &Cases::new().l40,
    //     2000,
    //     18011140874099403753,
    //     17093148322891613803,
    // );
    // sol.threshold_acceptance();

    //  seeds for best solution so far 150 cities  7232223228056910425, 10503878646336740862
    //0.16289591628580316
    //    let mut sol: SimAnn = SimAnn::new(
    //                     0.001,
    //                     800000.0,
    //                     0.98,
    //                     &Cases::new().l150,
    //                     1000,
    //                     10503878646336740862,
    //                     7232223228056910425,
    //                 );
    // sol.threshold_acceptance();

    //    (0.16289591628580316, [978, 5, 6, 988, 174, 4, 489, 501, 164, 11, 984, 331, 999, 674, 500, 825, 680, 20, 670, 511, 350, 840, 660, 510, 343, 985, 504, 327, 334, 8, 662, 1003, 349, 995, 837, 979, 493, 509, 329, 496, 182, 172, 163, 676, 665, 673, 173, 2, 656, 815, 19, 505, 168, 832, 661, 663, 657, 829, 1, 986, 508, 9, 184, 667, 507, 7, 823, 678, 816, 187, 982, 14, 181, 332, 345, 820, 26, 654, 653, 344, 490, 22, 185, 991, 351, 981, 990, 333, 3, 165, 352, 668, 176, 23, 817, 347, 499, 491, 492, 25, 17, 444, 826, 336, 980, 297, 74, 1001, 191, 151, 828, 12, 520, 671, 179, 54, 652, 1075, 483, 512, 821, 77, 171, 346, 75, 183, 16, 190, 186, 675, 339, 1038, 502, 340, 822, 166, 658, 666, 818, 655, 819, 330, 1073, 169, 1037, 326, 328, 167, 495, 494], 7232223228056910425, 10503878646336740862)

    //     (0.2637132755109184, [979, 493, 329, 163, 172, 496, 815, 657, 168, 1, 656, 2, 653, 490, 654, 7, 816, 982, 332, 820, 981, 333, 3, 165, 6, 5, 978, 817, 4, 489, 492, 491, 984, 331, 164, 327, 980, 186, 483, 54], 5305305275735044714, 15547215823999170105)
    // (0.2637132755109185, [54, 483, 186, 980, 327, 164, 331, 984, 491, 492, 489, 4, 817, 9\78, 5, 6, 165, 3, 333, 981, 820, 332, 982, 816, 7, 654, 490, 653, 2, 656, 1, 168, 657, 815, 496, 172, 163, 329, 493, 979], 10509564102243258223, 3134537306968620002)

    //(0.16999101986218318, [494, 495, 167, 328, 326, 1037, 169, 1073, 330, 819, 655, 818, 666, 658, 166, 74, 1001, 822, 191, 151, 828, 502, 340, 1038, 12, 339, 675, 190, 186, 520, 16, 671, 54, 652, 171, 483, 1075, 821, 512, 179, 183, 75, 346, 77, 489, 25, 492, 491, 501, 17, 444, 826, 349, 331, 995, 347, 499, 984, 164, 11, 1003, 674, 334, 343, 985, 500, 825, 680, 504, 327, 511, 350, 840, 297, 980, 336, 670, 20, 660, 510, 999, 662, 8, 837, 979, 493, 509, 329, 505, 168, 832, 661, 829, 657, 663, 508, 986, 1, 9, 184, 667, 172, 182, 496, 19, 815, 656, 2, 173, 673, 163, 351, 990, 333, 3, 174, 4, 817, 23, 176, 668, 352, 978, 5, 991, 26, 820, 332, 181, 982, 14, 345, 654, 653, 678, 187, 816, 823, 7, 507, 344, 490, 665, 676, 22, 185, 981, 165, 988, 6], 4506950566254653576, 5238917583190544038)

    // let mut sol: SimAnn = SimAnn::new(
    //     0.002,
    //     800000.0,
    //     0.95,
    //     &Cases::new().l150,
    //     2000,
    //     5238917583190544038,
    //     4506950566254653576,
    // );
    // sol.threshold_acceptance();

    //(0.15018466635146888, [668, 176, 23, 352, 978, 5, 6, 988, 165, 3, 981, 990, 333, 991, 351, 185, 22, 676, 163, 172, 182, 173, 673, 665, 344, 653, 490, 654, 26, 820, 345, 332, 181, 14, 982, 187, 816, 678, 823, 7, 507, 2, 656, 667, 184, 815, 986, 508, 657, 663, 832, 661, 829, 1, 9, 168, 505, 19, 496, 329, 509, 493, 979, 837, 8, 662, 999, 343, 510, 660, 20, 680, 825, 500, 985, 504, 674, 334, 1003, 349, 331, 995, 984, 501, 11, 164, 17, 25, 492, 491, 499, 347, 817, 174, 4, 489, 77, 183, 346, 171, 54, 652, 1075, 483, 512, 821, 75, 179, 671, 16, 520, 186, 190, 675, 339, 502, 340, 1038, 12, 828, 151, 191, 444, 826, 511, 327, 670, 350, 840, 336, 980, 297, 1001, 74, 822, 166, 658, 666, 818, 655, 819, 330, 1073, 169, 1037, 326, 328, 167, 495, 494], 16750126562427265336, 16991197142420910314)

    //best solution so far 150 cities
    //     IT:221376
    // S:0.14942416236838876320
    // P:[668, 176, 23, 352, 978, 5, 6, 988, 165, 3, 981, 990, 333, 991, 351, 185, 22, 676, 665, 163, 172, 496, 182, 673, 653, 344, 490, 654, 26, 820, 345, 332, 181, 14, 982, 187, 816, 678, 823, 7, 507, 656, 667, 184, 815, 173, 2, 9, 832, 663, 657, 661, 829, 1, 986, 508, 168, 505, 19, 329, 509, 493, 979, 837, 8, 662, 999, 343, 510, 660, 20, 680, 825, 500, 985, 504, 674, 334, 1003, 349, 331, 995, 984, 501, 11, 164, 17, 25, 492, 491, 499, 347, 817, 174, 4, 489, 77, 183, 346, 171, 54, 652, 1075, 483, 512, 821, 75, 179, 671, 16, 520, 186, 190, 675, 339, 502, 340, 1038, 12, 828, 151, 191, 444, 826, 511, 327, 670, 350, 840, 336, 980, 297, 1001, 74, 822, 166, 658, 666, 818, 655, 819, 330, 1073, 169, 1037, 326, 328, 167, 495, 494]
    // N:16750126562427265336
    // I:16991197142420910314

    // real	0m6.120s
    // user	0m6.069s
    // sys	0m0.095s

    // let mut sol: SimAnn = SimAnn::new(
    //     0.0001,
    //     800000.0,
    //     0.98,
    //     &Cases::new().l150,
    //     1000,
    //     16991197142420910314,
    //     16750126562427265336,
    // );
    // sol.threshold_acceptance();

    //     AP:0.7604166666666666
    // (0.14919736414181115, [494, 495, 167, 328, 326, 1037, 169, 1073, 330, 819, 655, 818, 666, 658, 166, 74, 1001, 297, 980, 336, 826, 444, 17, 492, 491, 499, 347, 817, 4, 174, 23, 176, 668, 352, 978, 5, 6, 988, 165, 3, 981, 990, 333, 991, 351, 185, 22, 676, 163, 172, 182, 496, 19, 505, 815, 184, 667, 173, 673, 665, 344, 653, 490, 654, 26, 820, 345, 332, 181, 14, 982, 187, 816, 678, 823, 7, 507, 656, 2, 9, 1, 829, 661, 832, 663, 657, 986, 508, 168, 329, 509, 493, 979, 837, 331, 662, 8, 999, 343, 510, 660, 20, 680, 670, 840, 350, 511, 327, 504, 825, 500, 985, 674, 334, 1003, 349, 995, 984, 11, 164, 501, 25, 489, 77, 183, 346, 75, 821, 512, 483, 1075, 652, 54, 171, 179, 671, 16, 520, 186, 190, 675, 339, 502, 340, 1038, 12, 828, 151, 191, 822], 9105696943009919638, 14468490169328286363)

    //   let mut sol: SimAnn = SimAnn::new(
    //     0.0001,
    //     800000.0,
    //     0.98,
    //     &Cases::new().l150,
    //     1000,
    //     14468490169328286363,
    //     9105696943009919638,
    // );
    // sol.threshold_acceptance();

    //found best
    //(0.1489909750316175, [343, 985, 500, 825, 680, 20, 660, 510, 980, 297, 336, 840, 350, 670, 327, 511, 504, 674, 334, 999, 8, 662, 331, 995, 837, 979, 493, 509, 329, 19, 505, 168, 9, 1, 829, 661, 832, 663, 657, 508, 986, 815, 184, 667, 656, 2, 173, 673, 182, 496, 172, 163, 665, 676, 820, 26, 654, 490, 344, 653, 507, 7, 823, 678, 816, 187, 982, 14, 181, 345, 332, 991, 185, 22, 351, 981, 990, 333, 3, 165, 4, 174, 988, 6, 5, 978, 352, 668, 176, 23, 817, 489, 347, 499, 491, 492, 25, 501, 984, 349, 1003, 11, 164, 17, 444, 826, 339, 16, 671, 179, 512, 821, 1075, 483, 652, 54, 171, 77, 346, 75, 183, 190, 186, 520, 675, 340, 502, 1038, 12, 828, 151, 191, 822, 1001, 74, 166, 658, 666, 818, 655, 819, 330, 1073, 169, 1037, 326, 328, 167, 495, 494], 2905730760004994292, 11025063659276534009)
    //     let mut sol: SimAnn = SimAnn::new(
    //     0.0001,
    //     800000.0,
    //     0.98,
    //     &Cases::new().l150,
    //     1000,
    //     11025063659276534009,
    //     2905730760004994292,
    // );
    // sol.threshold_acceptance();

    //found new best
    //(0.14635922095400786, [494, 495, 167, 328, 326, 1037, 169, 1073, 330, 819, 655, 818, 666, 658, 166, 74, 1001, 822, 191, 151, 828, 12, 1038, 502, 340, 675, 339, 826, 444, 17, 501, 11, 164, 334, 999, 8, 662, 331, 995, 837, 979, 493, 509, 329, 182, 496, 19, 505, 168, 508, 986, 657, 663, 832, 661, 829, 1, 9, 2, 656, 667, 184, 173, 815, 172, 163, 673, 665, 676, 344, 490, 654, 653, 507, 7, 823, 678, 816, 187, 982, 14, 181, 332, 345, 820, 26, 991, 185, 22, 351, 981, 990, 333, 3, 165, 988, 6, 5, 978, 352, 668, 176, 23, 174, 4, 817, 347, 499, 489, 25, 492, 491, 984, 349, 1003, 674, 985, 343, 510, 660, 20, 680, 825, 500, 504, 511, 327, 670, 350, 840, 336, 980, 297, 190, 186, 520, 16, 671, 179, 512, 821, 75, 346, 183, 77, 171, 483, 1075, 652, 54], 11048797390406085514, 5826181057563262003)
    // change batch average to  let mut batch_average: f64 = 1000000000.0;
    //  let mut sol: SimAnn = SimAnn::new(
    //     0.0001,
    //     800000.0,
    //     0.98,
    //     &Cases::new().l150,
    //     1000,
    //     5826181057563262003,
    //     11048797390406085514,
    // );
    // sol.threshold_acceptance();

    //even better solution c150
    //(0.14418250861220022, [54, 652, 1075, 483, 171, 77, 183, 346, 75, 821, 512, 179, 671, 16, 520, 186, 190, 675, 340, 502, 151, 828, 12, 1038, 339, 826, 444, 17, 164, 11, 501, 25, 492, 491, 499, 347, 489, 4, 174, 817, 23, 176, 668, 352, 978, 5, 6, 988, 165, 3, 981, 990, 333, 991, 351, 185, 22, 676, 665, 173, 2, 656, 184, 815, 172, 182, 496, 19, 505, 168, 508, 986, 9, 1, 829, 657, 663, 661, 832, 667, 507, 653, 344, 490, 654, 26, 820, 345, 332, 181, 14, 982, 187, 816, 678, 823, 7, 673, 163, 329, 509, 493, 979, 837, 995, 984, 1003, 349, 331, 662, 8, 999, 674, 334, 343, 510, 660, 20, 680, 825, 500, 985, 504, 511, 327, 670, 350, 840, 336, 297, 980, 191, 822, 1001, 74, 166, 658, 666, 818, 655, 819, 330, 1073, 169, 1037, 326, 328, 167, 495, 494], 11613177925935108993, 8480295797754924014)
    //             let mut sol: SimAnn = SimAnn::new(
    //     0.0001,
    //     800000.0,
    //     0.98,
    //     &Cases::new().l150,
    //     1000,
    //     8480295797754924014,
    //     11613177925935108993,
    // );
    // sol.threshold_acceptance();

    //c150 : 0.149078160
    //c40 : 0.263713270
    Ok(())
}
