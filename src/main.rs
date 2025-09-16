// #![windows_subsystem = "windows"]
#![allow(dead_code)]

use gilrs::*;

fn main() {
    // sandbox
    let mut gilrs = GilrsBuilder::new().with_default_filters(false).build().unwrap();

    // Iterate over all connected gamepads
    for (_id, gamepad) in gilrs.gamepads() {
        println!("{} is {:?}", gamepad.name(), gamepad.power_info());
    }

    loop {
        // Examine new events
        while let Some(Event { id, event, time, .. }) = gilrs.next_event() {
            println!("{:?} New event from {}: {:?}", time, id, event);
        }
    }
}
