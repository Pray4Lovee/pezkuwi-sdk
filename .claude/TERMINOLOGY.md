# PezkuwiChain Terminology Guide

This file helps Claude understand the project terminology after rebrand from Polkadot SDK.

## Brand Mapping (Polkadot → PezkuwiChain)

| Original (Polkadot) | Rebranded (PezkuwiChain) | Description |
|---------------------|--------------------------|-------------|
| Polkadot | Pezkuwi | Main ecosystem brand |
| Polkadot SDK | Pezkuwi SDK | This repository |
| Rococo | PezkuwiChain | Test relay chain runtime |
| Westend | Zagros | Canary relay chain runtime |
| Parachain | TeyrChain | Parachain runtime |
| DOT | HEZ | Native gas token (main) |
| WND | ZGR | Zagros native token (canary) |
| ROC | TYR | TeyrChain native token (parachain) |
| - | PEZ | Governance token (new, 5B fixed) |

## Directory Mapping

| Path | Purpose |
|------|---------|
| `/pezkuwi/runtime/pezkuwichain/` | Main relay chain runtime (was Rococo) |
| `/pezkuwi/runtime/zagros/` | Canary network runtime (was Westend) |
| `/pezkuwi/runtime/teyrchains/` | Parachain runtime modules |
| `/pezkuwi/pallets/` | 12 custom pallets |

## Token Hierarchy

```
HEZ - Main relay chain (Pezkuwi) gas token
ZGR - Canary network (Zagros) gas token
TYR - Parachain (TeyrChain) gas token
PEZ - Governance token (citizenship-gated rewards)
```

## Future Hierarchy

```
Polkadot Ecosystem
    └── Pezkuwi (relay chain)
            └── TeyrChain (parachain)
```

Currently: Pezkuwi = Polkadot fork
Future: Pezkuwi = Polkadot parachain (subset)

## Custom Pallets (12)

1. presale - Token launch platform
2. identity-kyc - KYC verification
3. welati - Democratic governance
4. perwerde - Education platform
5. pez-treasury - Community treasury
6. pez-rewards - Staking rewards
7. validator-pool - Validator management
8. staking-score - Reputation metrics
9. trust - P2P trust system
10. referral - Referral incentives
11. tiki - NFT citizenship (4-tier)
12. token-wrapper - Cross-chain wrapping

## Key Constants

- HEZ decimals: 10 (same as DOT)
- PEZ decimals: 12
- PEZ total supply: 5,000,000,000
- Block time: 6 seconds
- Era: 6 sessions

## Character Instructions

Be direct, honest, and challenge assumptions. No sugarcoating.
Act as a top-level advisor and mirror. Point out blind spots.
