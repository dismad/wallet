use std::path::PathBuf;

use clap::{Parser, builder::Styles};

#[cfg(outside_buildscript)]
use abscissa_core::{Command, Runnable};

use crate::fl;

#[derive(Debug, Parser)]
#[cfg_attr(outside_buildscript, derive(Command))]
#[command(author, about, version)]
#[command(help_template = format!("\
{{before-help}}{{about-with-newline}}
{}{}:{} {{usage}}

{{all-args}}{{after-help}}\
    ",
    Styles::default().get_usage().render(),
    fl!("usage-header"),
    Styles::default().get_usage().render_reset()))]
#[command(next_help_heading = fl!("flags-header"))]
pub struct EntryPoint {
    #[command(subcommand)]
    pub(crate) cmd: ZalletCmd,

    /// Enable verbose logging
    #[arg(short, long)]
    pub(crate) verbose: bool,

    /// Specify the data directory for the Zallet wallet.
    ///
    /// This must be an absolute path.
    #[arg(short, long)]
    pub(crate) datadir: Option<PathBuf>,

    /// Use the specified configuration file.
    ///
    /// Relative paths will be prefixed by the datadir.
    #[arg(short, long)]
    pub(crate) config: Option<PathBuf>,
}

#[derive(Debug, Parser)]
#[cfg_attr(outside_buildscript, derive(Command, Runnable))]
pub(crate) enum ZalletCmd {
    /// The `start` subcommand
    Start(StartCmd),

    /// Generate an example `zallet.toml` config.
    ExampleConfig(ExampleConfigCmd),

    /// Generate a `zallet.toml` config from an existing `zcash.conf` file.
    #[cfg(all(zallet_build = "wallet", feature = "zcashd-import"))]
    MigrateZcashConf(MigrateZcashConfCmd),

    /// Add the keys and transactions of a zcashd wallet.dat file to the wallet database.
    #[cfg(all(zallet_build = "wallet", feature = "zcashd-import"))]
    MigrateZcashdWallet(MigrateZcashdWalletCmd),

    /// Initialize wallet encryption.
    #[cfg(zallet_build = "wallet")]
    InitWalletEncryption(InitWalletEncryptionCmd),

    /// Generate a BIP 39 mnemonic phrase and store it in the wallet.
    #[cfg(zallet_build = "wallet")]
    GenerateMnemonic(GenerateMnemonicCmd),

    /// Import a BIP 39 mnemonic phrase into the wallet.
    #[cfg(zallet_build = "wallet")]
    ImportMnemonic(ImportMnemonicCmd),

    /// Communicate with a Zallet wallet's JSON-RPC interface.
    #[cfg(feature = "rpc-cli")]
    Rpc(RpcCliCmd),

    /// Commands for repairing broken wallet states.
    #[command(subcommand)]
    Repair(RepairCmd),
}

/// `start` subcommand
#[derive(Debug, Parser)]
#[cfg_attr(outside_buildscript, derive(Command))]
pub(crate) struct StartCmd {}

/// `example-config` subcommand
#[derive(Debug, Parser)]
#[cfg_attr(outside_buildscript, derive(Command))]
pub(crate) struct ExampleConfigCmd {
    /// Where to write the Zallet config file.
    ///
    /// - By default, the default Zallet config file path is used.
    /// - The value `-` will write the config to stdout.
    #[arg(short, long)]
    pub(crate) output: Option<String>,

    /// Force an existing Zallet config file to be overwritten.
    #[arg(short, long)]
    pub(crate) force: bool,

    /// Temporary flag ensuring any alpha users are aware the config is not stable.
    #[arg(long)]
    pub(crate) this_is_alpha_code_and_you_will_need_to_recreate_the_example_later: bool,
}

/// `migrate-zcash-conf` subcommand
#[cfg(all(zallet_build = "wallet", feature = "zcashd-import"))]
#[derive(Debug, Parser)]
#[cfg_attr(outside_buildscript, derive(Command))]
pub(crate) struct MigrateZcashConfCmd {
    /// Specify `zcashd` configuration file.
    ///
    /// Relative paths will be prefixed by `zcashd_datadir` location.
    #[arg(long, default_value = "zcash.conf")]
    pub(crate) conf: PathBuf,

    /// Specify `zcashd` data directory (this path cannot use '~').
    #[arg(long)]
    pub(crate) zcashd_datadir: Option<PathBuf>,

    /// Allow a migration when warnings are present.
    #[arg(long)]
    pub(crate) allow_warnings: bool,

    /// Where to write the Zallet config file.
    ///
    /// - By default, the default Zallet config file path is used.
    /// - The value `-` will write the config to stdout.
    #[arg(short, long)]
    pub(crate) output: Option<String>,

    /// Force an existing Zallet config file to be overwritten.
    #[arg(short, long)]
    pub(crate) force: bool,

