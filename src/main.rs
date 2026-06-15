use gtk::prelude::*;
use gtk::Application;
mod helpers;
mod underscore_globals;
mod popup;
mod config;

// Here marks the beginning of my journey in commenting my code.
// Because a little birdie told me, people like it when public code is understandable.
// And honestly, I too hate it when I'm trying to understand how something works and the moron who made it didn't put comments in.
// So enjoy.

// <=== 'UNDERSCORE' NOTIFICATION DAEMON ===>

const APPLICATION_ID: &str = "com.underscore.nd";

fn main() {
    underscore_globals::init_globals().expect("Could not initialise globals!");
    let notification_app = Application::builder()
        .application_id(APPLICATION_ID)
        .build();

    notification_app.connect_activate(|app| {
        popup::send_notification(&app, "Testing testing")
    });
    notification_app.run();
}


// am i a good enough person?