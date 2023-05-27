use std::{sync::{Arc, Mutex}, thread, time::{Instant, Duration}};




const LIGHT_SPEED:f64 = 299_792_458.0;

fn main() {

    let masses:Vec<f64> = vec![0.001,0.002,0.003,0.004,0.005,0.006,0.007,0.008,0.009,0.011,0.012,0.013];
    
    let results = Arc::new(Mutex::new(Vec::new()));
    let start_time = Instant::now();

    para_fn(masses, &results);
    //iter_fn(masses, &results);

    let end_time = Instant::now();
    let execution_time = end_time - start_time;
    let results = results.lock().unwrap();

    // 결과 출력
    println!("energy: {:?} joules", *results);
    println!("Execution Time: {:?}", execution_time);

}

fn para_fn(masses: Vec<f64>, results: &Arc<Mutex<Vec<f64>>>) {
    let handles: Vec<_> = masses.into_iter().map(|number| {
        let results = Arc::clone(results);
        thread::spawn(move || {
            let result = mass_to_energy(number);
            let mut results = results.lock().unwrap();
            results.push(result);
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }
}

fn iter_fn(masses: Vec<f64>, results: &Arc<Mutex<Vec<f64>>>) {
    let _handles: Vec<_> = masses.into_iter().map(|number| {
        let results = Arc::clone(results);
            let result = mass_to_energy(number);
            let mut results = results.lock().unwrap();
            results.push(result);
    }).collect();
}



fn mass_to_energy(mass: f64) -> f64 {
    let energy:f64 = mass * (LIGHT_SPEED * LIGHT_SPEED);
    //calculate time
    thread::sleep(Duration::from_millis(200));
    energy
}

fn energy_to_mass(energy: f64) -> f64{
    let mass:f64 = energy / (LIGHT_SPEED * LIGHT_SPEED);
    mass
}