use bitcoin::Transaction;
use CRgbNeededTx;
use CRgbSerializedTx;
use generics::WrapperOf;
use rgb::traits::NeededTx;
use std::collections::HashMap;
use std::mem;

#[no_mangle]
pub extern "C" fn rgb_init_needed_tx_map(map: &mut Box<HashMap<NeededTx, Transaction>>) {
    let mut new_map = Box::new(HashMap::new());
    mem::swap(&mut new_map, &mut *map);
    mem::forget(new_map);
}

#[no_mangle]
pub extern "C" fn rgb_push_needed_tx_map(map: &mut HashMap<NeededTx, Transaction>, key: &CRgbNeededTx, val: &CRgbSerializedTx) {
    map.insert(key.decode(), val.decode());
}

#[no_mangle]
pub extern "C" fn rgb_debug_print_needed_tx_map(map: &HashMap<NeededTx, Transaction>) {
    println!("{:#?}", map);
}