// Operations 

// trait Algs{
//     fn naturalDistance();
//     fn cost();
//     fn switcheroo(arr : &mut [u16]);
//     fn ctrlz();
//     fn thresholdAcceptance(temp : f64, solution : &mut[u16]);
// }



struct SimAnn for Algs{
    temperature : f64,
    solution : [u16],
    sumOfWeights : f64,
    n1 : u16,
    n2 : u16, 
    allConnections : [[u16; 1092]; 1092],
    allCities : [City, ]

    fn naturalDistance();
    
    
}

impl SimAnn{
  
    
    fn new(
        reader : 
    )->Self{
        Self{
            temperature : initialTemperature();
            
        }
    }
    

    pub fn run(){
    }

}
