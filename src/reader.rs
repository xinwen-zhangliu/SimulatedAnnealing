use crate::city::City;

use sqlite::{Connection,  Value};

pub struct Reader{
    cities: Vec<u16>,
    connection: Connection,
    //distances: Option<&'a mut [f64]>,
}



impl Reader {

    fn open_connection(path : &str)-> Connection{
        Connection::open(path).unwrap()
    }
    
    pub fn new(list: Vec<u16>, path : &str) -> Reader {
        Reader {
            cities: list,
            connection : Self::open_connection(path),
        }
    }

    fn check_connection(){
        //if connection is not open then panic
    }
    
    fn close_connection(&self) {}

    pub fn get_distances_ordered(&self) -> Vec<f64> {
        
        //convert tthe vector of cities to string in this format (a,b,....,n)
        let begin : &str = "(";
        let end : &str = ")";
        let body =  self.cities.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",") ;
        
        let list = format!("{}{}{}", begin, body, end);
        let list2 = list.clone();
        
        println!("query with binding : {} " , list);
        // let query =concat!( "{}{}{}", "SELECT distance FROM connections WHERE id_city_1 IN "
        //                                , list ," OR WHERE id_city_2 IN "
        //                                , list, " ORDER BY distance DESC");
        let query = "SELECT distance FROM connections WHERE id_city_1 IN ".to_owned()
                                       //+ &list + &" AND  WHERE id_city_2 IN ".to_owned()
                                       + &list+ &" ORDER BY distance DESC;".to_owned();
       
        let mut distances : Vec<f64>  = Vec::new();
        for row in self
            .connection
            .prepare(query)
            .unwrap()
            .into_iter()
        //.bind((":cities", list.as_str()))
            // .bind::<&[(_, Value)]>(&[
            //     (":cities1", list.into()),
            //     (":cities2", list2.into()),
            // ][..])
            // .unwrap()
            .map(|row| row.unwrap()){
                let distance = row.read::<f64, _>("distance");
                distances.push(distance);
            }
//        .bind((":cities", listOfCities));
        distances
    }

    fn read_cities(&self) -> Vec<City> {
        let mut all_cities: Vec<City> = Vec::new();
        for row in self
            .connection
            .prepare("SELECT * FROM cities;")
            .unwrap()
            .into_iter()
            .map(|row| row.unwrap())
        {
            let city = City {
                id: row.read::<i64, _>("id"),
                lat: row.read::<f64, _>("latitude"),
                long: row.read::<f64, _>("longitude"),
            };
            all_cities.push(city);
        }
        all_cities
    }

    fn read_connections(&self) -> Vec<Vec<f64>> {
        let query = "SELECT * FROM connections;";
        let mut all_connections = vec![vec![0.0f64; 1092]; 1092];
        for row in self
            .connection
            .prepare(query)
            .unwrap()
            .into_iter()
            .map(|row| row.unwrap())
        {
            let city1 = row.read::<i64, _>("id_city_1");
            let city2 = row.read::<i64, _>("id_city_2");
            let distance = row.read::<f64, _>("distance");
            let c1 = usize::try_from(city1).unwrap();
            let c2 = usize::try_from(city2).unwrap();
            all_connections[c1 - 1][c2 - 1] = distance;
        }
        all_connections
    }
}
