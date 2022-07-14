use std::{cell::Cell, rc::Rc, time::Duration};

use crate::config::Passivated;
use simulator::{Key, State, StateKey, Yielder, scheduler::ClockRef};

pub async fn producer(
    co: Yielder<()>,
    shared_state: Rc<Cell<State>>,
    count_key: StateKey<u32>,
    consumer_key: StateKey<Option<Key>>,
    passivate_key: StateKey<[Passivated; 2]>,
    _clock: ClockRef,
) {
    // println!("Starting Producer");
    let hold_time = Duration::from_secs(2);
    let produce_ammount = 1;
    let thresh_hold = 15;

    let mut state = shared_state.take();
    let consumer_key = state.remove(consumer_key).flatten().unwrap();
    shared_state.set(state);

    loop {
        let mut state = shared_state.take();

        let passivated_list = state.get_mut(passivate_key).unwrap();
        if passivated_list[1] == Passivated::True {
            passivated_list[1] = Passivated::Warned;
            co.activate_one(consumer_key).await;
        }

        let count = state.get_mut(count_key).unwrap();
        if *count < thresh_hold {
            *count += produce_ammount;
            // println!("PRODUCED - Before: {} | After: {} | At: {:?}", *count - produce_ammount, count, clock.time());
            shared_state.set(state);
            co.hold(hold_time).await;
        } else {

            let passivate_list = state.get_mut(passivate_key).unwrap();
            passivate_list[0] = Passivated::True;

            shared_state.set(state);
            co.passivate().await;

            let mut state = shared_state.take();
            let passivate_list = state.get_mut(passivate_key).unwrap();
            passivate_list[0] = Passivated::False;
            shared_state.set(state);
        }
    }
}

pub async fn consumer(
    co: Yielder<()>,
    shared_state: Rc<Cell<State>>,
    count_key: StateKey<u32>,
    producer_key: StateKey<Option<Key>>,
    passivate_key: StateKey<[Passivated; 2]>,
    _clock: ClockRef,
) {
    // println!("Starting Consumer"); 
    let hold_time = Duration::from_secs(8);
    let consume_ammount = 8;

    let mut state = shared_state.take();
    let producer_key = state.remove(producer_key).flatten().unwrap();
    shared_state.set(state);

    loop {
        let mut state = shared_state.take();

        let passivated_list = state.get_mut(passivate_key).unwrap();
        if passivated_list[0] == Passivated::True {
            passivated_list[0] = Passivated::Warned;
            co.activate_one(producer_key).await;
        }

        let count = state.get_mut(count_key).unwrap();
        if *count >= consume_ammount {
            *count -= consume_ammount;
            // println!("CONSUMED - Before: {} | After: {} | At: {:?}", *count + consume_ammount, count, clock.time());
            shared_state.set(state);
            co.hold(hold_time).await;
        } else {
            let passivate_list = state.get_mut(passivate_key).unwrap();
            passivate_list[1] = Passivated::True;

            shared_state.set(state);
            co.passivate().await;

            let mut state = shared_state.take();
            let passivate_list = state.get_mut(passivate_key).unwrap();
            passivate_list[0] = Passivated::False;
            shared_state.set(state);
        }
    }
}
