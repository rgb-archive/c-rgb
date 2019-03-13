use bitcoin::Transaction;
use rgb::traits::NeededTx;

use ::{CRgbAllocatedPtr, CRgbNeededTx, CRgbSerializedTx};
use generics::WrapperOf;
use hashmap::CRgbHashMap;

#[no_mangle]
pub extern "C" fn rgb_init_needed_tx_map() -> CRgbAllocatedPtr<CRgbHashMap<NeededTx, Transaction>> {
    CRgbAllocatedPtr {
        ptr: Box::new([CRgbHashMap::new()])
    }
}

#[no_mangle]
pub unsafe extern "C" fn rgb_push_needed_tx_map(map: &mut CRgbHashMap<NeededTx, Transaction>, key: &CRgbNeededTx, val: &CRgbSerializedTx) {
    map
        .decode_mut()
        .insert(key.decode(), val.decode());
}

#[no_mangle]
pub extern "C" fn rgb_debug_print_needed_tx_map(map: &CRgbHashMap<NeededTx, Transaction>) {
    println!("{:#?}", map);
}