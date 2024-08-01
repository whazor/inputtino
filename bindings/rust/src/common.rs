#[allow(dead_code)]
pub struct InputtinoDeviceDefinition {
    pub def: super::InputtinoDeviceDefinition,
    // Keep those around since we are passing them as pointers
    name: std::ffi::CString,
    phys: std::ffi::CString,
    uniq: std::ffi::CString,
}

impl InputtinoDeviceDefinition {
    pub fn new(name: &str, vendor_id: u16, product_id: u16, version: u16, phys: &str, uniq: &str) -> Self {
        let name = std::ffi::CString::new(name).unwrap();
        let phys = std::ffi::CString::new(phys).unwrap();
        let uniq = std::ffi::CString::new(uniq).unwrap();
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
    let user_data = user_data as *mut std::ffi::CString;
    *user_data = std::ffi::CString::from(error_str);
}
