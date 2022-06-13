use std::cell::RefCell;
use std::rc::Rc;
use std::ops::Deref;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Text};
use native_dialog::{FileDialog, MessageDialog, MessageType};

fn main() {
    // Create a new application
    let app = Application::builder()
        .application_id("org.schweininchen.yt-dlp-gui")
        .build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {

    let text = Rc::new(RefCell::new(Text::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .text("Youtube url")
        .build()));

    let text2 = Text::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .text("Youtube url")
        .build();

    let text3 = Rc::new(RefCell::new(text2));

    let text_clone = text.clone();
    let text3_clone = text3.clone();

    // Create a button with label and margins
    let button = Button::builder()
    .label("Press me!")
    .margin_top(12)
    .margin_bottom(12)
    .margin_start(12)
    .margin_end(12)
    .build();

    // Connect to "clicked" signal of `button`
    button.connect_clicked(move|button| {
        // Set the label to "Hello World!" after the button has been clicked on
        button.set_label("Hello World!");

        let path = FileDialog::new()
        .set_location("~")
        //.add_filter("PNG Image", &["png"])
        //.add_filter("JPEG Image", &["jpg", "jpeg"])
        .show_open_single_dir()
        .unwrap();

        let path = match path {
            Some(path) => path,
            None => return,
        };

        let yes = MessageDialog::new()
        .set_type(MessageType::Info)
        .set_title("Do you want to open the file?")
        .set_text(&format!("{:#?}", path))
        .show_confirm()
        .unwrap();

        let text_field = text.take().text().to_string();
        let url = text_field.as_str();

        println!("{} 🤯", text3.take().text());

        if yes {
            button.set_label(&url);
            button.set_label(&run_ytdlp(&path.to_str().unwrap(), url));
        }
    });

    // Und? funktioniert? Ne :( Der Button der die Url beinhalten sollte ist leer  >:(
    let container = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let _ = text3_clone.borrow().deref();
    let _ = &text_clone.take();
    container.append(text3_clone.borrow().deref());
    container.append(&button);

    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .build();

    window.set_child(Some(&container));

    // Present window
    window.show();
}

fn run_ytdlp(path: &str, url: &str) -> String {
    use std::process::Command;

    let output = {
        Command::new("yt-dlp").current_dir(path)
            .arg(url)
            .output()
            .expect("failed to execute process")
    };

    let hello = output.stdout;

    let s = match std::str::from_utf8(&hello) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    s.to_owned()
}
