use std::time::UNIX_EPOCH;

use documented::Documented;
use jsonrpsee::{core::RpcResult, tracing::warn};
use schemars::JsonSchema;
use serde::Serialize;

use crate::components::keystore::KeyStore;

/// Response to a `getwalletinfo` RPC request.
pub(crate) type Response = RpcResult<ResultType>;
pub(crate) type ResultType = GetWalletInfo;

/// The wallet state information.
#[derive(Clone, Debug, Serialize, Documented, JsonSchema)]
pub(crate) struct GetWalletInfo {
    /// The wallet version, in its "Bitcoin client version" form.
    walletversion: u64,

    /// The total confirmed transparent balance of the wallet in ZEC.
    balance: f64,

    /// The total unconfirmed transparent balance of the wallet in ZEC.
    ///
    /// Not included if `asOfHeight` is specified.
    unconfirmed_balance: Option<f64>,

    /// The total immature transparent balance of the wallet in ZEC.
    immature_balance: f64,

    /// The total confirmed shielded balance of the wallet in ZEC.
    shielded_balance: String,

    /// The total unconfirmed shielded balance of the wallet in ZEC.
    ///
    /// Not included if `asOfHeight` is specified.
    shielded_unconfirmed_balance: Option<String>,

    /// The total number of transactions in the wallet
    txcount: u64,

    /// The timestamp (seconds since GMT epoch) of the oldest pre-generated key in the
    /// key pool.
    keypoololdest: u64,

    /// How many new keys are pre-generated.
    keypoolsize: u32,

    /// The timestamp in seconds since epoch (midnight Jan 1 1970 GMT) that the wallet is
    /// unlocked for transfers, or 0 if the wallet is locked.
    #[serde(skip_serializing_if = "Option::is_none")]
    unlocked_until: Option<u64>,

    /// The BLAKE2b-256 hash of the HD seed derived from the wallet's emergency recovery phrase.
    mnemonic_seedfp: String,
}

/// Defines the method parameters for OpenRPC.
pub(super) fn params(_: &mut super::openrpc::Generator) -> Vec<super::openrpc::ContentDescriptor> {
    vec![]
}

pub(crate) async fn call(keystore: &KeyStore) -> Response {
    warn!("TODO: Implement getwalletinfo");

    let unlocked_until = if keystore.uses_encrypted_identities() {
        Some(
            keystore
                .unlocked_until()
                .await
                .map(|i| i.duration_since(UNIX_EPOCH).expect("valid").as_secs())
                .unwrap_or(0),
        )
    } else {
        None
    };

    Ok(GetWalletInfo {
        walletversion: 0,
        balance: 0.0,
        unconfirmed_balance: Some(0.0),
        immature_balance: 0.0,
        shielded_balance: "0.00".into(),
        shielded_unconfirmed_balance: Some("0.00".into()),
        txcount: 0,
        keypoololdest: 0,
        keypoolsize: 0,
        unlocked_until,
        mnemonic_seedfp: "TODO".into(),
    })
}
