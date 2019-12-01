use std::fs;
use std::io::{prelude::*, BufReader};

fn main() { 
    let file = fs::File::open("test_input.txt")
        .expect("input file not found");
    let reader = BufReader::new(file);

    let mut fuel_requirements: Vec<f64> = vec![];
    
    for line in reader.lines() {
        let mass = &line.unwrap().parse::<f64>()
            .expect("Could not parse input");
        
        fuel_requirements.push(fuel_required(mass))
    }

    println!("Required fuel: {}", fuel_requirements.iter().sum::<f64>())
}

fn fuel_required(mass: &f64) -> f64 {
   let result = (mass / 3.0).floor() - 2.0;

   if result < 0.0 {
       return 0.0 
   } else {
       return result + fuel_required(&result);
   }
}
