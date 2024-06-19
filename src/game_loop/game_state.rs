use crate::{GAME_CHANNEL, NOW_PLAYING_LED_CHANNEL, SCORE, SCORE_CHANNEL};
use core::sync::atomic::Ordering;
use embassy_time::{Duration, Instant, with_timeout};

use crate::message::{GameMsg, NowPlayingLedMsg, ScoreMsg};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum GameState {
    #[default]
    Stopped,
    Timing,
}

impl GameState {
    pub async fn advance(&mut self) {
        match *self {
            Self::Stopped => {
                // Await `ButtonDown` message
                self.process_stopped_button_down().await;

                // Await `ButtonUp` message
                self.process_stopped_button_up().await;

                *self = Self::Timing;
            }

            Self::Timing => {
                // Await `ButtonDown` message
                self.process_timing_button_down().await;

                // Await `ButtonUp` message
                self.process_timing_button_up().await;

                *self = Self::Stopped;
            }
        }
    }

    async fn process_stopped_button_down(&self) {
        // Process `GameMsg::ButtonDown`
        loop {
            let msg = GAME_CHANNEL.receive().await;

            match msg {
                GameMsg::ButtonDown => {
                    SCORE.store(0, Ordering::Relaxed);
                    break;
                }
                _ => {}
            }
        }
    }

    async fn process_stopped_button_up(&self) {
        // Process `GameMsg::ButtonUp`
        loop {
            let msg = GAME_CHANNEL.receive().await;

            match msg {
                GameMsg::ButtonUp => {
                    NOW_PLAYING_LED_CHANNEL.send(NowPlayingLedMsg::On).await;
                    self.random_pause().await;
                    SCORE_CHANNEL.send(ScoreMsg::Start).await;
                    break;
                }
                _ => {}
            }
        }
    }

    async fn process_timing_button_down(&self) {
        // Process `GameMsg::ButtonDown`
        loop {
            let msg = GAME_CHANNEL.receive().await;

            match msg {
                GameMsg::ButtonDown => {
                    SCORE_CHANNEL.send(ScoreMsg::Stop).await;
                    NOW_PLAYING_LED_CHANNEL.send(NowPlayingLedMsg::Off).await;
                    break;
                }
                _ => {}
            }
        }
    }

    async fn process_timing_button_up(&self) {
        // Process `GameMsg::ButtonUp`
        loop {
            let msg = GAME_CHANNEL.receive().await;

            use GameMsg as Gm;
            match msg {
                Gm::ButtonUp => {
                    break;
                }
                _ => {}
            }
        }
    }

    async fn random_pause(&self) {
        // Pause for a random duration from (0.5..=5)s.  Eat all messages (button key presses)
        // during delay period
        let delay_period = Duration::from_secs(3);
        let delay_start = Instant::now();
        loop {
            let elapsed_time = Instant::now().saturating_duration_since(delay_start);
            match elapsed_time >= delay_period {
                false => {
                    let _ =

                        with_timeout(delay_period - elapsed_time, GAME_CHANNEL.receive()).await;
                }
                true => break,
            }
        }
    }
}