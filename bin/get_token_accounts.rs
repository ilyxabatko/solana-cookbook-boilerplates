use solana_account_decoder::UiAccountEncoding;
use solana_client::{
    rpc_client::RpcClient,
    rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
    rpc_filter::{Memcmp, MemcmpEncodedBytes, RpcFilterType},
};
use solana_sdk::{commitment_config::CommitmentConfig, program_pack::Pack};
use spl_token::state::{Account, Mint};

fn main() {
    const MY_WALLET_ADDRESS: &str = "SDCcPraNtvK4XPk5XASqYExWyEPrH9YAnEwm6Hcuz3U";

    let rpc_url = String::from("http://api.devnet.solana.com");
    let connection = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    let filters = Some(vec![
        RpcFilterType::Memcmp(Memcmp {
            offset: 32,
            bytes: MemcmpEncodedBytes::Base58(MY_WALLET_ADDRESS.to_string()),
            encoding: None,
        }),
        RpcFilterType::DataSize(165),
    ]);

    let accounts = connection
        .get_program_accounts_with_config(
            &spl_token::ID,
            RpcProgramAccountsConfig {
                filters,
                account_config: RpcAccountInfoConfig {
                    encoding: Some(UiAccountEncoding::Base64),
                    commitment: Some(connection.commitment()),
                    ..RpcAccountInfoConfig::default()
                },
                ..RpcProgramAccountsConfig::default()
            },
        )
        .unwrap();

    println!(
        "Found {:?} token account(s) for wallet {MY_WALLET_ADDRESS}: ",
        accounts.len()
    );

    for (i, account) in accounts.iter().enumerate() {
        println!("-- Token Account Address {:?}:  {:?} --", i, account.0);

        let mint_token_account = Account::unpack_from_slice(account.1.data.as_slice()).unwrap();
        println!("Mint: {:?}", mint_token_account.mint);

        let mint_account_data = connection
            .get_account_data(&mint_token_account.mint)
            .unwrap();
        let mint = Mint::unpack_from_slice(mint_account_data.as_slice()).unwrap();
        println!(
            "Amount: {:?}",
            mint_token_account.amount as f64 / 10usize.pow(mint.decimals as u32) as f64
        );
    }
}
