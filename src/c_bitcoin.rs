use bitcoin::network::constants::Network;
use bitcoin::OutPoint;
use bitcoin::util::hash::Sha256dHash;
use generics::WrapperOf;

#[derive(Debug)]
#[repr(C)]
pub enum CRgbBitcoinNetwork {
    Mainnet,
    Testnet,
    Regtest,
}

impl WrapperOf<Network> for CRgbBitcoinNetwork {
    fn decode(&self) -> Network {
        match self {
            CRgbBitcoinNetwork::Mainnet => Network::Bitcoin,
            CRgbBitcoinNetwork::Testnet => Network::Testnet,
            CRgbBitcoinNetwork::Regtest => Network::Regtest
        }
    }

    fn encode(native: &Network) -> Self {
        match native {
            Network::Bitcoin => CRgbBitcoinNetwork::Mainnet,
            Network::Testnet => CRgbBitcoinNetwork::Testnet,
            Network::Regtest => CRgbBitcoinNetwork::Regtest
        }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct CRgbOutPoint {
    pub txid: Sha256dHash,
    pub vout: u32,
}

impl WrapperOf<OutPoint> for CRgbOutPoint {
    fn decode(&self) -> OutPoint {
        OutPoint {
            txid: self.txid,
            vout: self.vout,
        }
    }

    fn encode(op: &OutPoint) -> CRgbOutPoint {
        CRgbOutPoint {
            txid: op.txid,
            vout: op.vout,
        }
    }
}