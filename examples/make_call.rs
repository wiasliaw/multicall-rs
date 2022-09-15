use ethers::{prelude::*, utils::Anvil};
use std::{convert::TryFrom, sync::Arc, time::Duration};

use multicall_address::contract;
use multicall_contract::multicall2;

abigen!(
    IERC20,
    r#"[
        function totalSupply() public view returns (uint256)
    ]"#,
);

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // mainnet address
    let multicall_address = contract("multicall2").unwrap().address(Chain::Mainnet).unwrap();

    // anvil
    let anvil = Anvil::new()
        .fork("https://rpc.ankr.com/eth")
        .spawn();

    // wallet
    let wallet: LocalWallet = anvil.keys()[0].clone().into();

    // provider
    let provider = Provider::<Http>::try_from(anvil.endpoint()).unwrap()
        .interval(Duration::from_millis(10u64));

    // client
    let client = SignerMiddleware::new(provider.clone(), wallet.with_chain_id(1u32));

    // multicall
    let multicall2 = multicall2::Multicall2::new(multicall_address, Arc::new(client.clone()));
    let weth: Address = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2".parse()?;
    let usdc: Address = "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48".parse()?;

    let mut call_vec = Vec::new();
    // weth totalSupply
    call_vec.push(multicall2::multicall_2::Call {
        target: weth,
        call_data: "0x18160ddd".parse::<ethers::types::Bytes>().unwrap(),
    });
    // usdc totalSupply
    call_vec.push(multicall2::multicall_2::Call {
        target: usdc,
        call_data: "0x18160ddd".parse::<ethers::types::Bytes>().unwrap(),
    });

    let tx = multicall2.aggregate(call_vec).send()
        .await.unwrap()
        .await.unwrap()
        .unwrap();

    println!("{:?}", tx.transaction_hash);
    Ok(())
}
