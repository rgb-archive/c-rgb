#![allow(dead_code)]

extern crate bitcoin;
extern crate libc;
extern crate rgb;

use bitcoin::util::hash::Sha256dHash;
use c_bitcoin::CRgbOutPoint;
use contract::CRgbContract;
use generics::WrapperOf;
use rgb::traits::NeededTx;

pub mod generics;
pub mod c_bitcoin;
pub mod contract;

#[derive(Debug)]
#[repr(C)]
pub enum CRgbNeededTx {
    FromTXID(Sha256dHash, u32),
    // The u32 is just a dummy value which makes this branch the same size as the other one
    WhichSpendsOutPoint(CRgbOutPoint),
}

impl CRgbNeededTx {
    pub fn decode(&self) -> NeededTx {
        match self {
            CRgbNeededTx::FromTXID(txid, _) => NeededTx::FromTXID(txid.clone()),
            CRgbNeededTx::WhichSpendsOutPoint(rgb_outpoint) => NeededTx::WhichSpendsOutPoint(rgb_outpoint.decode())
        }
    }

    pub fn encode(element: &NeededTx) -> CRgbNeededTx {
        match element {
            NeededTx::FromTXID(txid) => CRgbNeededTx::FromTXID(txid.clone(), 0xDEADBEEF),
            NeededTx::WhichSpendsOutPoint(outpoint) => CRgbNeededTx::WhichSpendsOutPoint(CRgbOutPoint::encode(outpoint))
        }
    }
}

// Debugging functions

#[no_mangle]
pub extern "C" fn rgb_debug_sha256d(hash: &Sha256dHash) {
    println!("{:#?}", hash);
}

#[no_mangle]
pub extern "C" fn rgb_debug_print_contract(contract: &CRgbContract) {
    println!("{:#?}", contract.decode());
}

#[no_mangle]
pub extern "C" fn rgb_debug_print_needed_tx(e: &CRgbNeededTx) {
    println!("{:#?}", e.decode());
}