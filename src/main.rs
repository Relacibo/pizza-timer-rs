use std::{cell::RefCell, include_str, rc::Rc, time::Duration};

use iced::{program, Command, Program, Settings, Subscription};
use views::{setup::SetupView, AppView};
mod views;

#[tokio::main]
async fn main() -> iced::Result {
    let program = iced::program("Pizza timer", AppView::update, AppView::view)
        .subscription(AppView::subscription);
    program.run()
}
