use crate::button::Button;
use crate::checkbox::Checkbox;
use crate::factory::GUIFactory;

pub struct Application<B, C>
    where
        B: Button,
        C: Checkbox {

    button: B,
    checkbox: C
}

impl<B, C> Application<B, C>
    where
        B: Button,
        C: Checkbox {

    pub(crate) fn new(factory: impl GUIFactory<B, C>) -> Application<B, C> {
        Application {
            button: factory.create_button(),
            checkbox: factory.create_checkbox()
        }
    }

    pub(crate) fn paint(&self) {
        self.button.paint();
        self.checkbox.paint();
    }
}