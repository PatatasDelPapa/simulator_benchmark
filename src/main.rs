use std::time::Duration;

use simulator_benchmark::test_simulation;
use simulator_benchmark::simulation;

fn main() {
    simulation(50000);
}

#[allow(dead_code)]
fn testing_simulator() {
    let (mut simulation, count_key, shared_state) = test_simulation();    
    let limit = Duration::from_secs(1000);
    simulation.run_with_limit(limit);
    
    // let shared_state = simulation.state();
    let mut state = shared_state.take();
    // dbg!(&state);
    let count = state.remove(count_key).unwrap();
    println!("Final count = {}", count);
    println!("Final time = {:?}", simulation.time());
}