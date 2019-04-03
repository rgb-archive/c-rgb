use std::ffi::CStr;
use std::ptr;

use bitcoin::Transaction;
use bitcoin::util::hash::Sha256dHash;
use rgb::traits::NeededTx;

use ::{CRgbNeededTx, libc};
use contract::CRgbContract;
use CRgbAllocatedArray;
use hashmap::CRgbHashMap;
use proof::CRgbProof;

unsafe fn rust_drop_ptr<T>(ptr: *mut libc::c_void) {
    ptr::drop_in_place(ptr as *mut T);
}

unsafe fn rust_drop_array<T>(arr: *mut libc::c_void) {
    ptr::drop_in_place(arr as *mut CRgbAllocatedArray<T>);
}

#[no_mangle]
pub unsafe extern "C" fn _rgb_free_internal_struct(ptr: *mut libc::c_void, type_str: *const libc::c_char) {
    let type_str = CStr::from_ptr(type_str).to_str().unwrap();

    match type_str {
        "struct rgb_contract" => rust_drop_ptr::<CRgbContract>(ptr),
        "struct rgb_proof" => rust_drop_ptr::<CRgbProof>(ptr),
        "struct rgb_needed_tx" => rust_drop_ptr::<CRgbNeededTx>(ptr),
        "struct rgb_needed_tx_map" => rust_drop_ptr::<CRgbHashMap<NeededTx, Transaction>>(ptr),
        "struct rgb_sha256d" => rust_drop_ptr::<Sha256dHash>(ptr),

        _ => panic!("Could not drop unknown type {:?}", type_str),
    }
}

#[no_mangle]
pub unsafe extern "C" fn _rgb_free_internal_array(arr: *mut libc::c_void, type_str: *const libc::c_char) {
    let type_str = CStr::from_ptr(type_str).to_str().unwrap();

    match type_str {
        "struct rgb_allocated_array_rgb_contract" => rust_drop_array::<CRgbContract>(arr),
        "struct rgb_allocated_array_rgb_proof" => rust_drop_array::<CRgbProof>(arr),
        "struct rgb_allocated_array_rgb_needed_tx" => rust_drop_array::<CRgbNeededTx>(arr),
        "struct rgb_allocated_array_rgb_needed_tx_map" => rust_drop_array::<CRgbHashMap<NeededTx, Transaction>>(arr),
        "struct rgb_allocated_array_rgb_sha256d" => rust_drop_array::<Sha256dHash>(arr),
        "struct rgb_allocated_array_uint8_t" => rust_drop_array::<u8>(arr),

        _ => panic!("Could not drop unknown type {:?}", type_str),
    }
}