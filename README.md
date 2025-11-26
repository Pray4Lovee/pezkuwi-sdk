Pezkuwi-SDK
TeyrChain - The Kurdistan Blockchain Network

A sovereign blockchain parachain built for the Kurdistan region on Polkadot SDK v1.15.6.

Overview
TeyrChain (تێیرچەین) is a production-ready Substrate-based parachain runtime featuring:

12 Custom Pallets: Presale, Governance, Education, Identity, Treasury, and more
Dual Token Economics: HEZ (native) + PEZ (governance)
XCM Integration: Cross-chain asset transfers (wUSDT from Asset Hub)
Democratic Governance: Welati - Digital democracy system
Educational Platform: Perwerde - Learning and certification
Identity System: KYC and citizen verification
Economic Tools: PEZ treasury and rewards system
Key Features
🪙 Token Economics
HEZ Token: Native gas token (inflationary, Polkadot SDK standard)
PEZ Token: Governance token (5B fixed supply, presale distribution)
wUSDT: Bridged USDT from Polkadot Asset Hub (Asset ID 1000)
🏛️ Custom Pallets
Presale: Multi-presale launchpad with soft/hard caps, vesting, bonus tiers
Tiki: NFT-based social and economic features
Identity-KYC: Decentralized identity and compliance
Referral: Incentivized referral system
Perwerde: Educational platform and certification
Token Wrapper: Asset wrapping (wUSDT, etc.)
Welati: Democratic governance and voting
Staking Score: Reputation-based staking rewards
validator-pool: Trust embeded validator system
Trust: Decentralized trust and reputation
PEZ Treasury: Community treasury management
PEZ Rewards: Staking and participation rewards
🌉 Cross-Chain
XCM v5 implementation
Asset Hub USDT bridge (reserve-backed)
Polkadot/Kusama parachain ready
HRMP channels for system parachains
Documentation
Whitepaper: Complete technical specification
Runtime: pezkuwi/runtime/parachain/ (TeyrChain runtime)
Pallets: pezkuwi/pallets/ (11 custom pallets)
Parachain Node: cumulus/pezkuwi-parachain/ (Cumulus collator)
Quick Start
Build from Source
# Clone repository
git clone https://github.com/pezkuwichain/pezkuwi-sdk.git
cd pezkuwi-sdk

# Build release binary
cargo build --release

# Binary location
./target/release/pezkuwi-parachain
Run Local Development
# Start local relay chain (Alice + Bob validators)
# See scripts/devlocalfa-testnet/ for setup

# Start parachain collator
./target/release/pezkuwi-parachain \
  --collator \
  --alice \
  --chain=local \
  --base-path=/tmp/parachain/alice \
  --port 40333 \
  --rpc-port 8844 \
  -- \
  --chain=rococo-local \
  --port 30343 \
  --rpc-port 9977
Network Stages
Current: Alfa Testnet (4 validators) 🔄 Next: Beta Testnet (8 validators) 📅 Future: Staging → Mainnet (Polkadot/Kusama parachain)

Architecture
Polkadot/Kusama Relay Chain
    │
    └─── TeyrChain Parachain (ParaID TBD)
          ├─ Runtime: teyrchain-runtime
          ├─ Consensus: Aura + GRANDPA (via relay)
          ├─ Block Time: 6 seconds
          └─ 11 Custom Pallets
               ├─ XCM Bridge (Asset Hub USDT)
               └─ Democratic Governance
Technology Stack
Framework: Substrate (Polkadot SDK v1.15.6)
Parachain: Cumulus
Language: Rust
Runtime: WASM compilation
Consensus: Aura (PoA)
Finality: GRANDPA (relay chain)
Use Cases
Token Launches: Multi-presale platform with compliance
Digital Governance: Community voting and proposals (Welati)
Education: Online courses and certifications (Perwerde)
Identity: KYC and trust systems
Cross-Chain Finance: USDT bridge, asset swaps
Official Links
Website: https://pezkuwichain.io
Explorer: https://explorer.pezkuwichain.io
RPC: wss://rpc.pezkuwichain.io
Network Dashboard: https://network.pezkuwichain.io
Community
Telegram: @pezkuwichain
Discord: discord.gg/pezkuwichain
Twitter: @pezkuwichain
License
Apache 2.0 - Open Source

Built for the Kurdish Nation

TeyrChain (تێیرچەین) - Empowering Kurdistan through blockchain technology



<div align="center">

