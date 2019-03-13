use std::any::TypeId;

#[derive(Debug)]
#[repr(C)]
pub struct CRgbVec<V: 'static> {
    pub vec: Box<Vec<V>>,
    _v_type: TypeId,
}

impl<V: 'static> CRgbVec<V> {
    pub fn new() -> CRgbVec<V> {
        let typed_vec: Vec<V> = Vec::new();

        CRgbVec {
            vec: Box::new(typed_vec),
            _v_type: TypeId::of::<V>(),
        }
    }

    pub fn type_check(&self) -> bool {
        self._v_type == TypeId::of::<V>()
    }

    fn panic_failed_typecheck(&self) {
        panic!(
            "Typecheck failed for vec: V is {:?}, expected {:?}",
            TypeId::of::<V>(),
            self._v_type
        );
    }

    pub fn decode(&self) -> &Vec<V> {
        if !self.type_check() {
            self.panic_failed_typecheck();
        }

        &self.vec
    }

    pub fn decode_mut(&mut self) -> &mut Vec<V> {
        if !self.type_check() {
            self.panic_failed_typecheck();
        }

        &mut self.vec
    }
}