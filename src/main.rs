use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
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

    let (text, text_clone) = wrap(Text::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .text("Youtube url")
        .build());

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

        let url = text.into_inner().text().to_string().as_str();

        if yes {
            button.set_label(&path.to_str().unwrap());
            button.set_label(&run_ytdlp(&path.to_str().unwrap(), url));
        }
    });


    let container = gtk::Box::new(gtk::Orientation::Vertical, 0);
    container.append(&text_clone.borrow().deref());
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

fn wrap<T>(widget: T) -> (Rc<RefCell<T>>, Rc<RefCell<T>>)  {
    let wrapped = Rc::new(RefCell::new(widget));
    return (wrapped, wrapped.clone())
}