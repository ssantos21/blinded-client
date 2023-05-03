use sha3::Sha3_256;
use digest::Digest;

use crate::{wallet::Wallet, utils::{error::{CError, WalletErrorType}, helpers::FEE}};

use super::api::{get_statechain_fee_info, session_init};

/// Deposit coins into state entity. Returns shared_key_id, statechain_id, funding txid,
/// signed backup tx, back up transacion data and proof_key
pub fn deposit(
    wallet: &mut Wallet,
    amount: &u64,
) -> Result<(), CError> {
    // Get state entity fee info
    let se_fee_info = get_statechain_fee_info();

    // Ensure funds cover fees before initiating protocol
    if FEE + se_fee_info.deposit as u64 >= *amount {
        return Err(CError::WalletError(WalletErrorType::NotEnoughFunds));
    }

    //calculate SE fee amount from rate
    let deposit_fee = (amount * se_fee_info.deposit as u64) / 10000 as u64;
    let withdraw_fee = (amount * se_fee_info.withdraw as u64) / 10000 as u64;

    // Greedy coin selection.
    let (inputs, addrs, amounts) =
        wallet.coin_selection_greedy(&(amount + deposit_fee + FEE))?;

    // Generate proof key
    let proof_key = wallet.se_proof_keys.get_new_key()?;

    // Init. session - Receive shared wallet ID
    let shared_key_id = session_init(&proof_key.to_string());

    println!("shared_key_id: {:?}", shared_key_id);

    // generate solution for the PoW challenge
    let challenge = match shared_key_id.challenge {
        Some(c) => c,
        None => return Err(CError::Generic(String::from("missing pow challenge from server"))),
    };

    let difficulty = 4 as usize;
    let mut counter = 0;
    let zeros = String::from_utf8(vec![b'0'; difficulty]).unwrap();
    
    let mut hasher = Sha3_256::new();

    loop {
        // write input message
        hasher.update(&format!("{}:{:x}", challenge, counter).as_bytes());
        // read hash digest
        let result = hasher.finalize_reset();
        // convert hash result to hex string
        let result_str = hex::encode(result);
        // check if result has enough leading zeros
        if result_str[..difficulty] == zeros {
            break;
        };
        // increment counter and try again
        counter += 1

    }

    let solution = format!("{:x}", counter);

    println!("solution: {}", solution);

    Ok(())
}