# Telcoin Network Improvement Proposals (TNIPs)

The goal for TNIPs is to standardize and provide high-quality documentation for Telcoin Network itself and conventions built upon it. This repository tracks past and ongoing improvements to Telcoin Network in the form of Telcoin Network Improvement Proposals (TNIPs). [TNIP-1](https://tnips.telcoin.network/TNIPS/tnip-1) governs how TNIPs are published.

Telcoin Network is still under heavy development. The proposals outlined here guide and document the progress towards mainnet.

The [status page](https://tnips.telcoin.network/) tracks and lists TNIPs, which can be divided into the following categories:

**Before you write a TNIP, ideas MUST be thoroughly discussed on [Telcoin Network Forum](https://forum.telcoin.org/). Once consensus is reached in the community, thoroughly read and review [TNIP-1](https://tnips.telcoin.network/TNIPS/tnip-1), which describes the TNIP process.**

Please note that this repository is for documenting standards and not for help implementing them. These types of inquiries should be directed to the [Telcoin Network Discord Channel](https://discord.com/channels/1252990258514235544/1252996402942836857). For specific questions and concerns regarding TNIPs, it's best to comment on the relevant discussion thread of the TNIP denoted by the `discussions-to` tag in the TNIP's preamble.

## Preferred Citation Format

The canonical URL for a TNIP that has achieved draft status at any point is at <https://tnips.telcoin.network/>. For example, the canonical URL for TNIP-1 is <https://tnips.telcoin.network/TNIPS/tnip-1>.

Consider any document not published at <https://tnips.telcoin.network/> as a working paper. Additionally, consider published TNIPs with a status of "draft", "review", or "last call" to be incomplete drafts, and note that their specification is likely subject to change.

## Validation and Automerging

The CI is incomplete, but included here to document future ambitions.

All pull requests in this repository must pass automated checks before they can be automatically merged:

- TNIP-1 rules are enforced using [`tnipv`](https://github.com/telcoin-network/tnipv)[^2]
- Spelling is enforced with [CodeSpell](https://github.com/codespell-project/codespell)[^2]
  - False positives sometimes occur. When this happens, please submit a PR editing [.codespell-whitelist](https://github.com/telcoin-association/TNIPs/blob/main/config/.codespell-whitelist) and **ONLY** .codespell-whitelist
- Markdown best practices are checked using [markdownlint](https://github.com/DavidAnson/markdownlint)[^2]

[^2]: https://github.com/telcoin-network/TNIPs/blob/main/.github/workflows/ci.yml

It is possible to run the TNIP validator locally:

```sh
cargo install tnipv
tnipv <INPUT FILE / DIRECTORY>
```

## Build the status page locally
The TNIP book is built using rust.

The mdbook compiles the binary in `process-frontmatter`. This preprocessor looks parses content between "+++" to create the preamble for each TNIP.

### Install prerequisites

1. Open Terminal.

2. Check whether you have mdbook installed (requires rust).

   ```sh
   mdbook --version
   ```

3. If you don't have mdbook installed, install mdbook. If you don't have rust, it's recommended to install using `rustup`.

4. Install mdbook with cargo (shipped with rust):

   ```sh
   cargo install mdbook
   ```

### Build the book locally

1. Compile the markdown, start the server, and open the page:

   ```sh
   mdbook serve --open
   ```

More information on mdbook [here](https://rust-lang.github.io/mdBook/index.html).

This document was derived heavily from [Ethereum's EIP repo](https://github.com/ethereum/EIPs).
