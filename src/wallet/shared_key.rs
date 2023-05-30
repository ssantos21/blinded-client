use kms::ecdsa::two_party::MasterKey2;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::{shared::structs::PrepareSignTxMsg};

#[derive(Serialize, Deserialize)]
pub struct SharedKey {
    pub id: Uuid,
    pub share: MasterKey2,
    pub value: u64, //Satoshis
    pub statechain_id: Option<Uuid>,
    pub tx_backup_psm: Option<PrepareSignTxMsg>, // back up transaction data
    pub proof_key: Option<String>,
    // pub smt_proof: Option<InclusionProofSMT>,
    pub unspent: bool,
    pub funding_txid: String,
    pub previous_txs: Vec<String>, // chain of backup transactions (used in the blind version)
}

impl SharedKey {
    // pub fn new(
    //     id: &Uuid,
    //     client_shim: &ClientShim,
    //     secret_key: &SecretKey,
    //     value: &u64,
    //     protocol: Protocol,
    //     solution: String,
    // ) -> Result<SharedKey> {
    //     Self::new_repeat_keygen(id, client_shim, secret_key, value, protocol, solution, 0)
    // }

    // pub fn new_repeat_keygen(
    //     id: &Uuid,
    //     client_shim: &ClientShim,
    //     secret_key: &SecretKey,
    //     value: &u64,
    //     protocol: Protocol,
    //     solution: String,
    //     keygen_reps: u32
    // ) -> Result<SharedKey> {
    //     let mut key_share_priv: FE = ECScalar::zero(); // convert to curv lib
    //     key_share_priv.set_element(*secret_key);
    //     ecdsa::get_master_key_repeat_keygen(id, client_shim, &key_share_priv, value, protocol, solution, keygen_reps)
    // }

    // used in blind deposit where there is no InclusionProofSMT
    pub fn add_proof_key_and_funding_txid(
        &mut self,
        proof_key: &String,
        funding_txid: &String,
    ) {
        self.proof_key = Some(proof_key.to_owned());
        self.funding_txid = funding_txid.clone();
    }
}