    /// Temporary flag ensuring any alpha users are aware the migration is not stable.
    #[arg(long)]
    pub(crate) this_is_alpha_code_and_you_will_need_to_redo_the_migration_later: bool,
}

/// `migrate-zcashd-wallet` subcommand
#[cfg(all(zallet_build = "wallet", feature = "zcashd-import"))]
#[derive(Debug, Parser)]
#[cfg_attr(outside_buildscript, derive(Command))]
pub(crate) struct MigrateZcashdWalletCmd {
    /// Specify location of the `zcashd` `wallet.dat` file.
    ///
    /// Relative paths will be prefixed by `zcashd_datadir` location.
    #[arg(long, default_value = "wallet.dat")]
    pub(crate) path: PathBuf,

    /// Specify `zcashd` data directory (this path cannot use '~').
    #[arg(long)]
    pub(crate) zcashd_datadir: Option<PathBuf>,

    /// Buffer wallet transactions in-memory in the process of performing the wallet restore. For
    /// very active wallets, this might exceed the available memory on your machine, so enable this
    /// with caution.
    #[arg(long)]
    pub(crate) buffer_wallet_transactions: bool,

    /// Allow import of wallet data from multiple `zcashd` `wallet.dat` files. Each imported wallet
    /// will create a distinct set of accounts in `zallet`. Attempts to import wallet data
    /// corresponding to an already-imported wallet will result in an error.
    #[arg(long)]
    pub(crate) allow_multiple_wallet_imports: bool,

    /// Specify the path to the zcashd installation directory.
    ///
    /// This is required for locating the `db_dump` command used to extract data from the
    /// `wallet.dat` file. Wallet migration without a local `zcashd` installation is not yet
    /// supported.
    #[arg(long)]
    pub(crate) zcashd_install_dir: Option<PathBuf>,

    /// Allow a migration when warnings are present. If set to `false`, any warning will be treated
    /// as an error and cause the migration to abort. Setting this to `true` will allow the import
    /// of partially-corrupted wallets, or wallets that contain transaction data from consensus
    /// forks of the Zcash chain (only transaction data corresponding to known consensus rules will
    /// be imported.)
    #[arg(long)]
    pub(crate) allow_warnings: bool,

    /// Temporary flag ensuring any alpha users are aware the migration is not stable.
    #[arg(long)]
    pub(crate) this_is_alpha_code_and_you_will_need_to_redo_the_migration_later: bool,
}

/// `init-wallet-encryption` subcommand
#[cfg(zallet_build = "wallet")]
#[derive(Debug, Parser)]
#[cfg_attr(outside_buildscript, derive(Command))]
pub(crate) struct InitWalletEncryptionCmd {}

/// `generate-mnemonic` subcommand
#[cfg(zallet_build = "wallet")]
#[derive(Debug, Parser)]
#[cfg_attr(outside_buildscript, derive(Command))]
pub(crate) struct GenerateMnemonicCmd {}

/// `import-mnemonic` subcommand
#[cfg(zallet_build = "wallet")]
#[derive(Debug, Parser)]
#[cfg_attr(outside_buildscript, derive(Command))]
pub(crate) struct ImportMnemonicCmd {}

/// `rpc` subcommand
#[cfg(feature = "rpc-cli")]
#[derive(Debug, Parser)]
#[cfg_attr(outside_buildscript, derive(Command))]
pub(crate) struct RpcCliCmd {
    /// Client timeout in seconds during HTTP requests, or 0 for no timeout.
    ///
    /// Default is 900 seconds.
    ///
    /// The server timeout is configured with the `rpc.timeout` option in the
    /// configuration file.
    #[arg(long)]
    pub(crate) timeout: Option<u64>,

    /// The JSON-RPC command to send to Zallet.
    pub(crate) command: String,

    /// Any parameters for the command.
    pub(crate) params: Vec<String>,
}

#[derive(Debug, Parser)]
#[cfg_attr(outside_buildscript, derive(Command, Runnable))]
pub(crate) enum RepairCmd {
    TruncateWallet(TruncateWalletCmd),
}

/// Truncates the wallet database to at most the specified height.
///
/// Upon successful truncation, this method returns the height to which the data store was
/// actually truncated. `zallet start` will then sync the wallet as if this height was the
/// last observed chain tip height.
///
/// There may be restrictions on heights to which it is possible to truncate.
/// Specifically, it will only be possible to truncate to heights at which is is possible
/// to create a witness given the current state of the wallet's note commitment tree.
#[derive(Debug, Parser)]
#[cfg_attr(outside_buildscript, derive(Command))]
pub(crate) struct TruncateWalletCmd {
    /// The maximum height the wallet may treat as a mined block.
    ///
    /// Zallet may choose a lower block height to which the data store will be truncated
    /// if it is not possible to truncate exactly to the specified height.
    pub(crate) max_height: u32,
}
