use async_trait::async_trait;
use ethers::types::Address;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum ClientError {
    #[error("Network '{0}' is not supported")]
    NetworkNotSupported(String),
    #[error("Unable to decode server response: {0}")]
    MalformedResponse(String),
    #[error("RPC error: {0:?}")]
    // RpcError(RpcFailure),
    // #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Provided account credentials are incorrect")]
    IncorrectCredentials,
    #[error("Seed too short, must be at least 32 bytes long")]
    SeedTooShort,
    #[error("Token is not supported by zkSync")]
    UnknownToken,
    #[error("Incorrect address")]
    IncorrectAddress,

    #[error("Operation timeout")]
    OperationTimeout,
    #[error("Polling interval is too small")]
    PollingIntervalIsTooSmall,

    #[error("Signing error: {0}")]
    // SigningError(SignerError),
    // #[error("Missing required field for a transaction: {0}")]
    MissingRequiredField(String),

    #[error("Rootstock private key was not provided for this wallet")]
    NoEthereumPrivateKey,

    #[error("Provided value is not packable")]
    NotPackableValue,

    #[error("Provided function arguments are incorrect")]
    IncorrectInput,

    #[error("Other")]
    Other,
}


pub type ResponseResult<T> = Result<T, ClientError>;

#[async_trait]
/// `Provider` used to connect to zkSync network in order to send transactions
/// and retrieve some information from the server about
/// zkSync accounts, transactions, supported tokens and the like.
pub trait Provider {
    /// Requests and returns information about a ZKSync account given its address.
    async fn account_info(&self, address: Address) -> ResponseResult<AccountInfo>;

    /// Requests and returns a list of tokens supported by zkSync.
    async fn tokens(&self) -> ResponseResult<Tokens>;

    /// Requests and returns information about transaction execution status.
    async fn tx_info(&self, tx_hash: TxHash) -> ResponseResult<TransactionInfo>;

    /// Obtains minimum fee required to process transaction in zkSync network.
    async fn get_tx_fee(
        &self,
        tx_type: TxFeeTypes,
        address: Address,
        token: impl Into<TokenLike> + Send + 'async_trait,
    ) -> ResponseResult<Fee>;

    /// Obtains minimum fee required to process transactions batch in zkSync network.
    async fn get_txs_batch_fee(
        &self,
        tx_types: Vec<TxFeeTypes>,
        addresses: Vec<Address>,
        token: impl Into<TokenLike> + Send + 'async_trait,
    ) -> ResponseResult<BigUint>;

    /// Requests and returns information about an Rootstock operation given its `serial_id`.
    async fn ethop_info(&self, serial_id: u32) -> ResponseResult<EthOpInfo>;

    /// Requests and returns Rootstock withdrawal transaction hash for some offchain withdrawal.
    async fn get_eth_tx_for_withdrawal(
        &self,
        withdrawal_hash: TxHash,
    ) -> ResponseResult<Option<String>>;

    /// Requests and returns a smart contract address (for Rootstock network associated with network specified in `Provider`).
    async fn contract_address(&self) -> ResponseResult<ContractAddress>;

    /// Submits a transaction to the zkSync network.
    /// Returns the hash of the created transaction.
    async fn send_tx(
        &self,
        tx: ZkSyncTx,
        eth_signature: Option<PackedEthSignature>,
    ) -> ResponseResult<TxHash>;

    /// Submits a batch of transactions to the zkSync network.
    /// Returns the hashes of the created transactions.
    async fn send_txs_batch(
        &self,
        txs_signed: Vec<(ZkSyncTx, Option<PackedEthSignature>)>,
        eth_signature: Option<PackedEthSignature>,
    ) -> ResponseResult<Vec<TxHash>>;

    /// Type of network this provider is allowing access to.
    fn network(&self) -> Network;
}

