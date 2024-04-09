use {
    anyhow::{anyhow, Result},
    borsh::BorshDeserialize,
    clap::{command, Parser, Subcommand},
    presale::state::PresaleState,
    serde::Deserialize,
    solana_cli_config,
    solana_client::rpc_client::RpcClient,
    solana_sdk::{
        commitment_config::CommitmentConfig,
        instruction::Instruction,
        pubkey::Pubkey,
        signature::{read_keypair_file, Signer},
        transaction::Transaction,
    },
    std::{fs, str::FromStr},
};

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    config: Option<String>,
    #[arg(short, long)]
    rpc: Option<String>,
    #[arg(short, long)]
    payer: Option<String>,
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    Tokenize { amount: u64 },
    Redeem { amount: u64 },
    RedeemPt { amount: u64 },
    Claim,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let solana_config_file = if let Some(ref config) = *solana_cli_config::CONFIG_FILE {
        solana_cli_config::Config::load(config).unwrap_or_default()
    } else {
        solana_cli_config::Config::default()
    };

    let wallet_keypair =
        read_keypair_file(solana_config_file.keypair_path).expect("Can't open file-wallet");
    let wallet_pubkey = wallet_keypair.pubkey();

    let client = RpcClient::new_with_commitment(
        solana_config_file.json_rpc_url.to_string(),
        CommitmentConfig::confirmed(),
    );

    let ix: Instruction;

    match args.commands {
        Commands::Init => unimplemented!(),
        Commands::Tokenize { amount } => unimplemented!(),
        Commands::Redeem { amount } => unimplemented!(),
        Commands::RedeemPt { amount } => unimplemented!(),
        Commands::Claim => unimplemented!(),
    }

    let mut tx = Transaction::new_with_payer(&[instruction], Some(&wallet_pubkey));
    let recent_blockhash = client
        .get_latest_blockhash()
        .expect("Cannot retrieve latest blockhash");
    tx.sign(&vec![&wallet_keypair], recent_blockhash);

    let id = client
        .send_and_confirm_transaction(&tx)
        .expect("Transaction failed");

    println!("{:?}", id);

    Ok(())
}
