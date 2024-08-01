use std::ffi::{CString};

#[allow(dead_code)]
pub struct InputtinoDeviceDefinition {
    pub def: super::InputtinoDeviceDefinition,
    // Keep those around since we are passing them as pointers
    name: CString,
    phys: CString,
    uniq: CString,
}

impl InputtinoDeviceDefinition {
    pub fn new(name: &str, vendor_id: u16, product_id: u16, version: u16, phys: &str, uniq: &str) -> Self {
        let name = CString::new(name).unwrap();
        let phys = CString::new(phys).unwrap();
        let uniq = CString::new(uniq).unwrap();
        let def = super::InputtinoDeviceDefinition {
            name: name.as_ptr(),
            vendor_id: vendor_id,
            product_id: product_id,
            version: version,
            device_phys: phys.as_ptr(), // TODO: optional, if not present random MAC address
            device_uniq: uniq.as_ptr(),
        };
        InputtinoDeviceDefinition { def, name, phys, uniq }
    }
}

pub unsafe extern "C" fn error_handler_fn(error_message: *const ::core::ffi::c_char,
                                          user_data: *mut ::core::ffi::c_void) {
    let error_str = std::ffi::CStr::from_ptr(error_message);
    let user_data = user_data as *mut CString;
    *user_data = CString::from(error_str);
}


#[macro_export]
macro_rules! get_nodes {
    ( $fn_call:expr,$var:expr ) => {
        {
            let mut nodes_count: core::ffi::c_int = 0;
            let nodes = $fn_call($var, &mut nodes_count);
            if nodes.is_null() {
                return Err("Failed to get nodes".to_string());
            }

            let mut result = Vec::new();
            for i in 0..nodes_count {
                let node = std::ffi::CString::from_raw(*nodes.offset(i as isize));
                result.push(node.to_str().unwrap().to_string());
            }
            Ok(result)
        }
    };
}

#[macro_export]
macro_rules! make_device {
    ($fn_call:expr, $device:expr) => {
        {
            let error_str = std::ptr::null_mut();
            let error_handler = super::InputtinoErrorHandler {
                eh: Some(error_handler_fn),
                user_data: error_str,
            };
            let device = $fn_call(&$device.def, &error_handler);
            if device.is_null() { // TODO: test this
                let error_msg = (error_str as *mut std::ffi::CString).as_ref().unwrap().to_str().unwrap();
                Err("Failed to create Mouse: ".to_string() + error_msg)
            } else {
                Ok(device)
            }
        }
    };
}
