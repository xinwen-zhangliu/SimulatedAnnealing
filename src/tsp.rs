use simulated_annealing::sa::SimAnn;


trait TA {
    fn calcula_lote() -> (f64, f64);
    fn threshold_acceptance();
}

struct Solution {
    cities: Vec<u16>,
    cost: f64,
    epsilon : f64,
    phi : f64,
    temp : f64,
    sa : SimAnn,
  
}

impl Solution {
    pub fn new(epsilon: f64, initial_temp: f64, phi: f64, cities: &Vec<u16>) -> Self {
        Self {
            cities, cost : 0.0, epsilon, phi, temp :  initial_temp,
            sa : SimAnn::new(cities.len(), cities)
            
            
        }
    }

    fn calculate_batch(&mut self, temp: f64, solution: &Vec<u16>) -> f64 {
        let mut counter = 0;
        let mut sum: f64 = 0.0;
        let batch = 5;
        let mut r = 0.0;
        while counter < batch {
            let s2 =  sa.get_neighbor(self.cities);
            if sa.get_cost(s2) <= (sa.get_cost(cities) + self.temp){
                cities = s2;
                counter+=1;
                r+= sa.get_cost(s2);
            }
        }
        r/batch
    }

    fn threshold_acceptance() {

        
        let batch_average: f64 = 0.0;
        //let epsilon = 0.0; //change later
        while self.temp > self.epsilon {
            let mut q: f64 = f64::INFINITY;
            while batch_average < q {
                q = batch_average;
                batch_average  = self.calculate_batch(self.temp, self.cities);
            }
            
            temp = self.phi * temp;
        }
    }
}
