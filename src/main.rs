//! A 4-digit 7-segment clock that can be controlled by a button.
//!
//! Runs on a Raspberry Pi Pico RP2040. See the `README.md` for more information.
#![no_std]
#![no_main]
#![allow(clippy::future_not_send, reason = "Single-threaded")]
use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;
use lib::{Button, Clock, ClockNotifier, ClockState, Never, Result}; // This crate's own internal library
use panic_probe as _;

#[embassy_executor::main]
pub async fn main(spawner0: Spawner) -> ! {
    // If it returns, something went wrong.
    let err = inner_main(spawner0).await.unwrap_err();
    panic!("{err}");
}

#[expect(
    clippy::items_after_statements,
    reason = "Keeps related code together and avoids name conflicts"
)]
async fn inner_main(spawner: Spawner) -> Result<Never> {
    let hardware = lib::Hardware::default();
    static CLOCK_NOTIFIER: ClockNotifier = Clock::notifier();
    let mut clock = Clock::new(hardware.cells, hardware.segments, &CLOCK_NOTIFIER, spawner)?;
    let mut button = Button::new(hardware.button);
    info!("Clock and button created");

    // Run the state machine
    let mut state = ClockState::default();
    loop {
        defmt::info!("State: {:?}", state);
        state = state.run_and_next(&mut clock, &mut button).await;
    }
}

// TODO: Is testing possible?
