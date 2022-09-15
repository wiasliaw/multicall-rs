use ethers::types::Chain;
use multicall_address::contract;

fn main() {
    println!("{:?}", contract("multicall2").unwrap().address(Chain::Mainnet).unwrap());
}