![SDK Logo](./docs/images/Pezkuwi_Logo_Horizontal_Pink_White.png#gh-dark-mode-only)
![SDK Logo](./docs/images/Pezkuwi_Logo_Horizontal_Pink_Black.png#gh-light-mode-only)

# Pezkuwi SDK

![GitHub stars](https://img.shields.io/github/stars/pezkuwichain/pezkuwi-sdk)&nbsp;&nbsp;![GitHub
forks](https://img.shields.io/github/forks/pezkuwichain/pezkuwi-sdk)

<!-- markdownlint-disable-next-line MD013 -->
[![StackExchange](https://img.shields.io/badge/StackExchange-Community%20&%20Support-222222?logo=stackexchange)](https://pezkuwichain.app/community)&nbsp;&nbsp;![GitHub contributors](https://img.shields.io/github/contributors/pezkuwichain/pezkuwi-sdk)&nbsp;&nbsp;![GitHub commit activity](https://img.shields.io/github/commit-activity/m/pezkuwichain/pezkuwi-sdk)&nbsp;&nbsp;![GitHub last commit](https://img.shields.io/github/last-commit/pezkuwichain/pezkuwi-sdk)

> The Pezkuwi SDK repository provides all the components needed to start building on the
> [PezkuwiChain](https://pezkuwichain.app/) network, a multi-chain blockchain platform that enables
> different blockchains to interoperate and share information in a secure and scalable way.

</div>

## ⚡ Quickstart
If you want to get an example node running quickly you can execute the following getting started script:

```
curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/pezkuwichain/pezkuwi-sdk/master/scripts/getting-started.sh | bash
```

## 👩🏽‍💻 Building

In order to build this project you need to install some dependencies, follow the instructions in [this guide](https://docs.pezkuwichain.io/develop/parachains/install-pezkuwi-sdk).

## 📚 Documentation

* [Pezkuwi Documentation Portal](https://docs.pezkuwichain.io)
* [🦀 rust-docs](https://pezkuwichain.github.io/pezkuwi-sdk/master/pezkuwi_sdk_docs/index.html): Where we keep track of
the API docs of our Rust crates. Includes:
  * [Introduction](https://pezkuwichain.github.io/pezkuwi-sdk/master/pezkuwi_sdk_docs/pezkuwi_sdk/index.html)
	to each component of the Pezkuwi SDK: Substrate, FRAME, Cumulus, and XCM
  * [Guides](https://pezkuwichain.github.io/pezkuwi-sdk/master/pezkuwi_sdk_docs/guides/index.html),
	namely how to build your first FRAME pallet
  * [Templates](https://pezkuwichain.github.io/pezkuwi-sdk/master/pezkuwi_sdk_docs/pezkuwi_sdk/templates/index.html)
    for starting a new project.
  * [External Resources](https://pezkuwichain.github.io/pezkuwi-sdk/master/pezkuwi_sdk_docs/external_resources/index.html)
* Have a question? You can ask in the Pezkuwi SDK Developers Chat.
Messages from either of these channels are bridged to the other, so you can use whichever one you like.
  * [Telegram](https://t.me/pezkuwidevs)
  * [Matrix](https://matrix.to/#/#pezkuwidevs:matrix.org)
  * [Discord](https://discord.com/channels/722223075629727774/997505821955076196)
  * [Pezkuwi and Substrate StackExchange](https://pezkuwichain.app/community)

## 🚀 Releases

<!-- markdownlint-disable-next-line MD013 -->
![Current Stable Release](https://raw.githubusercontent.com/pezkuwichain/release-registry/main/badges/pezkuwi-sdk-latest.svg)&nbsp;&nbsp;![Next Stable Release](https://raw.githubusercontent.com/pezkuwichain/release-registry/main/badges/pezkuwi-sdk-next.svg)

The Pezkuwi SDK is released every three months as a `Pezkuwi stableYYMM` release. Each stable release is supported for
one year with patches. See the next upcoming versions in the [Release
Registry](https://github.com/pezkuwichain/release-registry/) and more docs in [RELEASE.md](./docs/RELEASE.md).

You can use [`psvm`](https://github.com/pezkuwichain/psvm) to update all dependencies to a specific
version without needing to manually select the correct version for each crate.

## 🛠️ Tooling

[Pezkuwi SDK Version Manager](https://github.com/pezkuwichain/psvm):
A simple tool to manage and update the Pezkuwi SDK dependencies in any Cargo.toml file.
It will automatically update the Pezkuwi SDK dependencies to their correct crates.io version.

## 🔐 Security

The security policy and procedures can be found in
[docs/contributor/SECURITY.md](./docs/contributor/SECURITY.md).

## 🤍 Contributing & Code of Conduct

Ensure you follow our [contribution guidelines](./docs/contributor/CONTRIBUTING.md). In every
interaction and contribution, this project adheres to the [Contributor Covenant Code of
Conduct](./docs/contributor/CODE_OF_CONDUCT.md).

### 👾 Ready to Contribute?

Take a look at the issues labeled with [`mentor`](https://github.com/pezkuwichain/pezkuwi-sdk/labels/C1-mentor)
(or alternatively [this](https://mentor.tasty.limo/) page, created by one of the maintainers) label to get started!
We always recognize valuable contributions by proposing an on-chain tip to the PezkuwiChain network as a token of our
appreciation.

## Pezkuwi Fellowship

Development in this repo usually goes hand in hand with the `fellowship` organization. In short,
this repository provides all the SDK pieces needed to build both PezkuwiChain and its teyrchains. But,
the actual PezkuwiChain runtime lives in the `fellowship/runtimes` repository. Read more about the
fellowship, this separation, the RFC process
[here](https://pezkuwi-fellows.github.io/dashboard/).

## History

This repository is the amalgamation of 3 separate repositories that used to make up Pezkuwi SDK,
namely Substrate, Pezkuwi and Cumulus. Read more about the merge and its history
[here](https://pezkuwi-public.notion.site/Pezkuwi-SDK-FAQ-fbc4cecc2c46443fb37b9eeec2f0d85f).
