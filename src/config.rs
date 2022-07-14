use std::rc::Rc;

use simulator::Simulation;

use crate::models::{consumer, producer};

pub fn set_simulation() -> Simulation<()> {
    let mut simulation = Simulation::default();

    let shared_state = simulation.state();
    let mut state = shared_state.take();

    let count_key = state.insert(0);
    let producer_key = state.insert(None);
    let consumer_key = state.insert(None);

    //[0 for Producer, 1 for consumer];
    let passivate_list: [Passivated; 2] = [Passivated::False; 2];
    let passivate_list = state.insert(passivate_list);

    let clock = simulation.clock();
    
    let producer = simulation.add_generator_fn(|co| {
        producer(
            co,
            Rc::clone(&shared_state),
            count_key,
            consumer_key,
            passivate_list,
            clock,
        )
    });
    
    let clock = simulation.clock();
    let consumer = simulation.add_generator_fn(|co| {
        consumer(
            co,
            Rc::clone(&shared_state),
            count_key,
            producer_key,
            passivate_list,
            clock,
        )
    });

    *state.get_mut(producer_key).unwrap() = Some(producer);
    *state.get_mut(consumer_key).unwrap() = Some(consumer);

    shared_state.set(state);

    simulation.schedule_now(producer);
    simulation.schedule_now(consumer);

    simulation
}

#[derive(PartialEq, Clone, Copy)]
pub enum Passivated {
    True,
    Warned,
    False,
}

pub fn test_config() -> (Simulation<()>, simulator::StateKey<u32>) {
   let mut simulation = Simulation::default();

    let shared_state = simulation.state();
    let mut state = shared_state.take();

    let count_key = state.insert(0);
    let producer_key = state.insert(None);
    let consumer_key = state.insert(None);

    //[0 for Producer, 1 for consumer];
    let passivate_list: [Passivated; 2] = [Passivated::False; 2];
    let passivate_list = state.insert(passivate_list);
    
    let clock = simulation.clock();

    let producer = simulation.add_generator_fn(|co| {
        producer(
            co,
            Rc::clone(&shared_state),
            count_key,
            consumer_key,
            passivate_list,
            clock,
        )
    });
    
    let clock = simulation.clock();

    let consumer = simulation.add_generator_fn(|co| {
        consumer(
            co,
            Rc::clone(&shared_state),
            count_key,
            producer_key,
            passivate_list,
            clock,
        )
    });

    *state.get_mut(producer_key).unwrap() = Some(producer);
    *state.get_mut(consumer_key).unwrap() = Some(consumer);

    shared_state.set(state);

    simulation.schedule_now(producer);
    simulation.schedule_now(consumer);

    (simulation, count_key)
}
