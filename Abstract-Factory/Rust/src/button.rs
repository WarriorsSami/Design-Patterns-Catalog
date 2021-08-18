pub trait Button {
    fn paint(&self);
}

pub struct LinuxButton;

impl Button for LinuxButton {
    fn paint(&self) {
        println!("You have created a Linux Button.");
    }
}

pub struct WindowsButton;

impl Button for WindowsButton {
    fn paint(&self) {
        println!("You have created a Windows Button.");
    }
}

pub struct MacOSButton;

impl Button for MacOSButton {
    fn paint(&self) {
        println!("You have created a MacOS Button.")
    }
}