use bitcoin::BitcoinHash;
use bitcoin::util::hash::Sha256dHash;
use c_bitcoin::CRgbOutPoint;
use contract::CRgbContract;
use CRgbNeededTx;
use generics::WrapperOf;
use rgb::contract::Contract;
use rgb::proof::OutputEntry;
use rgb::proof::Proof;
use rgb::traits::Verify;
use std::mem;
use std::os::raw::c_uchar;
use std::ptr;
use std::slice;

#[derive(Debug)]
#[repr(C)]
pub struct CRgbOutputEntry {
    pub asset_id: Sha256dHash,
    pub amount: u32,
    pub vout: u32,
}

impl WrapperOf<OutputEntry> for CRgbOutputEntry {
    fn decode(&self) -> OutputEntry {
        OutputEntry::new(self.asset_id.clone(), self.amount, self.vout)
    }

    fn encode(native: &OutputEntry) -> Self {
        CRgbOutputEntry {
            asset_id: native.get_asset_id(),
            amount: native.get_amount(),
            vout: native.get_vout(),
        }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct CRgbProof {
    pub bind_to_count: u32,
    pub bind_to: *mut CRgbOutPoint,
    pub input_count: u32,
    pub input: *mut CRgbProof,
    pub output_count: u32,
    pub output: *mut CRgbOutputEntry,
    pub contract: *mut CRgbContract,
}

impl WrapperOf<Proof> for CRgbProof {
    fn decode(&self) -> Proof {
        let bind_to_vec = unsafe {
            slice::from_raw_parts(self.bind_to, self.bind_to_count as usize)
                .iter()
                .map(|ref x| x.decode())
                .collect()
        };

        let input_vec = unsafe {
            slice::from_raw_parts(self.input, self.input_count as usize)
                .iter()
                .map(|ref x| x.decode())
                .collect()
        };

        let output_vec = unsafe {
            slice::from_raw_parts(self.output, self.output_count as usize)
                .iter()
                .map(|ref x| x.decode())
                .collect()
        };

        let contract: Option<Box<Contract>> = match self.contract as usize {
            0 => None,
            _ => unsafe {
                Some(Box::new((*self.contract).decode()))
            }
        };

        Proof {
            bind_to: bind_to_vec,
            input: input_vec,
            output: output_vec,
            contract,
        }
    }

    fn encode(native: &Proof) -> Self {
        // bind_to
        let mut bind_to_vec: Vec<CRgbOutPoint> = native.bind_to
            .iter()
            .map(|ref x| CRgbOutPoint::encode(x))
            .collect();
        bind_to_vec.shrink_to_fit();

        let bind_to_count = bind_to_vec.len();
        let bind_to = bind_to_vec.as_mut_ptr();
        mem::forget(bind_to_vec);

        // input
        let mut input_vec: Vec<CRgbProof> = native.input
            .iter()
            .map(|ref x| CRgbProof::encode(x))
            .collect();
        input_vec.shrink_to_fit();

        let input_count = input_vec.len();
        let input = input_vec.as_mut_ptr();
        mem::forget(input_vec);

        // output
        let mut output_vec: Vec<CRgbOutputEntry> = native.output
            .iter()
            .map(|ref x| CRgbOutputEntry::encode(x))
            .collect();
        output_vec.shrink_to_fit();

        let output_count = output_vec.len();
        let output = output_vec.as_mut_ptr();
        mem::forget(output_vec);

        // contract
        let contract = match native.contract {
            None => ptr::null_mut(),
            Some(ref f) => Box::into_raw(Box::new(CRgbContract::encode(f)))
        };

        CRgbProof {
            bind_to_count: bind_to_count as u32,
            bind_to,
            input_count: input_count as u32,
            input,
            output_count: output_count as u32,
            output,
            contract,
        }
    }
}


#[no_mangle]
pub extern "C" fn rgb_proof_is_root_proof(proof: &CRgbProof) -> u8 {
    match proof.decode().is_root_proof() {
        true => 1,
        false => 0
    }
}

#[no_mangle]
pub extern "C" fn rgb_proof_get_needed_txs(proof: &CRgbProof, needed_txs: &mut Box<[CRgbNeededTx]>) -> u32 {
    let needed_txs_native = proof.decode().get_needed_txs();
    let needed_txs_vec: Vec<CRgbNeededTx> = needed_txs_native
        .iter()
        .map(|ref x| CRgbNeededTx::encode(x))
        .collect();

    /* A little recap of what's going on here:
     *
     * The reason why we can't directly assign *needed_txs = vec.into_boxed_slice() is that
     * Rust would first try to de-allocate what's inside needed_txs by calling drop() on it.
     * Since we have no control over what's coming from the C side, and it's very likely that
     * needed_txs is an uninitialized variable, this would make Rust panic.
     *
     * The solution here is to use mem::swap, which does not call drop on either of the elements
     * being swapped. Then we mem::forget the uninitialized pointer, so that Rust will not try to
     * drop it once it gets out of scope.
     *
     * So we create a "dummy" object (which actually contains our data). We swap the pointer coming
     * from the C side with our dummy array (so now the uninitialized pointer is in `dummy`) and
     * finally we forget dummy.
     */

    let mut dummy = needed_txs_vec.into_boxed_slice();
    mem::swap(&mut dummy, &mut *needed_txs);
    mem::forget(dummy);

    needed_txs_native.len() as u32
}

#[no_mangle]
pub extern "C" fn rgb_proof_get_expected_script(proof: &CRgbProof, buffer: &mut Box<[u8]>) -> u32 {
    use bitcoin::network::serialize::serialize;

    let script = proof.decode().get_expected_script();
    let mut encoded: Vec<u8> = serialize(&script).unwrap();

    /* std::vec::Vec is encoded as <size>[...data...] by the consensus_encoding functions. This
       will result in invalid bitcoin scripts since the size would be interpreted as a first op_code.

       Theoretically this is a VarInt, but since commitment scripts are always much shorter than 256
       bytes, we can safely simply remove the first element, knowing that no other bytes from this
       field will remain */
    encoded.remove(0);

    let size = encoded.len();

    // Same swap trick described above in `rgb_proof_get_needed_txs`
    let mut dummy = encoded.into_boxed_slice();
    mem::swap(&mut dummy, &mut *buffer);
    mem::forget(dummy);

    size as u32
}

#[no_mangle]
pub extern "C" fn rgb_proof_get_serialized_size(proof: &CRgbProof) -> u32 {
    use bitcoin::network::serialize::serialize;

    let encoded: Vec<u8> = serialize(&proof.decode()).unwrap();

    encoded.len() as u32
}

#[no_mangle]
pub extern "C" fn rgb_proof_serialize(proof: &CRgbProof, buffer: &mut Box<[u8]>) -> u32 {
    use bitcoin::network::serialize::serialize;

    let encoded: Vec<u8> = serialize(&proof.decode()).unwrap();

    let size = encoded.len();

    // Same swap trick described above in `rgb_proof_get_needed_txs`
    let mut dummy = encoded.into_boxed_slice();
    mem::swap(&mut dummy, &mut *buffer);
    mem::forget(dummy);

    size as u32
}

#[no_mangle]
pub extern "C" fn rgb_proof_deserialize(buffer: *const c_uchar, len: u32, proof: &mut CRgbProof) {
    use bitcoin::network::serialize::deserialize;

    let sized_slice = unsafe { slice::from_raw_parts(buffer, len as usize) };
    let encoded: Vec<u8> = sized_slice.to_vec();

    let native_proof = deserialize(&encoded).unwrap();

    // Same swap trick described above in `rgb_proof_get_needed_txs`
    let mut dummy = CRgbProof::encode(&native_proof);
    mem::swap(&mut dummy, &mut *proof);
    mem::forget(dummy);
}