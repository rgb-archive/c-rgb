#![allow(dead_code)]

extern crate bitcoin;
extern crate libc;
extern crate rgb;

use bitcoin::Transaction;
use bitcoin::util::hash::Sha256dHash;
use c_bitcoin::CRgbOutPoint;
use contract::CRgbContract;
use generics::WrapperOf;
use rgb::traits::NeededTx;
use std::slice;

pub mod generics;
pub mod c_bitcoin;
pub mod contract;
pub mod needed_txs_map;

#[derive(Debug)]
#[repr(C)]
pub enum CRgbNeededTx {
    // TODO: impl WrapperOf
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

#[derive(Debug)]
#[repr(C)]
pub struct CRgbSerializedTx {
    pub size: u32,
    pub payload: *const u8,
}

impl WrapperOf<Transaction> for CRgbSerializedTx {
    fn decode(&self) -> Transaction {
        use bitcoin::network::serialize::deserialize;

        let sized_slice = unsafe { slice::from_raw_parts(self.payload, self.size as usize) };
        let native_tx: Transaction = deserialize(&sized_slice).unwrap();

        native_tx
    }

    fn encode(_native: &Transaction) -> Self {
        unimplemented!()
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

#[no_mangle]
pub extern "C" fn rgb_debug_print_serialized_tx(tx: &CRgbSerializedTx) {
    println!("{:#?}", tx.decode());
}