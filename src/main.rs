use std::time::Duration;

use simulator_benchmark::test_simulation;

fn main() {
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