use bitcoin::Address;
use bitcoin::util::hash::Sha256dHash;
use c_bitcoin::CRgbBitcoinNetwork;
use c_bitcoin::CRgbOutPoint;
use CRgbNeededTx;
use generics::WrapperOf;
use libc;
use rgb::contract::Contract;
use rgb::traits::Verify;
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::*;
use std::slice;
use std::str::FromStr;

//#[derive(Debug)]
#[repr(C)]
pub struct CRgbContract {
    pub title: [c_char; 256],
    pub issuance_utxo: CRgbOutPoint,
    pub initial_owner_utxo: CRgbOutPoint,
    pub network: CRgbBitcoinNetwork,
    pub total_supply: u32,
}

impl WrapperOf<Contract> for CRgbContract {
    fn decode(&self) -> Contract {
        let dummy_addrstr = "BC1QW508D6QEJXTDG4Y5R3ZARVARY0C5XW7KV8F3T4"; // TODO: this will be removed with PR#7
        let dummy_addr = Address::from_str(dummy_addrstr).unwrap();

        let cstr = unsafe { CStr::from_ptr(&self.title[0] as *const c_char) };

        Contract {
            title: cstr.to_str().unwrap().to_string(),
            issuance_utxo: self.issuance_utxo.decode(),
            initial_owner_utxo: self.initial_owner_utxo.decode(),
            network: self.network.decode(),
            total_supply: self.total_supply,

            burn_address: dummy_addr,
        }
    }

    fn encode(contract: &Contract) -> CRgbContract {
        let mut new_contract = CRgbContract {
            title: [0; 256],
            issuance_utxo: CRgbOutPoint::encode(&contract.issuance_utxo.clone()),
            initial_owner_utxo: CRgbOutPoint::encode(&contract.initial_owner_utxo.clone()),
            network: CRgbBitcoinNetwork::encode(&contract.network),
            total_supply: contract.total_supply,
        };

        let cstr = CString::new(contract.title.clone()).unwrap();

        unsafe {
            libc::strcpy(&mut new_contract.title[0] as *mut c_char, cstr.as_ptr() as *mut i8);
        }

        new_contract
    }
}

// Contracts

#[no_mangle]
pub extern "C" fn rgb_contract_get_asset_id(contract: &CRgbContract, hash_buffer: &mut Sha256dHash) {
    *hash_buffer = contract.decode().get_asset_id();
}

#[no_mangle]
pub extern "C" fn rgb_contract_get_needed_txs(contract: &CRgbContract, needed_txs: &mut Box<[CRgbNeededTx]>) -> u32 {
    let needed_txs_native = contract.decode().get_needed_txs();
    let needed_txs_vec: Vec<CRgbNeededTx> = needed_txs_native
        .iter()
        .map(|ref x| CRgbNeededTx::encode(x))
        .collect();

    *needed_txs = needed_txs_vec
        .into_boxed_slice();

    needed_txs_native.len() as u32
}

#[no_mangle]
pub extern "C" fn rgb_contract_get_expected_script(contract: &CRgbContract, buffer: &mut Box<[u8]>) -> u32 {
    use bitcoin::network::serialize::serialize;

    let script = contract.decode().get_expected_script();
    let mut encoded: Vec<u8> = serialize(&script).unwrap();

    /* std::vec::Vec is encoded as <size>[...data...] by the consensus_encoding functions. This
       will result in invalid bitcoin scripts since the size would be interpreted as a first op_code.

       Theoretically this is a VarInt, but since commitment scripts are always much shorter than 256
       bytes, we can safely simply remove the first element, knowing that no other bytes from this
       field will remain */
    encoded.remove(0);

    let size = encoded.len();

    *buffer = encoded
        .into_boxed_slice();

    size as u32
}

#[no_mangle]
pub extern "C" fn rgb_contract_serialize(contract: &CRgbContract, buffer: &mut Box<[u8]>) -> u32 {
    use bitcoin::network::serialize::serialize;

    let encoded: Vec<u8> = serialize(&contract.decode()).unwrap();

    let size = encoded.len();

    *buffer = encoded
        .into_boxed_slice();

    size as u32
}

// TODO: properly handle results

#[no_mangle]
pub extern "C" fn rgb_contract_deserialize(buffer: *const c_uchar, len: u32, contract: &mut CRgbContract) {
    use bitcoin::network::serialize::deserialize;

    let sized_slice = unsafe { slice::from_raw_parts(buffer, len as usize) };

    let encoded: Vec<u8> = sized_slice.to_vec();

    let native_contract = deserialize(&encoded).unwrap();
    *contract = CRgbContract::encode(&native_contract);
}
