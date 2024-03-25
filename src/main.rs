use std::{cell::RefCell, rc::Rc, sync::{Arc, Mutex}};

use alarm::Alarm;
use chrono::Duration;

use views::{AppMessage, AppView};
mod alarm;
mod views;

#[tokio::main]
async fn main() -> iced::Result {
    let alarm = Arc::new(Mutex::new(Alarm::new(Duration::try_seconds(10).unwrap())));
    let program = iced::program(
        "Pizza timer",
        move |state: &mut AppView, message: AppMessage| {
            AppView::update(state, message, &alarm)
        },
        AppView::view,
    )
    .subscription(AppView::subscription);
    program.run()
}
