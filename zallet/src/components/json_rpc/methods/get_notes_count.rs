use documented::Documented;
use jsonrpsee::core::RpcResult;
use schemars::JsonSchema;
use serde::Serialize;
use zcash_client_backend::data_api::{InputSource, NoteFilter, WalletRead};
use zcash_protocol::{ShieldedProtocol, value::Zatoshis};

use crate::components::{database::DbConnection, json_rpc::server::LegacyCode};

/// Response to a `z_getnotescount` RPC request.
pub(crate) type Response = RpcResult<ResultType>;
pub(crate) type ResultType = GetNotesCount;

/// The number of notes in the wallet.
#[derive(Clone, Debug, Serialize, Documented, JsonSchema)]
pub(crate) struct GetNotesCount {
    /// The number of Sprout notes in the wallet.
    ///
    /// Always zero, because Sprout is not supported.
    sprout: u32,

    /// The number of Sapling notes in the wallet.
    sapling: u32,

    /// The number of Orchard notes in the wallet.
    orchard: u32,
}

pub(super) const PARAM_MINCONF_DESC: &str =
    "Only include notes in transactions confirmed at least this many times.";
pub(super) const PARAM_AS_OF_HEIGHT_DESC: &str = "Execute the query as if it were run when the blockchain was at the height specified by this argument.";

pub(crate) fn call(
    wallet: &DbConnection,
    minconf: Option<u32>,
    as_of_height: Option<i32>,
) -> Response {
    // TODO: Switch to an approach that can respect `minconf` and `as_of_height`.
    if minconf.is_some() || as_of_height.is_some() {
        return Err(LegacyCode::InvalidParameter
            .with_static("minconf and as_of_height parameters are not yet supported"));
    }

    let selector = NoteFilter::ExceedsMinValue(Zatoshis::ZERO);

    let mut sapling = 0;
    let mut orchard = 0;
    for account_id in wallet
        .get_account_ids()
        .map_err(|e| LegacyCode::Database.with_message(e.to_string()))?
    {
        let account_metadata = wallet
            .get_account_metadata(account_id, &selector, &[])
            .map_err(|e| LegacyCode::Database.with_message(e.to_string()))?;

        if let Some(note_count) = account_metadata.note_count(ShieldedProtocol::Sapling) {
            sapling += note_count as u32;
        }
        if let Some(note_count) = account_metadata.note_count(ShieldedProtocol::Orchard) {
            orchard += note_count as u32;
        }
    }

    Ok(GetNotesCount {
        sprout: 0,
        sapling,
        orchard,
    })
}
