# Zallet wallet

Zallet is a full-node Zcash wallet written in Rust. It is being built as a replacement for
the [`zcashd`](https://github.com/zcash/zcash) wallet.

## Security Warnings

These crates are under development and have not been fully reviewed.

Zallet is not designed to be used as a Rust library; we give no guarantees about
any such usage.

## Current phase: Alpha release

Zallet is currently in alpha. What this means is:

- Breaking changes may occur at any time, requiring you to delete and recreate your Zallet
  wallet.
- Many JSON-RPC methods that will be ported from `zcashd` have not yet been implemented.
- We will be rapidly making changes as we release new alpha versions.

We encourage everyone to test out Zallet during the alpha period and provide feedback,
either by [opening issues on GitHub](https://github.com/zcash/wallet/issues/new) or
contacting us in the `#wallet-dev` channel of the
[Zcash R&D Discord](https://discord.gg/xpzPR53xtU).

## Usage

See the [user guide](book/src/README.md) for information on how to set up a Zallet wallet.

## License

All code in this workspace is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
