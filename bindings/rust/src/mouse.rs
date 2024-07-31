use std::ffi::{CString};
use crate::{inputtino_mouse_create, inputtino_mouse_destroy, inputtino_mouse_get_nodes, inputtino_mouse_move, inputtino_mouse_move_absolute, inputtino_mouse_press_button, inputtino_mouse_release_button, inputtino_mouse_scroll_horizontal, inputtino_mouse_scroll_vertical};
use crate::common::{InputtinoDeviceDefinition, error_handler_fn};

pub struct InputtinoMouse {
    mouse: *mut super::InputtinoMouse,
}

impl InputtinoMouse {
    pub fn new(device: &InputtinoDeviceDefinition) -> Result<Self, String> {
        let error_str = std::ptr::null_mut();
        let error_handler = super::InputtinoErrorHandler {
            eh: Some(error_handler_fn),
            user_data: error_str,
        };
        unsafe {
            let mouse = inputtino_mouse_create(&device.def, &error_handler);
            if mouse.is_null() { // TODO: test this
                let error_msg = (error_str as *mut std::ffi::CString).as_ref().unwrap().to_str().unwrap();
                Err("Failed to create Mouse: ".to_string() + error_msg)
            } else {
                Ok(InputtinoMouse { mouse })
            }
        }
    }

    pub fn get_nodes(&self) -> Result<Vec<String>, String> {
        unsafe {
            let mut nodes_count: core::ffi::c_int = 0;
            let nodes = inputtino_mouse_get_nodes(self.mouse, &mut nodes_count);
            if nodes.is_null() {
                return Err("Failed to get nodes".to_string());
            }

            let mut result = Vec::new();
            for i in 0..nodes_count {
                let node = CString::from_raw(*nodes.offset(i as isize));
                result.push(node.to_str().unwrap().to_string());
            }
            Ok(result)
        }
    }

    pub fn move_rel(&self, x: i32, y: i32) {
        unsafe {
            inputtino_mouse_move(self.mouse, x, y);
        }
    }

    pub fn move_abs(&self, x: i32, y: i32, screen_width: i32, screen_height: i32) {
        unsafe {
            inputtino_mouse_move_absolute(self.mouse, x, y, screen_width, screen_height);
        }
    }

    pub fn press_button(&self, button: super::INPUTTINO_MOUSE_BUTTON) {
        unsafe {
            inputtino_mouse_press_button(self.mouse, button);
        }
    }

    pub fn release_button(&self, button: super::INPUTTINO_MOUSE_BUTTON) {
        unsafe {
            inputtino_mouse_release_button(self.mouse, button);
        }
    }

    pub fn scroll_vertical(&self, amount: i32) {
        unsafe {
            inputtino_mouse_scroll_vertical(self.mouse, amount);
        }
    }

    pub fn scroll_horizontal(&self, amount: i32) {
        unsafe {
            inputtino_mouse_scroll_horizontal(self.mouse, amount);
        }
    }
}

impl Drop for InputtinoMouse {
    fn drop(&mut self) {
        unsafe {
            inputtino_mouse_destroy(self.mouse);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::{CString};
    use super::*;

    #[test]
    fn test_inputtino_c_mouse() {
        let device_name = CString::new("Rusty Mouse").unwrap();
        let device_phys = CString::new("Rusty Mouse Phys").unwrap();
        let device_uniq = CString::new("Rusty Mouse Uniq").unwrap();
        let def = crate::InputtinoDeviceDefinition {
            name: device_name.as_ptr(),
            vendor_id: 0,
            product_id: 0,
            version: 0,
            device_phys: device_phys.as_ptr(),
            device_uniq: device_uniq.as_ptr(),
        };
        // TODO: test this somehow
        let error_str = std::ptr::null_mut();
        let error_handler = crate::InputtinoErrorHandler {
            eh: Some(error_handler_fn),
            user_data: error_str,
        };

        unsafe {
            let mouse = inputtino_mouse_create(&def, &error_handler);
            assert!(!mouse.is_null());

            let mut nodes_count: core::ffi::c_int = 0;
            let nodes = inputtino_mouse_get_nodes(mouse, &mut nodes_count);
            assert_eq!(nodes_count, 2);
            assert!(!nodes.is_null());
            // Check that the nodes start with /dev/input/event
            assert!(CString::from_raw(*nodes.offset(0)).to_str().unwrap().starts_with("/dev/input/event"));
            assert!(CString::from_raw(*nodes.offset(1)).to_str().unwrap().starts_with("/dev/input/event"));

            inputtino_mouse_destroy(mouse);
        }
    }
}
