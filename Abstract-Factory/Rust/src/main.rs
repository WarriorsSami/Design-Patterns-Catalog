mod button;
mod checkbox;
mod factory;
mod application;

use std::env;
use crate::application::Application;
use crate::factory::{LinuxFactory, WindowsFactory, MacOSFactory};


/*fn configure_application<B, C>() -> Application<B, C>
    where
        B: Button,
        C: Checkbox {

    let mut app: Application<B, C>;
    let os_name: String = env::consts::OS.parse().unwrap();

    if os_name.contains("linux") {
        let gui = LinuxFactory;
        app = Application::new(gui);
    } else if os_name.contains("windows") {
        let gui = WindowsFactory;
        app = Application::new(gui);
    } else {
        let gui = MacOSFactory;
        app = Application::new(gui);
    }
    app
}*/

fn main() {
    //configure_application().paint();

    let os_name: String = env::consts::OS.parse().unwrap();

    if os_name.contains("linux") {
        let gui = LinuxFactory;
        let app = Application::new(gui);
        app.paint();
    } else if os_name.contains("windows") {
        let gui = WindowsFactory;
        let app = Application::new(gui);
        app.paint();
    } else {
        let gui = MacOSFactory;
        let app = Application::new(gui);
        app.paint();
    }
}