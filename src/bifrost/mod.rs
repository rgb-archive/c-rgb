extern crate bifrost;

use std::ffi::CStr;
use std::os::raw::c_char;

use bitcoin::util::hash::Sha256dHash;

use c_bitcoin::CRgbOutPoint;
use CRgbAllocatedArray;
use generics::WrapperOf;
use proof::CRgbProof;

use self::bifrost::client::{get_proofs_for, upload_proofs};

#[no_mangle]
pub extern "C" fn rgb_bifrost_upload_proofs(server: *const c_char, proof: &CRgbProof, txid: Sha256dHash) -> u8 {
    let server = unsafe { CStr::from_ptr(server) };

    match upload_proofs(&server.to_str().unwrap().to_string(), &proof.decode(), &txid) {
        Ok(_) => 1,
        Err(_e) => 0
    }
}

#[no_mangle]
pub extern "C" fn rgb_bifrost_get_proofs_for(server: *const c_char, outpoint: &CRgbOutPoint) -> CRgbAllocatedArray<CRgbProof> {
    let server = unsafe { CStr::from_ptr(server) };

    match get_proofs_for(&server.to_str().unwrap().to_string(), &outpoint.decode()) {
        Ok(vec) => {
            let encoded_vec: Vec<CRgbProof> = vec
                .iter()
                .map(|ref x| CRgbProof::encode(x))
                .collect();

            CRgbAllocatedArray {
                ptr: encoded_vec.into_boxed_slice()
            }
        }
        Err(_e) => CRgbAllocatedArray {
            ptr: Box::new([])
        }
    }
}