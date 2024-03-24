
use views::{AppView};
mod views;

#[tokio::main]
async fn main() -> iced::Result {
    let program = iced::program("Pizza timer", AppView::update, AppView::view)
        .subscription(AppView::subscription);
    program.run()
}
