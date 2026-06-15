use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Window, WindowType,Label, Button};

// Here marks the beginning of my journey in commenting my code.
// Because a little birdie told me, people like it when public code is understandable.
// And honestly, I too hate it when I'm trying to understand how something works and the moron who made it didn't put comments in.
// So enjoy.

// <=== 'UNDERSCORE' NOTIFICATION DAEMON ===>

const APPLICATION_ID: &str = "com.underscore.nd";

fn send_notification(app: &Application, text: &str) {

    // This creates the actual window for the notification, display text, buttons etc.
    let win = ApplicationWindow::builder()
            .application(app)
            .decorated(false)
            .skip_taskbar_hint(true)
            .accept_focus(false)
            .resizable(false)
            .title("Underscore | Notification Daemon")
            .build();


    // A GTK window can only contain one child, so we create a box to display multiple items.
    let notification_box = gtk::Box::new(gtk::Orientation::Vertical, 8);

    // The label will take the text parameter (how tf do you spell that word??) and set it as a label on the notification window.
    let notification_label = Label::new(Some(text));

    // This dismiss button closes the notification when clicked...
    let dismiss_button = Button::with_label("Dismiss");
    dismiss_button.connect_clicked(glib::clone!(  // We use glib::clone!() so we dont lose ownership stuff blah blah blah
        #[strong]
        win,
        move |_| { // Make a closure to close the window! ^-^
            win.close(); 
        }
    ));

    // Pack it all into the notification_box so it all displays properly.
    notification_box.pack_start(&notification_label,false,false,0);
    notification_box.pack_start(&dismiss_button,false,false,0);

    // Now we dd the notification box to the window.
    win.add(&notification_box);

    // set_keep_above(true) is what renders it above everything else so nothing can go on top of it (ideally, atleast)
    win.set_keep_above(true);

    // And then we show it.
    win.show_all();

    // ... otherwise it closes after three seconds.
}

fn main() {
    let notification_app = Application::builder()
        .application_id(APPLICATION_ID)
        .build();

    notification_app.connect_activate(|app| {
        send_notification(&app, "Testing testing testing testing testing")
    });
    notification_app.run();
}


// am i a good enough person?
