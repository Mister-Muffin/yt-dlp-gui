use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, CenterBox, CheckButton, Text};
use native_dialog::{FileDialog, MessageDialog, MessageType};
use std::cell::RefCell;
use std::ops::Deref;
use std::process::{Command, Output};
use std::rc::Rc;

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

    let (text, text_clone) = wrap(
        Text::builder()
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .placeholder_text("Insert Youtube URL")
            .width_chars(42)
            .build(),
    );

    let (only_audio_check, only_audio_check_clone) = wrap(
        CheckButton::builder()
            .label("Audio only")
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .build(),
    );

    // Create a button with label and margins
    let button = Button::builder()
        .label("Download!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Connect to "clicked" signal of `button`
    button.connect_clicked(move |button| {
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
            .set_title("Do you want to select this directory?")
            .set_text(&format!("{:#?}", path))
            .show_confirm()
            .unwrap();

        let text_field = text.take().text().to_string();
        let url = text_field.as_str();

        println!("{} 🤯", text.take().text());

        if yes {
            button.set_label(&url);
            button.set_label(&run_ytdlp(
                &path.to_str().unwrap(),
                url,
                only_audio_check.take().is_active(),
            ));
        }
    });

    let center_audio_only_check = CenterBox::new();
    center_audio_only_check.set_center_widget(Some(only_audio_check_clone.borrow().deref()));

    let center_text = CenterBox::new();
    center_text.set_center_widget(Some(text_clone.borrow().deref()));

    let container = gtk::Box::new(gtk::Orientation::Vertical, 0);
    container.append(&center_text);
    container.append(&button);
    container.append(&center_audio_only_check);

    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Yt-dlp GUI")
        .default_width(360)
        .build();

    window.set_child(Some(&container));

    // Present window
    window.show();
}

fn run_ytdlp(path: &str, url: &str, audio_only: bool) -> String {
    let mut output: Output;
    if audio_only {
        output = {
            Command::new("yt-dlp")
                .current_dir(path)
                .arg("-x")
                .arg("--audio-format")
                .arg("mp3")
                .arg(url)
                .output()
                .expect("failed to execute process")
        };
    } else {
        output = {
            Command::new("yt-dlp")
                .current_dir(path)
                .arg(url)
                .output()
                .expect("failed to execute process")
        };
    }

    let hello = output.stdout;

    std::str::from_utf8(&hello).expect("Invalid UTF-8 sequence");

    "Success".to_string()
}

fn wrap<T>(widget: T) -> (Rc<RefCell<T>>, Rc<RefCell<T>>)  {
    let wrapped = Rc::new(RefCell::new(widget));
    return (wrapped, wrapped.clone())
}