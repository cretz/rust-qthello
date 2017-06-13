
extern crate qt_widgets;
extern crate qt_core;

use qt_widgets::application::Application;
use qt_widgets::push_button::PushButton;
use qt_core::string::String;


fn main() {
    Application::create_and_exit(|app| {
        
        let mut button = PushButton::new(&String::from("Hello, World"));
        button.show();

        Application::exec()
    })
}