use gtk::{
    builders::EventControllerKeyBuilder,
    gdk::Display,
    glib,
    prelude::{ApplicationExt, ApplicationExtManual, GtkWindowExt},
    traits::{ButtonExt, GridExt, WidgetExt},
    Application, ApplicationWindow, Button, CssProvider, EventController, EventControllerKey,
    Inhibit, Label, StyleContext,
};

use std::{cell::RefCell, include_str, rc::Rc};

const APP_ID: &str = "org.gtk_rs.HelloWorld2";

const TITLE: &str = "Pizza Timer";

#[derive(Debug, Clone, Copy)]
enum CursorPosition {
    Ones,
    Tens,
}

#[derive(Debug, Clone)]
struct ClockState {
    pub minutes: u32,
    pub seconds: u32,
}

fn main() -> glib::ExitCode {
    let state = ClockState {
        minutes: 0,
        seconds: 0,
    };
    let state = Rc::new(RefCell::new(state));

    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_| load_css());
    // Connect to "activate" signal of `app`
    app.connect_activate(move |app| create_set_clock_window(app, state.clone()));

    // Run the application
    app.run()
}

fn create_set_clock_window(app: &Application, state: Rc<RefCell<ClockState>>) {
    let label_minutes_ten = gtk::Label::builder().label("0").build();
    let label_minutes_one = gtk::Label::builder().label("0").build();
    label_minutes_one.add_css_class("cursor");

    let label_seconds = gtk::Label::builder().label("00").build();

    let label_colon = gtk::Label::builder().label(":").build();

    let grid = gtk::Grid::builder().build();
    grid.attach(&label_minutes_ten, 0, 0, 1, 1);
    grid.attach(&label_minutes_one, 1, 0, 1, 1);
    grid.attach(&label_colon, 2, 0, 1, 1);
    grid.attach(&label_seconds, 3, 0, 1, 1);
    grid.add_css_class("timer");

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title(TITLE)
        .margin_top(0)
        .margin_bottom(0)
        .child(&grid)
        .build();
    window.add_css_class("body");

    let controller = EventControllerKey::builder().build();
    controller.connect_key_pressed(
        move |event_controller_key, keyval, keycode, modifier_type| {
            if let Some(c) = keyval.to_unicode() {
                state.replace_with(|s| handle_keypress(s, c));
                update_clock_labels(
                    &label_minutes_ten,
                    &label_minutes_one,
                    &label_seconds,
                    &*state.borrow(),
                )
            }
            Inhibit(true)
        },
    );

    window.add_controller(controller);

    // Present window
    window.present();
}

fn handle_keypress(state: &ClockState, c: char) -> ClockState {
    let state = state.clone();
    let ClockState { minutes, seconds } = state;
    match c {
        '.' => {
            let seconds = match seconds {
                0 => 30,
                15 => 45,
                30 => 15,
                _ => 0,
            };
            ClockState { seconds, ..state }
        }
        '0'..='9' => {
            let parsed = c.to_string().parse::<u32>().unwrap();

            let additor = minutes % 10 * 10;
            let minutes = parsed + additor;

            ClockState { minutes, ..state }
        }
        _ => state,
    }
}

fn update_clock_labels(
    minutes_ten_label: &Label,
    minutes_one_label: &Label,
    seconds_label: &Label,
    clock_state: &ClockState,
) {
    let ClockState { minutes, seconds } = clock_state;
    let minutes_ten_value = minutes / 10;
    let minutes_one_value = minutes % 10;
    minutes_ten_label.set_label(minutes_ten_value.to_string().as_str());
    minutes_one_label.set_label(minutes_one_value.to_string().as_str());

    let seconds_string = format!("{:02}", seconds);
    seconds_label.set_label(seconds_string.as_str());
}

fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_data(include_str!("css/style.css"));

    // Add the provider to the default screen
    StyleContext::add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
