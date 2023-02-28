use simulated_annealing::sa::SimAnn;
use simulated_annealing::reader::Reader;
use simulated_annealing::testCases::Cases;
   


use float_cmp::ApproxEq;
#[cfg(test)]
mod tests{
    use sqlite::Connection;
    use simulated_annealing::city::City;
    use float_cmp::approx_eq;

    use super::*;
    
    

    #[test]
    fn test_nat_distance(){
        let query = r#"SELECT * FROM connections ORDER BY RANDOM() LIMIT 15;"#;
        let reader : Reader = Reader::new(Cases::new().l40, "db/citiesDB.db");
        let all_cities : Vec<City> = reader.read_cities();
        let all_connections : Vec<Vec<f64>> = reader.read_connections();
        let sa : SimAnn = SimAnn::new(Cases::new().l40.len().try_into().unwrap(), Cases::new().l40);

        let connection  = Connection::open("db/citiesDB.db").unwrap();
        for row in 
            connection
            .prepare(query)
            .unwrap()
            .into_iter()
            .map(|row| row.unwrap())
        {
            let city1 = row.read::<i64, _>("id_city_1");
            let city2 = row.read::<i64, _>("id_city_2");
            let dist = row.read::<f64, _>("distance");
            
            let nat_dist : f64 = sa.get_nat_distance(all_cities[city1 as usize -1], all_cities[city2 as usize -1]);
            dbg!( dist , nat_dist); 
            assert!( approx_eq!(f64, dist, nat_dist, epsilon = 0.01, ulps = 2) );
        }
    }


    #[test]
    fn test_cost_function(){
        let case : Cases = Cases::new();
        //        let reader : Reader = Reader::new( "")
        let sa40 : SimAnn = SimAnn::new(case.l40.len().try_into().unwrap(), case.l40);
        let cost40 : f64 = 4129508.339517763;
        let md40 : f64 = 4947749.059999999590218;
        let norm40 : f64 = 180088219.480000019073486;

        let sa150 : SimAnn = SimAnn::new(case.l150.len().try_into().unwrap(), case.l150);
        let cost150 : f64 = 6249022.603226478;
        let md150 : f64 = 4979370.000000000000000000;
        let norm150 : f64 = 721914154.580000042915344;
        //let new_cost150 : f64 = 

        
        //assert!( approx_eq!(f64, a, b, epsilon = 0.00000003) );
    }
    
}
