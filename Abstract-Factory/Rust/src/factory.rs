use crate::button::{Button, LinuxButton, MacOSButton, WindowsButton};
use crate::checkbox::{Checkbox, LinuxCheckbox, MacOSCheckbox, WindowsCheckbox};

pub trait GUIFactory<B, C>
    where
        B: Button,
        C: Checkbox {

    fn create_button(&self) -> B;
    fn create_checkbox(&self) -> C;
}

pub struct LinuxFactory;

impl GUIFactory<LinuxButton, LinuxCheckbox> for LinuxFactory {
    fn create_button(&self) -> LinuxButton {
        LinuxButton
    }

    fn create_checkbox(&self) -> LinuxCheckbox {
        LinuxCheckbox
    }
}

pub struct WindowsFactory;

impl GUIFactory<WindowsButton, WindowsCheckbox> for WindowsFactory {
    fn create_button(&self) -> WindowsButton {
        WindowsButton
    }

    fn create_checkbox(&self) -> WindowsCheckbox {
        WindowsCheckbox
    }
}

pub struct MacOSFactory;

impl GUIFactory<MacOSButton, MacOSCheckbox> for MacOSFactory {
    fn create_button(&self) -> MacOSButton {
        MacOSButton
    }

    fn create_checkbox(&self) -> MacOSCheckbox {
        MacOSCheckbox
    }
}