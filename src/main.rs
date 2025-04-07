// use clap::Parser;
use clap::{Parser, Subcommand};

/// Simple program to greet a person
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[clap(subcommand)]
    Bs58(Bs58Commands),
}

#[derive(Subcommand)]
pub enum Bs58Commands {
    #[clap()]
    Decode(DecodeArgs),
    #[clap()]
    Encode(EncodeArgs),
}

#[derive(Parser, Debug, Clone)]
#[command(version)]
pub struct DecodeArgs {
    #[arg(env)]
    pub input: String,
}

#[derive(Parser, Debug, Clone)]
#[command(version)]
pub struct EncodeArgs {
    #[arg(env)]
    pub input: String,
}

fn main() {
    let cli = Cli::parse();

    let result = match &cli.command {
        Commands::Bs58(Bs58Commands::Decode(args)) => decode(args),
        Commands::Bs58(Bs58Commands::Encode(args)) => encode(args),
    };

    if let Err(e) = result {
        println!("Error: {}", e);
        std::process::exit(1);
    }
}

pub fn decode(args: &DecodeArgs) -> Result<()> {
    let res = bs58::decode(&args.input)
        .into_vec()
        .map_err(|_| CliError::BS58Error)?;

    println!("{} decoded to {:?}", args.input, res);

    Ok(())
}
pub fn encode(args: &EncodeArgs) -> Result<()> {
    let res = bs58::encode(&args.input).into_string();

    println!("{} encoded to {:?}", res, args.input);

    Ok(())
}

use thiserror::Error;

pub type Result<T, E = CliError> = std::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("bs58 error")]
    BS58Error,

    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

use solana_client::rpc_client::RpcClient;
use solana_pubkey::Pubkey;
use solana_signature::Signature;
use solana_transaction_status::{EncodedConfirmedTransactionWithStatusMeta, UiTransactionEncoding};
use std::str::FromStr;

pub fn get_data() {
    let url = "https://api.mainnet-beta.solana.com".to_string();

    let client = RpcClient::new(url);

    let alice_pubkey = Pubkey::from_str("SPoo1Ku8WFXoNDMHPsrGSTSG1Y47rzgn41SLUNakuHy").unwrap();

    // Fetch recent transaction signatures
    let signatures = client
        .get_signatures_for_address(&alice_pubkey)
        .expect("Failed to get signatures");

    for signature_info in signatures.iter().take(10) {
        let tx = client
            .get_transaction(
                &Signature::from_str(&signature_info.signature).unwrap(),
                UiTransactionEncoding::JsonParsed,
            )
            .expect("Failed to get transaction");

        filter_transaction(&tx);
    }
}

fn filter_transaction(tx: &EncodedConfirmedTransactionWithStatusMeta) {
    unimplemented!()
}
