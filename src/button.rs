use defmt::info;
use embassy_futures::select::{select, Either};
use embassy_rp::gpio::Input;
use embassy_time::{Duration, Timer};

use crate::shared_constants::{BUTTON_DEBOUNCE_DELAY, LONG_PRESS_DURATION};

/// A struct representing a virtual button input.
pub struct Button<'a>(Input<'a>);

impl<'a> Button<'a> {
    /// Creates a new `Button` instance.
    #[must_use]
    pub const fn new(button: Input<'a>) -> Self {
        Self(button)
    }

    /// Measures the duration of a button press but does not wait for the button to be released.
    ///
    /// This method does not wait for the button to be released.  It only waits
    /// as long as necessary to determine whether the press was "short" or "long".
    pub async fn press_duration(&mut self) -> PressDuration {
        // wait for the button to be released
        self.wait_for_button_up().await;
        self.debounce_delay().await;

        // Wait for the voltage level on this pin to go high (button has been pressed)
        self.wait_for_button_down().await;

        // Sometimes the start (and end) of a press can be "noisy" (fluctuations between
        // "pressed" and "unpressed" states on the microsecond time scale as the physical
        // contacts changing from "not touching" through "almost touching" to "touching" (or
        // vice-versa)).  We're going to ignore the button's state during the noisy, fluctuating
        // "almost touching" state.  This is called "debouncing".
        self.debounce_delay().await;

        // The button is now fully depressed.

        // Wait for the button to be released or to be a "LONG" press.

        let press_duration =
            match select(self.wait_for_release(), Timer::after(LONG_PRESS_DURATION)).await {
                Either::First(_) => PressDuration::Short,
                Either::Second(()) => PressDuration::Long,
            };
        info!("Press duration: {:?}", press_duration);
        press_duration
    }

    /// Pause for a predetermined time to let the button's state become consistent.
    async fn debounce_delay(&mut self) -> &mut Self {
        Timer::after(BUTTON_DEBOUNCE_DELAY).await;
        self
    }

    // wait for the button to be released
    async fn wait_for_button_up(&mut self) -> &mut Self {
        self.0.wait_for_low().await;
        self
    }

    /// Waits for the button to be released.
    pub async fn wait_for_up(&mut self) -> &mut Self {
        self.0.wait_for_low().await;
        self
    }

    /// Pause until voltage is present on the input pin.
    async fn wait_for_button_down(&mut self) -> &mut Self {
        self.0.wait_for_high().await;
        self
    }

    async fn wait_for_release(&mut self) -> &mut Self {
        self.0.wait_for_falling_edge().await;
        self
    }

    /// Waits for the button to be pressed.
    pub async fn wait_for_press(&mut self) -> &mut Self {
        self.0.wait_for_rising_edge().await;
        self
    }
}

// Instead of having API describing a short vs a long button-press vaguely using a `bool`, we define
// an `enum` to clarify what each state represents.  The compiler will compile this down to the
// very same `boolean` that we would have coded by hand.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, defmt::Format)]
pub enum PressDuration {
    Short,
    Long,
}

// Make `PressDuration` solely responsible for the distinction in `Duration` between a short and long
// button press.
impl From<Duration> for PressDuration {
    fn from(duration: Duration) -> Self {
        if duration >= LONG_PRESS_DURATION {
            Self::Long
        } else {
            Self::Short
        }
    }
}
