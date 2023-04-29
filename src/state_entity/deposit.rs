use crate::wallet::Wallet;


/// Message to server initiating state entity protocol.
/// Shared wallet ID returned
pub fn session_init(wallet: &mut Wallet, proof_key: &String) -> Result<UserID> {
    requests::postb(
        &wallet.client_shim,
        &format!("deposit/init"),
        &DepositMsg1 {
            auth: "auth".to_string(),
            proof_key: proof_key.to_owned(),
        },
    )
}