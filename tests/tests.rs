use simulated_annealing::reader::Reader;
use simulated_annealing::path::Path;
use simulated_annealing::testCases::Cases;

// running with command line output
// cargo test -- --no-capture

//#[crate_name = "tests"]
#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use float_cmp::approx_eq;
    use simulated_annealing::City;
    use sqlite::Connection;

    use super::*;

    #[test]
    /// Tests the function get_nat_distance in Path against the distances in the database, byt getting a random selection of 25 pairs.
    fn test_nat_distance() {
        //we get 25 random pairs of cities and compare it witht he result from calculating the natural distance
        let query = r#"SELECT * FROM connections ORDER BY RANDOM() LIMIT 25;"#;
        let reader: Reader = Reader::new("db/citiesDB.db");
        let all_cities: Vec<City> = reader.read_cities();
        let path: Path = Path::new(
            Cases::new().l40.len().try_into().unwrap(),
            &Cases::new().l40,
            123,
        );

        let connection = Connection::open("db/citiesDB.db").unwrap();
        for row in connection
            .prepare(query)
            .unwrap()
            .into_iter()
            .map(|row| row.unwrap())
        {
            let city1 = row.read::<i64, _>("id_city_1");
            let city2 = row.read::<i64, _>("id_city_2");
            let dist = row.read::<f64, _>("distance");

            let nat_dist: f64 = path.get_nat_distance(
                all_cities[city1 as usize - 1],
                all_cities[city2 as usize - 1],
            );
            assert!(approx_eq!(f64, dist, nat_dist, epsilon = 0.01, ulps = 2));
        }
    }

    #[test]
    /// Tests the cost function, maximum distance and normalizer of the cases with 40 and 150 cities, against predefined values.
    fn test_cost_function() {
        let case: Cases = Cases::new();
        let mut results: Vec<f64> = Vec::new();
        let mut path40: Path = Path::new(case.l40.len().try_into().unwrap(), &case.l40,123);
        let mut path150: Path = Path::new(case.l150.len().try_into().unwrap(), &case.l150, 123);

        path40.prepare();
        path150.prepare();
        path40.add_initial_distance();
        path150.add_initial_distance();
        results.push(path40.get_cost());
        results.push(path40.get_max_distance());
        results.push(path40.get_normalizer());
        results.push(path150.get_cost());
        results.push(path150.get_max_distance());
        results.push(path150.get_normalizer());

        //cost max_distance normalizer
        //first for the case with 40 cities then for the case with 150 cities
        let cases: Vec<f64> = vec![
            4129508.339517763,
            4947749.059999999590218,
            180088219.480000019073486,
            6249022.603226478,
            4979370.000000000000000000,
            721914154.580000042915344,
        ];

        for i in 0..6 {
            //println!("{} = {} ", cases[i], results[i]);
            assert!(approx_eq!(f64, cases[i], results[i], epsilon = 0.000001));
        }
    }

    #[test]
    fn test_sum_of_distances() {
        let mut case: Cases = Cases::new();
        let mut path: Path = Path::new(case.l40.len().try_into().unwrap(), &case.l40, 123);
        path.prepare();

        //let mut initial_solution = path.get_initial_solution(&mut case.l40, 8);
        path.add_initial_distance();
        for i in 1..100 {
            println!("{}", path.get_sum_of_distances());
            path.get_neighbor(&mut case.l40[..]);
            let updated_sum = path.get_sum_of_distances();
            let linear_sum = path.add_dist(&mut case.l40);
            println!(
                "{:?} \n update:{:.20} = linear:{:.201}",
                case.l40, updated_sum, linear_sum
            );
            println!("{}", i);
            assert!(approx_eq!(f64, updated_sum, linear_sum, epsilon = 1.0));
        }
    }

    #[test]
    fn test_sum_swap_unswap() {
        let mut case: Cases = Cases::new();
        let mut path: Path = Path::new(case.l40.len().try_into().unwrap(), &case.l40, 123);
        path.prepare();

        for _i in 1..50 {
            let mut cities = path.get_initial_solution(&mut case.l40, 7);
            path.add_initial_distance();
            let before_swap = path.get_cost();
            path.get_neighbor(&mut cities);
            path.undo(&mut cities);
            let after_undo = path.get_cost();

            assert!(approx_eq!(
                f64,
                before_swap,
                after_undo,
                epsilon = 0.0000001
            ));
        }
    }
}
