extern crate kaleidoscope;

use std::slice;

use ::{CRgbAllocatedArray, CRgbAllocatedPtr};
use c_bitcoin::CRgbOutPoint;
use generics::WrapperOf;
use kaleidoscope::kaleidoscope_outpoint::CRgbKaleidoscopeOutpoint;
use proof::CRgbProof;

use self::kaleidoscope::tx_builder::spend_proofs;

pub mod address;
pub mod kaleidoscope_outpoint;

#[repr(C)]
pub struct CRgbProofTxPair {
    pub proof: CRgbAllocatedPtr<CRgbProof>,
    pub serialized_tx: CRgbAllocatedArray<u8>,
}

#[no_mangle]
pub extern "C" fn rgb_kaleidoscope_spend_proofs(
    input_proof_count: u32,
    input_proofs: *const CRgbProof,
    bitcoin_input_count: u32,
    bitcoin_inputs: *const CRgbOutPoint,
    output_count: u32,
    outputs: *const CRgbKaleidoscopeOutpoint,
) -> CRgbProofTxPair {
    let input_proofs = unsafe {
        slice::from_raw_parts(input_proofs, input_proof_count as usize)
            .iter()
            .map(|ref x| x.decode())
            .collect()
    };

    let bitcoin_inputs = unsafe {
        slice::from_raw_parts(bitcoin_inputs, bitcoin_input_count as usize)
            .iter()
            .map(|ref x| x.decode())
            .collect()
    };

    let outputs = unsafe {
        slice::from_raw_parts(outputs, output_count as usize)
            .iter()
            .map(|ref x| x.decode())
            .collect()
    };

    let (proof, tx) = spend_proofs(&input_proofs, &bitcoin_inputs, &outputs);

    use bitcoin::network::serialize::RawEncoder;
    use bitcoin::network::encodable::ConsensusEncodable;

    let encoded: Vec<u8> = Vec::new();
    let mut enc = RawEncoder::new(encoded);

    if let Err(e) = tx.consensus_encode(&mut enc) {
        panic!("Could not encode the newly generated transaction. Error: {:?}", e);
    }

    CRgbProofTxPair {
        proof: CRgbAllocatedPtr::new(CRgbProof::encode(&proof)),
        serialized_tx: CRgbAllocatedArray {
            ptr: enc.into_inner().into_boxed_slice()
        },
    }
}