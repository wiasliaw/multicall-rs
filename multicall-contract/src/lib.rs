include!(concat!(env!("OUT_DIR"), "/multicall.rs"));
include!(concat!(env!("OUT_DIR"), "/multicall2.rs"));

pub use multicall as multicall1;
pub use multicall_2 as multicall2;
