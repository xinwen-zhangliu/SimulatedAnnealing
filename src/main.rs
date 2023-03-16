use simulated_annealing::path::Path;
use simulated_annealing::sa::SimAnn;
use simulated_annealing::testCases::Cases;
use simulated_annealing::threadspawninator::TSI;
use sqlite::Result;
use std::env;

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

    // let db_path = std::env::var_os("CARGO_MANIFEST_DIR").unwrap();
    //     dbg!(&db_path);
    //     let path_str = db_path.into_string().unwrap() + "/db/citiesDB.db";

    // dbg!(path_str);

    // let mut thread = TSI::new();
    // thread.spawn_threads(200);

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
    // (0.2637132755109185, [54, 483, 186, 980, 327, 164, 331, 984, 491, 492, 489, 4, 817, 978, 5, 6, 165, 3, 333, 981, 820, 332, 982, 816, 7, 654, 490, 653, 2, 656, 1, 168, 657, 815, 496, 172, 163, 329, 493, 979], 10509564102243258223, 3134537306968620002)

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

    let mut sol: SimAnn = SimAnn::new(
        0.0001,
        800000.0,
        0.98,
        &Cases::new().l150,
        1000,
        16991197142420910314,
        16750126562427265336,
    );
    sol.threshold_acceptance();

    //c150 : 0.149078160
    //c40 : 0.263713270
    Ok(())
}
