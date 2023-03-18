pub mod reader;
#[allow(non_snake_case)]
pub mod testCases;
pub mod path;
pub mod sa;
pub mod threadspawninator;


#[derive(Debug, Copy, Clone)]
/// Struct that represents a city
pub struct City{
    pub id : i64 ,
    pub lat : f64,
    pub long : f64,
}
