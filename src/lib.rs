pub mod reader;
pub mod testCases;
pub mod sa;
pub mod tsp;
pub mod threadspawninator;


#[derive(Debug, Copy, Clone)]
pub struct City{
    pub id : i64 ,
    pub lat : f64,
    pub long : f64,
}
