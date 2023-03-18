use crate::City;


use sqlite::Connection;

/// Struct for accessing and reading the database of cities
pub struct Reader {
    connection: Connection,
}


impl Reader {
    /// Constructor
    pub fn new(path: &str) -> Reader {
        Reader {
            connection: Connection::open(path).unwrap(),
        }
    }

    /// From the vector of cities that is passes the function will return the distances
    /// between each pair in an ordered vector from highest to lowest.
    pub fn get_distances_ordered(&self, cities: &Vec<usize>) -> Vec<f64> {
        let begin: &str = "(";
        let end: &str = ")";
        let body = cities
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",");

        let list = format!("{}{}{}", begin, body, end);


        let query = "SELECT distance FROM connections WHERE id_city_1 IN ".to_owned()
            + &list
            + &" AND  id_city_2 IN ".to_owned()
            + &list
            + &" ORDER BY distance DESC;".to_owned();

        let mut distances: Vec<f64> = Vec::new();
        for row in self
            .connection
            .prepare(query)
            .unwrap()
            .into_iter()
            .map(|row| row.unwrap())
        {
            let distance = row.read::<f64, _>("distance");
            distances.push(distance);
        }

        distances
    }

    /// Gets all the cities form that database, maps them to struct City and returns
    /// a vector with each of them.
    pub fn read_cities(&self) -> Vec<City> {
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

    /// Gets all the connections from the database and saves the distances in a 2D vector
    pub fn read_connections(&self) -> Vec<Vec<f64>> {
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
            all_connections[c1  - 1][c2 - 1] = distance;
            all_connections[c2 - 1][c1 - 1] = distance;
        }
        all_connections
    }
}
