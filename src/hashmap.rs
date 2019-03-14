use std::any::TypeId;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug)]
#[repr(C)]
pub struct CRgbHashMap<K: 'static + Eq + Hash, V: 'static> {
    pub map: Box<HashMap<K, V>>,
    _k_type: TypeId,
    _v_type: TypeId,
}

impl<K: 'static + Eq + Hash, V: 'static> CRgbHashMap<K, V> {
    pub fn new() -> CRgbHashMap<K, V> {
        let typed_map: HashMap<K, V> = HashMap::new();

        CRgbHashMap {
            map: Box::new(typed_map),
            _k_type: TypeId::of::<K>(),
            _v_type: TypeId::of::<V>(),
        }
    }

    pub fn from_native(map: HashMap<K, V>) -> CRgbHashMap<K, V> {
        CRgbHashMap {
            map: Box::new(map),
            _k_type: TypeId::of::<K>(),
            _v_type: TypeId::of::<V>(),
        }
    }

    pub fn type_check(&self) -> bool {
        self._k_type == TypeId::of::<K>() && self._v_type == TypeId::of::<V>()
    }

    fn panic_failed_typecheck(&self) {
        panic!(
            "Typecheck failed for hashmap: K is {:?}, expected {:?}. V is {:?}, expected {:?}",
            TypeId::of::<K>(),
            self._k_type,
            TypeId::of::<V>(),
            self._v_type
        );
    }

    pub fn decode(&self) -> &HashMap<K, V> {
        if !self.type_check() {
            self.panic_failed_typecheck();
        }

        &self.map
    }

    pub fn decode_mut(&mut self) -> &mut HashMap<K, V> {
        if !self.type_check() {
            self.panic_failed_typecheck();
        }

        &mut self.map
    }
}