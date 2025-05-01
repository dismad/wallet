use age::secrecy::SecretString;
use documented::Documented;
use jsonrpsee::core::RpcResult;
use schemars::JsonSchema;
use serde::Serialize;

use crate::components::{json_rpc::server::LegacyCode, keystore::KeyStore};

/// Response to a `walletpassphrase` RPC request.
pub(crate) type Response = RpcResult<ResultType>;

/// Empty result indicating success.
#[derive(Clone, Debug, Serialize, Documented, JsonSchema)]
#[serde(transparent)]
pub(crate) struct ResultType(());

/// Defines the method parameters for OpenRPC.
pub(super) fn params(g: &mut super::openrpc::Generator) -> Vec<super::openrpc::ContentDescriptor> {
    vec![
        g.param::<String>(
            "passphrase",
            "The passphrase for decrypting the wallet's age identity.",
            true,
        ),
        g.param::<u64>(
            "timeout",
            "Time in seconds after which the wallet wil relock.",
            true,
        ),
    ]
}

pub(crate) async fn call(keystore: &KeyStore, passphrase: SecretString, timeout: u64) -> Response {
    if !keystore.uses_encrypted_identities() {
        return Err(LegacyCode::WalletWrongEncState.with_static(
            "Error: running with an unencrypted wallet, but walletpassphrase was called.",
        ));
    }

    if !keystore.unlock(passphrase, timeout).await {
        return Err(LegacyCode::WalletPassphraseIncorrect
            .with_static("Error: The wallet passphrase entered was incorrect."));
    }

    Ok(ResultType(()))
}
