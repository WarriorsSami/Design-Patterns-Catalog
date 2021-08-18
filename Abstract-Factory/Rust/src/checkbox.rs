pub trait Checkbox {
    fn paint(&self);
}

pub struct LinuxCheckbox;

impl Checkbox for LinuxCheckbox {
    fn paint(&self) {
        println!("You have created a Linux Checkbox.");
    }
}

pub struct WindowsCheckbox;

impl Checkbox for WindowsCheckbox {
    fn paint(&self) {
        println!("You have created a Windows Checkbox.");
    }
}

pub struct MacOSCheckbox;

impl Checkbox for MacOSCheckbox {
    fn paint(&self) {
        println!("You have created a MacOS Checkbox.")
    }
}