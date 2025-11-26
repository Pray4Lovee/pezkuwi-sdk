Pezkuwichain - Kurdistan Blockchain Network
A sovereign blockchain parachain built on Polkadot SDK v1.15.6 for the Kurdish nation
Overview
TeyrChain (تێیرچەین) is a production-ready Substrate-based parachain featuring:

15 Custom Pallets: Presale, Governance (Welati), Education (Perwerde), Identity-KYC, Treasury, and more
Dual Token Economics: HEZ (native gas) + PEZ (governance, 5B fixed supply)
XCM Integration: Cross-chain USDT transfers via Polkadot Asset Hub
TNPoS Consensus: Trust-enhanced validator selection and rewards
Democratic Governance: On-chain voting and treasury management

Key Features
🪙 Token Economics
HEZ Token - Native gas token following Polkadot's inflationary model

Used for: Transaction fees, staking, network security
Distribution: 85% to staking rewards, 15% to treasury
Decimals: 10

PEZ Token - Fixed supply governance token (5,000,000,000 PEZ)

Treasury allocation: 1,012,500,000 PEZ (20.25%)
Presale allocation: 93,750,000 PEZ (1.875%)
Founder allocation: 93,750,000 PEZ (1.875%)
Rewards pool: ~3,800,000,000 PEZ (~76%)
48-month halving cycles
Decimals: 12

wUSDT - Bridged stablecoin from Polkadot Asset Hub (Asset ID 1000)
🛠 Custom Pallets
PalletPurposepresaleMulti-round token launches with vesting and bonus tiersidentity-kycDecentralized identity and KYC compliancewelatiDemocratic governance, proposals and votingperwerdeEducational platform and certificationpez-treasuryCommunity treasury with halving mechanismpez-rewardsTrust-based staking rewards distributionvalidator-poolSimplified validator participationstaking-scoreReputation-based staking metricstrustSocial trust and reputation systemreferralGrowth incentive systemtikiNFT and social featurestoken-wrapperAsset wrapping for cross-chain transfers
🌉 Cross-Chain Features

XCM v5 implementation for cross-consensus messaging
USDT Bridge from Polkadot Asset Hub (reserve-backed)
HRMP Channels for parachain communication

Use Cases

Token Launches: Compliant multi-round presales
Digital Governance: Community voting and treasury management
Education: Blockchain-verified certificates (Perwerde)
Identity: KYC-compliant digital identity system
Cross-Chain Finance: USDT bridge and asset swaps

Links

Website: pezkuwichain.io
Explorer: explorer.pezkuwichain.io
Documentation: docs.pezkuwichain.io
RPC Endpoint: wss://rpc.pezkuwichain.io

Community

Telegram: @pezkuwichain
Discord: discord.gg/pezkuwichain
Twitter: @pezkuwichain
GitHub: github.com/pezkuwichain
Medium: /@pezkuwichain
Facebook: https://www.facebook.com/profile.php?id=61582484611719


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
