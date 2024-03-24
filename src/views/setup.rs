use chrono::Duration;
use iced::{
    keyboard,
    widget::{row, text, Row},
};
use tokio::time::Instant;

use super::AppMessage;

const COMBO_TIMER_SECONDS: i64 = 1;

#[derive(Copy, Clone, Debug)]
pub enum SetupMessage {
    Ok,
    Combo { digit: u8, instant: Instant },
    SwitchSeconds,
    IncrementMinutes,
    DecrementMinutes,
}

#[derive(Debug, Clone, Copy)]
pub struct Combo {
    position: u32,
    instant_start: Instant,
}

#[derive(Debug, Clone, Copy)]
pub struct SetupView {
    pub combo: Option<Combo>,
    pub minutes: u32,
    pub seconds: u32,
    pub finished: bool,
}

impl Default for SetupView {
    fn default() -> Self {
        Self {
            minutes: 5,
            seconds: 0,
            finished: false,
            combo: None,
        }
    }
}

impl SetupView {
    pub fn update(&self, message: SetupMessage) -> SetupView {
        let SetupView {
            minutes, seconds, ..
        } = self;

        let next_state = match message {
            SetupMessage::Ok => SetupView {
                finished: true,
                ..*self
            },
            SetupMessage::Combo { digit, instant } => {
                let SetupView { combo, minutes, .. } = self;
                let (position, instant_start) = if let Some(Combo {
                    position,
                    instant_start,
                }) = combo
                {
                    if *instant_start
                        < instant
                            - Duration::try_seconds(COMBO_TIMER_SECONDS)
                                .unwrap()
                                .to_std()
                                .unwrap()
                    {
                        return SetupView {
                            combo: None,
                            ..*self
                        };
                    }
                    (*position, *instant_start)
                } else {
                    (0, instant)
                };

                let (minutes, combo) = if position == 0 {
                    (
                        digit as u32,
                        Some(Combo {
                            position: position + 1 % 2,
                            instant_start,
                        }),
                    )
                } else {
                    (minutes * 10 + (digit as u32), None)
                };
                SetupView {
                    minutes,
                    combo,
                    ..*self
                }
            }
            SetupMessage::SwitchSeconds => {
                let seconds = match seconds {
                    0..=14 => 30,
                    15..=29 => 45,
                    30..=44 => 15,
                    _ => 0,
                };
                SetupView {
                    seconds,
                    combo: None,
                    ..*self
                }
            }
            SetupMessage::IncrementMinutes => SetupView {
                minutes: (*minutes + 1) % 21,
                combo: None,
                ..*self
            },
            SetupMessage::DecrementMinutes => SetupView {
                minutes: (((*minutes as i32) - 1).rem_euclid(21)) as u32,
                combo: None,
                ..*self
            },
        };
        next_state
    }
    pub fn view(&self) -> Row<AppMessage> {
        let SetupView {
            minutes, seconds, ..
        } = self;
        let label_minutes = text(format!("{minutes:0>2}"));

        let label_seconds = text(format!("{seconds:0>2}"));

        let label_colon = text(":");
        row![label_minutes, label_colon, label_seconds]
    }

    pub fn key_board_subscription(
        key: keyboard::Key,
        _modifiers: keyboard::Modifiers,
    ) -> Option<SetupMessage> {
        match key.as_ref() {
            keyboard::Key::Named(keyboard::key::Named::Enter) => Some(SetupMessage::Ok),
            keyboard::Key::Character("-") => Some(SetupMessage::DecrementMinutes),
            keyboard::Key::Character("+") => Some(SetupMessage::IncrementMinutes),
            keyboard::Key::Character(".") => Some(SetupMessage::SwitchSeconds),
            keyboard::Key::Character(
                c @ ("0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"),
            ) => Some(SetupMessage::Combo {
                digit: c.parse().unwrap(),
                instant: Instant::now(),
            }),
            _ => None,
        }
    }
}
