use sp_core::sr25519::Pair;
use sp_core::H256;
use subxt::{tx::PairSigner, Error, OnlineClient, PolkadotConfig};

#[subxt::subxt(runtime_metadata_path = "artifacts/devnet.scale")]
pub mod devnet {
    #[subxt(substitute_type = "frame_support::storage::bounded_vec::BoundedVec")]
    use ::sp_std::vec::Vec;
}
use devnet::runtime_types::pallet_smart_contract::types::Contract;
use devnet::runtime_types::pallet_tfgrid::twin::TwinIp;
use devnet::runtime_types::pallet_tfgrid::types::Twin;

pub struct TfchainClient {
    pub pair: Pair,
    pub api: OnlineClient<PolkadotConfig>,
}

pub async fn new_substrate_client(url: String, pair: Pair) -> Result<TfchainClient, Error> {
    let api = OnlineClient::<PolkadotConfig>::from_url(url).await?;

    Ok(TfchainClient { pair, api })
}

impl TfchainClient {
    pub async fn create_twin(&self, ip: String) -> Result<H256, Error> {
        let create_twin_tx = devnet::tx()
            .tfgrid_module()
            .create_twin(ip.as_bytes().to_vec());
        let signer = PairSigner::new(self.pair.clone());
        self.api
            .tx()
            .sign_and_submit_default(&create_twin_tx, &signer)
            .await
    }

    pub async fn sign_terms_and_conditions(
        &self,
        document_link: String,
        document_hash: String,
    ) -> Result<H256, Error> {
        let create_twin_tx = devnet::tx().tfgrid_module().user_accept_tc(
            document_link.as_bytes().to_vec(),
            document_hash.as_bytes().to_vec(),
        );
        let signer = PairSigner::new(self.pair.clone());
        self.api
            .tx()
            .sign_and_submit_default(&create_twin_tx, &signer)
            .await
    }

    pub async fn get_twin_by_id(
        &self,
        id: u32,
    ) -> Result<Twin<TwinIp, sp_core::crypto::AccountId32>, Error> {
        let twin = devnet::storage().tfgrid_module().twins(id);
        let t = self.api.storage().fetch(&twin, None).await?.unwrap();
        println!("twin: {:?}", t);

        Ok(t)
    }

    pub async fn get_contract_by_id(&self, id: u64) -> Result<Contract, Error> {
        let contract = devnet::storage().smart_contract_module().contracts(id);
        let c = self.api.storage().fetch(&contract, None).await?.unwrap();

        println!("contract: {:?}", c);

        Ok(c)
    }
}
