use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::str::FromStr;

use bitcoin::Address;
use libc;

use generics::WrapperOf;

//#[derive(Debug)]
#[repr(C)]
pub struct CRgbAddress {
    pub str: [c_char; 40]
}

impl WrapperOf<Address> for CRgbAddress {
    fn decode(&self) -> Address {
        let address_string = unsafe { CStr::from_ptr(&self.str as *const c_char) };

        Address::from_str(address_string.to_str().unwrap()).unwrap()
    }

    fn encode(native: &Address) -> Self {
        let mut crgb_address = CRgbAddress {
            str: [0; 40]
        };

        let cstr = CString::new(native.to_string()).unwrap();

        unsafe {
            libc::strcpy(&mut crgb_address.str[0] as *mut c_char, cstr.as_ptr() as *mut i8);
        };

        crgb_address
    }
}

#[no_mangle]
pub extern "C" fn rgb_debug_print_bitcoin_address(address: &CRgbAddress) {
    println!("{:?}", address.decode());
}