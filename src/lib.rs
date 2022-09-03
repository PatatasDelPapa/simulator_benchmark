use std::cell::Cell;
use std::rc::Rc;
use std::time::Duration;

use crate::state::{State, StateKey};

mod config;
mod models;
mod state;

pub fn simulation(limit: u64) {
    let mut simulation = config::set_simulation();
    let limit = Duration::from_secs(limit);

    simulation.run_with_limit(limit);
}

pub fn test_simulation() -> (simulator::Simulation<()>, StateKey<u32>, Rc<Cell<State>>) {
    config::test_config()
}
