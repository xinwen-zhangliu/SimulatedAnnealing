use crate::city::City;
use crate::reader::Reader;
use crate ::testCases::Cases;

pub struct SimAnn {
    temperature : f64,
    initial_solution : Vec<u16>,
    num_of_cities : u16,
    n1 : u16,
    n2 : u16, 
    
    sum_of_distances : f64,
    norm : f64,
    
    all_cities : Vec<City>,
    all_connections : Vec<Vec<f64>>,
    
}



impl SimAnn{
    
    pub fn new(
        num : u16,
        list_of_cities : Vec<u16>,
    )->Self{
        Self{
            initial_solution : list_of_cities,
            temperature : 0.0,
            num_of_cities : num,
            n1 : 0,
            n2 : 0,
            sum_of_distances : 0.0,
            norm : 0.0,
            all_cities : Vec::new(),
            all_connections : vec![vec![0.0f64; 1092]; 1092],
        }
    }
    
    
    pub fn prepare(){
        //read database and get all the cities and connections
        let case : Cases = Cases::new();
        let reader  : Reader = Reader::new(case.l40, "db/citiesDB.db");
        let mut arr  : Vec<f64> = Vec::new();
        

        arr = reader.get_distances_ordered();
        
        
    }

  

    fn normalizer(&self ,arr : &[f64]){
        //get the list of distances from high-to-low
        
        //L if L.length < num_of_cities-1

        //else take the first num_of_cities-1 distances

        //add everything
    }

    fn initial_solution(&self){
        
    }

}
