mod counter;
mod simple_account;

use alloy::primitives::U256;
use alloy::providers::ext::AnvilApi;
use alloy::{
    network::{ReceiptResponse, TransactionBuilder},
    providers::{Provider, ProviderBuilder},
};
use eyre::Result;
use crate::simple_account::SimpleAccount;

#[tokio::main]
async fn main() -> Result<()> {
    let address = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266";
    let provider = ProviderBuilder::new()
        .on_anvil_with_wallet();
    provider.anvil_set_nonce(address.parse()?, 0).await?;

    let contract = SimpleAccount::deploy(&provider).await?;

    println!("Deployed contract at address: {}", contract.address());

    let builder = contract.setNumber(U256::from(42));
    let tx_hash = builder.send().await?.watch().await?;

    println!("Set number to 42: {tx_hash}");

    let latest_block = provider.get_block_number().await?;
    Ok(())
}
