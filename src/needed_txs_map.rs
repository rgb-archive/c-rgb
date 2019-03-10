use std::collections::HashMap;

use bitcoin::Transaction;
use rgb::traits::NeededTx;

use ::{CRgbAllocatedBox, CRgbNeededTx};
use CRgbSerializedTx;
use generics::WrapperOf;

#[no_mangle]
pub extern "C" fn rgb_init_needed_tx_map() -> CRgbAllocatedBox<HashMap<NeededTx, Transaction>> {
    CRgbAllocatedBox {
        ptr: vec![HashMap::new()].into_boxed_slice()
    }
}

#[no_mangle]
pub extern "C" fn rgb_push_needed_tx_map(map: &mut HashMap<NeededTx, Transaction>, key: &CRgbNeededTx, val: &CRgbSerializedTx) {
    map.insert(key.decode(), val.decode());
}

#[no_mangle]
pub extern "C" fn rgb_debug_print_needed_tx_map(map: &HashMap<NeededTx, Transaction>) {
    println!("{:#?}", map);
}