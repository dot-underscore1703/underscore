use gtk::prelude::*;
use gtk::{Application, ApplicationWindow,Label, Button,pango,Box,gdk, CssProvider, StyleContext};
use gdk::RGBA;
use glib::clone;
use glib::ControlFlow;
use crate::underscore_globals;

fn adjust_box(nbox: &Box) {
    nbox.set_margin_top(underscore_globals::UNDERSCORE_CONFIG.get().unwrap().padding.margin_top as i32);
    nbox.set_margin_bottom(underscore_globals::UNDERSCORE_CONFIG.get().unwrap().padding.margin_top as i32);
    nbox.set_margin_start(underscore_globals::UNDERSCORE_CONFIG.get().unwrap().padding.margin_top as i32);
    nbox.set_margin_end(underscore_globals::UNDERSCORE_CONFIG.get().unwrap().padding.margin_top as i32);
}

fn fade_out_and_close(win: &ApplicationWindow) {
    // This controls how long the fade out is based on user config.
    let mut opacity = 1.0;
    let duration = underscore_globals::UNDERSCORE_CONFIG.get().unwrap().fade.fade_out_ms;
    let tick_ms = 16;

    let steps = duration as f64 / tick_ms as f64;
    let decrement = 1.0 / steps;

    glib::timeout_add_local(std::time::Duration::from_millis(16), glib::clone!(
        #[strong] win,
        move || {
            opacity -= decrement;

            if opacity <= 0.0 {
                win.close();
                return glib::ControlFlow::Break;
            }

            win.set_opacity(opacity);
            glib::ControlFlow::Continue
        }
    ));
}

fn fade_in_and_show(win: &ApplicationWindow) {
    // This controls how long the fade in is based on user config.
    let mut opacity = 0.0;
    let duration = underscore_globals::UNDERSCORE_CONFIG.get().unwrap().fade.fade_in_ms;
    let tick_ms = 16;

    let steps = duration as f64 / tick_ms as f64;
    let increment = 1.0 / steps;

    win.set_opacity(opacity);
    position_top_right(&win);
    win.show_all();

    glib::timeout_add_local(std::time::Duration::from_millis(16), glib::clone!(
        #[strong] win,
        move || {
            opacity += increment;

            if opacity >= 1.0 {
                win.set_opacity(1.0);
                return glib::ControlFlow::Break;
            }

            win.set_opacity(opacity);
            glib::ControlFlow::Continue
        }
    ));
}

fn adjust_label(label: &Label) {
    label.set_line_wrap(true);
    label.set_line_wrap_mode(pango::WrapMode::Word);
    label.set_max_width_chars(40);
}


fn position_top_right(win: &gtk::ApplicationWindow) {
    win.set_position(gtk::WindowPosition::None);
    win.show_all();

    let display = gdk::Display::default().expect("No display");
    let monitor = display.primary_monitor().expect("No primary monitor");
    let geometry = monitor.geometry();

    let window_width = win.allocated_width();
    let margin = 20;

    let x = geometry.x() + geometry.width() - window_width - margin;
    let y = geometry.y() + margin;

    win.move_(x, y);
}

fn set_window_color(widget: &impl gtk::prelude::WidgetExt, css_color: &str) {
    let css = format!(
        "* {{
            background-color: {};
            color: white;
        }}",
        css_color
    );

    let provider = Css
}


pub fn send_notification(app: &Application, text: &str) {

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
    adjust_box(&notification_box);     // This adds padding so the button and label don't slam against the sides of the box.


    // The label will take the text parameter (how tf do you spell that word??) and set it as a label on the notification window.
    let notification_label = Label::new(Some(text));
    adjust_label(&notification_label);     // This makes the contents of the label wrap around the window.

    // This dismiss button closes the notification when clicked...
    let dismiss_button = Button::with_label("Dismiss");
    dismiss_button.connect_clicked(clone!(  // We use glib::clone!() so we dont lose ownership stuff blah blah blah
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

    // This sets the width of the window to be static, and let the contents of the window determine its height.
    win.set_default_size(50,-1);

    // And then we show it.
    fade_in_and_show(&win);

    // ... otherwise it closes after three seconds.
    glib::timeout_add_seconds_local(3, clone!(
        #[strong]   
        win,
        move || {
            fade_out_and_close(&win);
            ControlFlow::Break
        }
    ));
}