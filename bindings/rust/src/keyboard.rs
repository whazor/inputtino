use crate::common::{error_handler_fn, InputtinoDeviceDefinition};
use crate::{get_nodes, inputtino_keyboard_create, inputtino_keyboard_get_nodes, make_device};

pub struct InputtinoKeyboard {
    kb: *mut super::InputtinoKeyboard,
}

impl InputtinoKeyboard {
    pub fn new(device: &InputtinoDeviceDefinition) -> Result<Self, String> {
        unsafe {
            let dev = make_device!(inputtino_keyboard_create, device);
            match dev {
                Ok(kb) => Ok(InputtinoKeyboard { kb }),
                Err(e) => Err(e),
            }
        }
    }

    pub fn get_nodes(&self) -> Result<Vec<String>, String> {
        unsafe {
            get_nodes!(inputtino_keyboard_get_nodes, self.kb)
        }
    }

    pub fn press_key(&self, key: i16) {
        unsafe {
            super::inputtino_keyboard_press(self.kb, key);
        }
    }

    pub fn release_key(&self, key: i16) {
        unsafe {
            super::inputtino_keyboard_release(self.kb, key);
        }
    }
}

impl Drop for InputtinoKeyboard {
    fn drop(&mut self) {
        unsafe {
            super::inputtino_keyboard_destroy(self.kb);
        }
    }
}
