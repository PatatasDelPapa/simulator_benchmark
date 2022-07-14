use std::time::Duration;

mod config;
mod models;

pub fn simulation(limit: u64) {
    let mut simulation = config::set_simulation();
    let limit = Duration::from_secs(limit);

    simulation.run_with_limit(limit);
}

pub fn test_simulation() -> (simulator::Simulation<()>, simulator::StateKey<u32>) {
    config::test_config()
}