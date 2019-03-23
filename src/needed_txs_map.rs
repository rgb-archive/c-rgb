use std::collections::HashMap;

use bitcoin::Transaction;
use rgb::traits::NeededTx;

use ::{CRgbAllocatedArray, CRgbNeededTx};
use ::{CRgbAllocatedPtr, CRgbSerializedTx};
use generics::WrapperOf;

#[no_mangle]
pub extern "C" fn rgb_init_needed_tx_map() -> CRgbAllocatedPtr<HashMap<NeededTx, Transaction>> {
    CRgbAllocatedPtr {
        ptr: Box::new([HashMap::new()])
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