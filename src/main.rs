use sqlite::Result;
use std::env;
use std::path::Path;

use simulated_annealing::sa::SimAnn;
use simulated_annealing::testCases::Cases;
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

         let db_path = std::env::var_os("CARGO_MANIFEST_DIR").unwrap();
        dbg!(&db_path);
        let path_str = db_path.into_string().unwrap() + "/db/citiesDB.db";

    dbg!(path_str);
    let mut thread = TSI::new();
    thread.spawn_threads(40);

    // let mut sol: Solution = Solution::new(
    //                 0.002,
    //                 800000.0,
    //                 0.95,
    //                 &Cases::new().l40,
    //                 2000,
    //                 13603085585784548072,
    //                 11058656509428391188,
    // );

    // let mut sol: Solution = Solution::new(
    //             0.002,
    //             800000.0,
    //             0.95,
    //             &Cases::new().l150,
    //             2000,
    //             3370200950440291139,
    //            5892369532929239771,
    //         );

    //11058656509428391188, 13603085585784548072
    //(0.2617308114181899, [328, 167, 495, 494, 326, 169, 1037, 330, 818, 655, 666, 658, 1073, 819, 166, 822, 336, 343, 510, 825, 680, 1003, 492, 25, 817, 668, 978, 176, 988, 5, 165, 352, 351, 185, 22, 490, 26, 344, 507, 182, 667, 173, 184, 168, 663, 505, 986, 329, 2, 172, 496, 665, 163, 19, 673, 653, 654, 676, 820, 345, 181, 982, 14, 332, 187, 678, 816, 823, 7, 832, 661, 657, 9, 1, 829, 508, 656, 815, 991, 333, 3, 981, 990, 4, 174, 6, 23, 509, 493, 979, 499, 347, 331, 662, 349, 501, 17, 11, 339, 75, 346, 183, 512, 821, 16, 675, 340, 828, 12, 502, 190, 186, 151, 1038, 520, 191, 74, 1001, 297, 980, 670, 500, 350, 444, 164, 8, 999, 334, 985, 826, 504, 327, 674, 995, 837, 511, 840, 660, 20, 984, 491, 489, 77, 671, 179, 54, 652, 1075, 483, 171], 5892369532929239771, 3370200950440291139),

    //5892369532929239771, 3370200950440291139),

    // let mut sol: Solution = Solution::new(
    //                0.002,
    //                14.0,
    //                0.95,
    //                &Cases::new().l40,
    //                700,
    //               42,
    //                7,
    //            );

    //ran with seed = 7 in sa
    
    // let mut sol: Solution = Solution::new(
    //     0.002,
    //     2000000.0,
    //     0.95,
    //     &Cases::new().l150,
    //     5000,
    //     17132881300615463402,
    //     16071864923883299597,
    // );
    // sol.threshold_acceptance();

    Ok(())
}
