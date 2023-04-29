pub mod wallet;
pub mod keystore;
pub mod utils;
mod state_entity;

use bitcoin::Network;
use clap::{Parser, Subcommand};
use rand::{thread_rng, Rng};
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use serde_json::json;

use crate::wallet::Wallet;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new wallet, with a new random BIP 84 extended key
    CreateWallet { wallet_name: String },
    /// Get a new address
    GetNewAddress { wallet_name: String },
    /// Get a wallet balance
    GetBalance { wallet_name: String },
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct DepositMsg1 {
    pub auth: String,
    pub proof_key: String,
}

fn wallet_exists(wallet_name: &str) -> bool {
    let home_dir = dirs::home_dir();

    let mut path = home_dir.clone().unwrap();
    path.push(".statechain");
    path.push("wallets");
    path.push(format!("{}.json", wallet_name));

    std::path::Path::new(&path).exists()
}

fn create_wallet(wallet_name: &str, network: Network) {
    let mut seed = [0u8; 32];
    thread_rng().fill(&mut seed);
    let wallet = Wallet::new(&seed, wallet_name, network);
    wallet.save();
    println!("Wallet {} created: {}", wallet_name, wallet);
}

fn get_new_address(wallet_name: &String) {
    let mut wallet = Wallet::load(wallet_name).unwrap();
    let (address, index) = wallet.keys.get_new_address().unwrap();
    wallet.save();
    let obj = json!({"address": address.to_string(), "index": index});

    println!("{}", serde_json::to_string_pretty(&obj).unwrap());
}

fn get_balance(wallet_name: &String) {
    let mut wallet = Wallet::load(wallet_name).unwrap();

    let balance = wallet.get_all_addresses_balance();

    #[derive(Serialize, Deserialize, Debug)]
    struct Balance {
        address: String,
        balance: u64,
        unconfirmed_balance: i64,
    }

    let balance = balance
        .iter()
        .map(|(address, balance)| Balance {
            address: address.to_string(),
            balance: balance.confirmed,
            unconfirmed_balance: balance.unconfirmed,
        })
        .collect::<Vec<Balance>>();

    let obj = json!(balance);

    println!("{}", serde_json::to_string_pretty(&obj).unwrap());
}

fn main() {
    let cli = Cli::parse();
    let network = Network::Testnet;
    
    match &cli.command {
        Commands::CreateWallet { wallet_name } => {
            if wallet_exists(wallet_name) {
                println!("Wallet {} already exists", wallet_name);
                return;
            }
            create_wallet(wallet_name, network);
        }
        Commands::GetNewAddress { wallet_name } => {
            get_new_address(wallet_name);
        },
        Commands::GetBalance { wallet_name } => {
            get_balance(wallet_name);
        },
    }

    println!("Hello, world!");


}
