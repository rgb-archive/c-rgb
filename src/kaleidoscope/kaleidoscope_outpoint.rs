use std::{mem, ptr};

use bitcoin::Address;
use bitcoin::util::hash::Sha256dHash;

use CRgbAllocatedPtr;
use generics::WrapperOf;
use hashmap::CRgbHashMap;
use kaleidoscope::address::CRgbAddress;

use super::kaleidoscope::tx_builder::BitcoinRgbOutPoints;

#[derive(Debug)]
#[repr(C)]
pub struct CRgbKaleidoscopeOutpoint {
    pub bitcoin_address: *mut CRgbAddress,
    pub bitcoin_amount: u64,
    pub rgb_outputs: Box<CRgbHashMap<Sha256dHash, u32>>,
}

impl WrapperOf<BitcoinRgbOutPoints> for CRgbKaleidoscopeOutpoint {
    fn decode(&self) -> BitcoinRgbOutPoints {
        let bitcoin_address: Option<Address> = match self.bitcoin_address as usize {
            0 => None,
            _ => unsafe {
                let boxed_addr = Box::from_raw(self.bitcoin_address);
                let decoded = boxed_addr.decode();
                mem::forget(boxed_addr);

                Some(decoded)
            }
        };

        BitcoinRgbOutPoints {
            bitcoin_address,
            bitcoin_amount: self.bitcoin_amount,
            rgb_outputs: self.rgb_outputs.decode().clone(),
        }
    }

    fn encode(native: &BitcoinRgbOutPoints) -> Self {
        let bitcoin_address: *mut CRgbAddress = match native.bitcoin_address {
            None => ptr::null_mut(),
            Some(ref addr) => Box::into_raw(Box::new(CRgbAddress::encode(addr)))
        };

        CRgbKaleidoscopeOutpoint {
            bitcoin_address,
            bitcoin_amount: native.bitcoin_amount,
            rgb_outputs: Box::new(CRgbHashMap::from_native(native.rgb_outputs.clone())),
        }
    }
}

#[no_mangle]
pub extern "C" fn rgb_init_kaleidoscope_outpoint_map() -> CRgbAllocatedPtr<CRgbHashMap<Sha256dHash, u32>> {
    CRgbAllocatedPtr {
        ptr: Box::new([CRgbHashMap::new()])
    }
}

#[no_mangle]
pub unsafe extern "C" fn rgb_push_kaleidoscope_outpoint_map(map: &mut CRgbHashMap<Sha256dHash, u32>, key: Sha256dHash, val: u32) {
    map
        .decode_mut()
        .insert(key, val);
}

#[no_mangle]
pub extern "C" fn rgb_debug_print_kaleidoscope_outpoint(outpoint: &CRgbKaleidoscopeOutpoint) {
    println!("{:#?}", outpoint.decode());
}