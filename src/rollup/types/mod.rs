use std::collections::HashMap;

use num::BigUint;
use serde::{Deserialize, Serialize};

// The declaration of the most primitive types used in zkSync network.
// Most of them are just re-exported from the `web3` crate.

#[macro_use]
mod basic_type;
pub mod serde_wrappers;
pub mod pubkey_hash;

use std::fmt;
use std::num::ParseIntError;
use std::ops::{Add, Deref, DerefMut, Sub};
use std::str::FromStr;

pub use ethers::types::{Address, Log, TransactionReceipt, H160, H256, U128, U256};

use self::pubkey_hash::PubKeyHash;
use self::serde_wrappers::{BigUintSerdeWrapper, BigUintSerdeAsRadix10Str};

basic_type!(
    /// Unique identifier of the token in the zkSync network.
    TokenId,
    u32
);

basic_type!(
    /// Unique identifier of the account in the zkSync network.
    AccountId,
    u32
);

basic_type!(
    /// zkSync network block sequential index.
    BlockNumber,
    u32
);

basic_type!(
    /// zkSync account nonce.
    Nonce,
    u32
);

basic_type!(
    /// Unique identifier of the priority operation in the zkSync network.
    PriorityOpId,
    u64
);

basic_type!(
    /// Block number in the Rootstock network.
    EthBlockId,
    u64
);

basic_type!(
    /// Unique identifier of the zkSync event.
    EventId,
    u64
);
basic_type!(
    /// Shared counter for L1 and L2  transactions
    /// This counter is used for total txs/priority ops ordering.
    /// It is required because we generally consider L1 and L2 operations different entities and
    /// store them separately.
    SequentialTxId,
    u64
);


/// Token supported in zkSync protocol
#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Token {
    /// id is used for tx signature and serialization
    pub id: TokenId,
    /// Contract address of ERC20 token or Address::zero() for "RBTC"
    pub address: Address,
    /// Token symbol (e.g. "RBTC" or "RDOC")
    pub symbol: String,
    /// Token precision (e.g. 18 for "RBTC" so "1.0" RBTC = 10e18 as U256 number)
    pub decimals: u8,
    pub kind: TokenKind,
    pub is_nft: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum TokenKind {
    ERC20,
    NFT,
    None,
}

impl Default for TokenKind {
    fn default() -> Self {
        Self::ERC20
    }
}

impl Token {
    pub fn new(id: TokenId, address: Address, symbol: &str, decimals: u8, kind: TokenKind) -> Self {
        Self {
            id,
            address,
            symbol: symbol.to_string(),
            decimals,
            kind,
            is_nft: matches!(kind, TokenKind::NFT),
        }
    }

    pub fn new_nft(id: TokenId, symbol: &str) -> Self {
        Self {
            id,
            address: Default::default(),
            symbol: symbol.to_string(),
            decimals: 0,
            kind: TokenKind::NFT,
            is_nft: true,
        }
    }
}

/// ERC-20 standard token.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenInfo {
    /// Address (prefixed with 0x)
    pub address: Address,
    /// Powers of 10 in 1.0 token (18 for default RBTC-like tokens)
    pub decimals: u8,
    /// Token symbol
    pub symbol: String,
}

impl TokenInfo {
    pub fn new(address: Address, symbol: &str, decimals: u8) -> Self {
        Self {
            address,
            symbol: symbol.to_string(),
            decimals,
        }
    }
}

pub type Tokens = HashMap<String, Token>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NFT {
    pub id: TokenId,
    pub symbol: String,
    pub creator_id: AccountId,
    pub content_hash: H256,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AccountState {
    pub balances: HashMap<String, BigUintSerdeWrapper>,
    pub nfts: HashMap<TokenId, NFT>,
    pub nonce: Nonce,
    pub pub_key_hash: PubKeyHash,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DepositingFunds {
    #[serde(with = "BigUintSerdeAsRadix10Str")]
    amount: BigUint,
    expected_accept_block: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DepositingAccountBalances {
    balances: HashMap<String, DepositingFunds>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BlockStatus {
    Committed,
    Verified,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountInfo {
    pub address: Address,
    pub id: Option<AccountId>,
    pub depositing: DepositingAccountBalances,
    pub committed: AccountState,
    pub verified: AccountState,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BlockInfo {
    pub block_number: i64,
    pub committed: bool,
    pub verified: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TransactionInfo {
    pub executed: bool,
    pub success: Option<bool>,
    pub fail_reason: Option<String>,
    pub block: Option<BlockInfo>,
}

impl TransactionInfo {
    /// Indicates whether this transaction is verified.
    pub fn is_verified(&self) -> bool {
        self.executed && self.block.as_ref().filter(|x| x.verified).is_some()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EthOpInfo {
    pub executed: bool,
    pub block: Option<BlockInfo>,
}

impl EthOpInfo {
    /// Indicates whether this operation is verified.
    pub fn is_verified(&self) -> bool {
        self.executed && self.block.as_ref().filter(|x| x.verified).is_some()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContractAddress {
    pub main_contract: String,
    pub gov_contract: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChangePubKeyFeeType {
    Onchain,
    ECDSA,
    CREATE2,
}

/// Type of the fee calculation pattern.
/// Unlike the `TxFeeTypes`, this enum represents the fee
/// from the point of zkSync view, rather than from the users
/// point of view.
/// Users do not divide transfers into `Transfer` and
/// `TransferToNew`, while in zkSync it's two different operations.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OutputFeeType {
    Transfer,
    TransferToNew,
    FastWithdraw,
    Withdraw,
    ChangePubKey(ChangePubKeyFeeType),
    MintNFT,
    WithdrawNFT,
    FastWithdrawNFT,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Fee {
    pub fee_type: OutputFeeType,
    #[serde(with = "BigUintSerdeAsRadix10Str")]
    pub gas_tx_amount: BigUint,
    #[serde(with = "BigUintSerdeAsRadix10Str")]
    pub gas_price_wei: BigUint,
    #[serde(with = "BigUintSerdeAsRadix10Str")]
    pub gas_fee: BigUint,
    #[serde(with = "BigUintSerdeAsRadix10Str")]
    pub zkp_fee: BigUint,
    #[serde(with = "BigUintSerdeAsRadix10Str")]
    pub total_fee: BigUint,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct BatchFee {
    #[serde(with = "BigUintSerdeAsRadix10Str")]
    pub total_fee: BigUint,
}
