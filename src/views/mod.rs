use chrono::Duration;
use iced::{keyboard, time, Element, Subscription, Theme};
use tokio::time::Instant;

use self::{
    ringing::RingingView,
    running::RunningView,
    setup::{SetupMessage, SetupView},
};

pub mod ringing;
pub mod running;
pub mod setup;

#[derive(Debug, Clone, Copy)]
pub enum AppMessage {
    Setup(SetupMessage),
    Tick(Instant),
    Reset,
}

#[derive(Debug, Clone, Copy)]
pub enum AppView {
    Setup(SetupView),
    Running(RunningView),
    Ringing(RingingView),
}

impl Default for AppView {
    fn default() -> Self {
        AppView::Setup(Default::default())
    }
}

impl AppView {
    pub fn update(&mut self, message: AppMessage) {
        let next_state = match (*self, message) {
            (AppView::Setup(s), AppMessage::Setup(m)) => {
                let updated = s.update(m);
                if updated.finished {
                    Some(AppView::Running(RunningView::init(s, Instant::now())))
                } else {
                    Some(AppView::Setup(updated))
                }
            }
            (AppView::Running(r), AppMessage::Tick(i)) => {
                let updated = r.update(i);
                if updated.finished {
                    Some(AppView::Ringing(RingingView::init(Instant::now())))
                } else {
                    Some(AppView::Running(updated))
                }
            }
            (AppView::Ringing(r), AppMessage::Tick(i)) => {
                let updated = r.update(i);
                if updated.finished {
                    Some(Default::default())
                } else {
                    Some(AppView::Ringing(updated))
                }
            }
            (_, AppMessage::Reset) => Some(Default::default()),
            _ => None,
        };
        if let Some(s) = next_state {
            *self = s;
        }
        println!("new state: {self:?}");
    }
    pub fn view<'a>(&'a self) -> Element<'a, AppMessage, Theme> {
        match self {
            AppView::Setup(s) => s.view().into(),
            AppView::Running(r) => r.view().into(),
            AppView::Ringing(r) => r.view().into(),
        }
    }

    pub fn subscription(&self) -> Subscription<AppMessage> {
        let sub = match self {
            AppView::Running(..) | AppView::Ringing(..) => {
                let action = time::every(Duration::try_milliseconds(100).unwrap().to_std().unwrap())
                    .map(|i| AppMessage::Tick(i.into()));
                Some(action)
            }
            AppView::Setup(..) => {
                let s = keyboard::on_key_press(|key, modifiers| {
                    SetupView::key_board_subscription(key, modifiers).map(AppMessage::Setup)
                });
                Some(s)
            }
            _ => None,
        };

        let default_keypress = keyboard::on_key_press(|key, _| match key.as_ref() {
            keyboard::Key::Character("r") => Some(AppMessage::Reset),
            _ => None,
        });
        let batched_subscriptions = sub
            .into_iter()
            .chain([default_keypress])
            .collect::<Vec<_>>();

        Subscription::batch(batched_subscriptions)
    }
}
