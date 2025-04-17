//! Documentation about the keystore database structure.
//!
//! The database structure is managed by [`Database::open`], which applies migrations
//! (defined in [`migrations`]) that produce the current structure.
//!
//! The SQL code in this module's constants encodes the current database structure, as
//! represented internally by SQLite. We do not use these constants at runtime; instead we
//! check the output of the migrations in a test, to pin the expected database structure.
//!
//! [`Database::open`]: crate::components::database::Database::open

// The constants in this module are only used in tests, but `#[cfg(test)]` prevents them
// from showing up in `cargo doc --document-private-items`.
#![allow(dead_code)]

pub(in crate::components) mod migrations;

/// Stores the age recipients for the wallet's identity file.
///
/// ### Columns
///
/// - `recipient` is the string encoding of an age recipient.
/// - `added`: The time at which the recipient was added to the wallet, as a string in the
///   format `yyyy-MM-dd HH:mm:ss.fffffffzzz`.
pub(crate) const TABLE_AGE_RECIPIENTS: &str = r#"
CREATE TABLE ext_zallet_keystore_age_recipients (
    recipient STRING NOT NULL,
    added TEXT NOT NULL
)
"#;

/// Stores encrypted mnemonic seed phrases.
///
/// We do not make any assertion as to whether the seed for a given fingerprint was
/// derived from the encrypted mnemonic using a [BIP 39 passphrase] other than the empty
/// string.
///
/// ### Columns
///
/// - `hd_seed_fingerprint` is the [ZIP 32 fingerprint] for the seed derived from a
///   [BIP 39 mnemonic phrase]. This is present to enable quick lookups of which mnemonic
///   needs to be decrypted at spend time (rather than trial-decrypting every mnemonic).
/// - `encrypted_mnemonic` is a [BIP 39 mnemonic phrase] in an [age encrypted file].
///
/// [ZIP 32 fingerprint]: https://zips.z.cash/zip-0032#seed-fingerprints
/// [BIP 39 mnemonic phrase]: https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki
/// [BIP 39 passphrase]: https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki#from-mnemonic-to-seed
/// [age encrypted file]: https://c2sp.org/age#encrypted-file-format
pub(crate) const TABLE_MNEMONICS: &str = r#"
CREATE TABLE ext_zallet_keystore_mnemonics (
    hd_seed_fingerprint BLOB NOT NULL UNIQUE,
    encrypted_mnemonic BLOB NOT NULL
)
"#;
