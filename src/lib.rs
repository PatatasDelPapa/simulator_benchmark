use std::time::Duration;
use std::rc::Rc;
use std::cell::Cell;

mod config;
mod models;

use simulator::State;

pub fn simulation(limit: u64) {
    let mut simulation = config::set_simulation();
    let limit = Duration::from_secs(limit);

    simulation.run_with_limit(limit);
}

pub fn test_simulation() -> (simulator::Simulation<()>, simulator::StateKey<u32>, Rc<Cell<State>>) {
    config::test_config()
}