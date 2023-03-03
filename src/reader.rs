use crate::city::City;

use sqlite::Connection;

pub struct Reader {
    connection: Connection,
}

impl Reader {
    fn open_connection(path: &str) -> Connection {
        Connection::open(path).unwrap()
    }

    pub fn new(path: &str) -> Reader {
        Reader {
            connection: Connection::open(path).unwrap(),
        }
    }

    pub fn get_distances_ordered(&self, cities: &Vec<u16>) -> Vec<f64> {
        let begin: &str = "(";
        let end: &str = ")";
        let body = cities
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",");

        let list = format!("{}{}{}", begin, body, end);

        println!("query with binding : {} ", list);
        let query = "SELECT distance FROM connections WHERE id_city_1 IN ".to_owned()
            + &list
            + &" AND  id_city_2 IN ".to_owned()
            + &list
            + &" ORDER BY distance DESC;".to_owned();

        //  let query = "SELECT distances FROM connections
        // WHERE id_city_1 IN :cities AND id_city_2 IN :cities ORDER BY DESC;";

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

    pub fn read_connections(&self) -> Vec<Vec<f64>> {
        let query = "SELECT * FROM connections;";
        dbg!("asdas");
        let mut all_connections = vec![vec![0.0f64; 1092]; 1092];
        dbg!("as");

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
