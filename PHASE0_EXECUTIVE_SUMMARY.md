# Phase 0: Discovery & Documentation - Executive Summary

**Project:** polkadot-sdk-fresh Rebranding
**Phase:** 0 (Discovery & Documentation)
**Status:** ✅ COMPLETE
**Date:** November 26, 2025
**Branch:** rebranding-phase0-discovery

---

## Executive Summary

Phase 0 discovery successfully cataloged **ALL** URLs, cryptographic parameters, and protocol constants across the polkadot-sdk-fresh codebase. This comprehensive audit provides a complete roadmap for systematic rebranding from Polkadot/Rococo/Westend to Pezkuwi/PezkuwiChain/Zagros.

### Total Scope Documented
- **6,477 URL references** across 4,812 files
- **147+ cryptographic parameters** (SS58 prefixes, protocol IDs, genesis configs)
- **154+ protocol constants** (runtime versions, consensus params, economic configs)
- **150+ files requiring updates** during rebranding phases

---

## Phase 0 Deliverables

### 1. PHASE0_URL_INVENTORY.md (673 lines)
**URL Discovery & Categorization**

- **Total URLs:** 6,477 references
- **PRESERVE (89%):** External dependencies (GitHub, crates.io, Apache licenses)
- **REPLACE (4.1%):** Polkadot branding URLs (266 refs)
  - polkadot.com → pezkuwi.com
  - telemetry.polkadot.io → telemetry.pezkuwi.network
  - wiki.polkadot.network → docs.pezkuwi.network
- **ANALYZE (2.7%):** WebSocket/RPC endpoints (176 refs)

**Critical Infrastructure Needed:**
- telemetry.pezkuwi.network (15 locations)
- zagros-rpc.pezkuwi.network
- docs.pezkuwi.network
- forum.pezkuwi.network

### 2. PHASE0_CRYPTO_AUDIT.md (1,304 lines)
**Cryptographic Parameters & Security Audit**

- **Total Parameters:** 147+
- **SS58 Prefixes:**
  - Polkadot: 0
  - Kusama: 2
  - Rococo: 42 (generic Substrate - **MUST REGISTER NEW**)
  - Westend: 42 (generic Substrate - **MUST REGISTER NEW**)
- **System Parachain IDs:** 7 per relay chain (AssetHub=1000, BridgeHub=1013, etc.)
- **Token Configurations:**
  - DOT: 10 decimals
  - ROC: 12 decimals
  - WND: 12 decimals
  - Proposed HEZ: 12 decimals
  - Proposed ZGR: 12 decimals
  - Proposed TYR: 12 decimals
- **Development Keys:** //Alice, //Bob, //Charlie, //Dave, //Eve, //Ferdie (115+ locations)
- **Protocol ID:** "dot" → Needs replacement

**9 Critical Findings:**
1. SS58 prefix registration required (4-6 weeks lead time)
2. Protocol ID collision prevention
3. Development keys in genesis (security risk)
4. System parachain ID consistency
5. Token decimal decisions
6. Genesis hash migration dependencies
7. Bootnode infrastructure replacement
8. Telemetry endpoint updates
9. Chain type designations

### 3. PHASE0_PROTOCOL_CONSTANTS.md (39K)
**Protocol Constants & Configuration Discovery**

- **Total Constants:** 154+
- **Network-Specific:** 32 requiring rebranding (21%)
- **Runtime Versions:** 13 files (2 relay chains + 11 parachains)
  - spec_name: "rococo" → "pezkuwichain"
  - spec_name: "westend" → "zagros"
  - impl_name: "parity-rococo-v2.0" → "pezkuwi-pezkuwichain"
- **Asset Hub Legacy Names:**
  - "statemine" → pezkuwichain-assethub (wallet compatibility critical)
  - "westmint" → zagros-assethub (wallet compatibility critical)
- **Environment Variables:**
  - ROCOCO_EPOCH_DURATION → PEZKUWICHAIN_EPOCH_DURATION
  - WESTEND_EPOCH_DURATION → ZAGROS_EPOCH_DURATION

**Unchanged Protocol Constants:**
- Block time: 6 seconds
- BABE configuration
- Weight/PoV limits (10MB PoV, 3MB code)
- Economic parameters (numeric values)

---

## Critical Decision Points

### 1. SS58 Address Prefixes
**Decision Required:** Register new unique prefixes for PezkuwiChain and Zagros
- **Timeline:** 4-6 weeks
- **Action:** Apply to ss58-registry immediately
- **Risk:** Launch blocker if delayed

### 2. Token Symbol Verification
**Proposed:**
- HEZ (Pezkuwi mainnet token) - 12 decimals
- ZGR (Zagros testnet token) - 12 decimals
- TYR (TeyrChain parachain token) - 12 decimals

**Action:** Verify availability and register

### 3. Protocol IDs
**Proposed:**
- "pez" or "pezkuwi" for PezkuwiChain
- "zgr" or "zagros" for Zagros
- "teyr" for TeyrChain

**Action:** Confirm naming convention

### 4. System Parachain IDs
**Recommendation:** Keep existing IDs for continuity
- Asset Hub: 1000
- Contracts: 1002
- Encointer: 1003
- People: 1004
- Broker: 1005
- Bridge Hub: 1013

---

## Risk Assessment Matrix

| Risk | Severity | Probability | Mitigation |
|------|----------|------------|------------|
| SS58 prefix registration delay | CRITICAL | MEDIUM | Apply Week 1, have backup options |
| Development keys in production | CRITICAL | LOW | Automated CI checks, mandatory review |
| Bootnode infrastructure failure | HIGH | MEDIUM | Deploy 6-8 redundant bootnodes, 99.9% SLA |
| Wallet compatibility break (Asset Hub) | HIGH | MEDIUM | Coordinate with wallet providers, metadata validation |
| Genesis hash migration issues | MEDIUM | MEDIUM | Comprehensive testing, rollback plan |
| Token symbol conflicts | MEDIUM | LOW | Early verification and registration |
| Protocol ID collisions | MEDIUM | LOW | Unique naming, network isolation testing |
| Infrastructure deployment delays | MEDIUM | MEDIUM | Parallel deployment, phased rollout |
| Documentation drift | LOW | HIGH | Continuous documentation updates |

---

## File Update Summary

### High Priority (Immediate Updates Required)
**13 Runtime Configuration Files:**
1. /polkadot/runtime/rococo/src/lib.rs (spec_name, impl_name)
2. /polkadot/runtime/westend/src/lib.rs (spec_name, impl_name)
3-13. System parachain runtimes (AssetHub, BridgeHub, People, Coretime, Collectives)

### Medium Priority (Phase 1-3)
- 35+ chain specification JSON files
- 20 genesis preset files
- 10 network/bootnode configuration files
- 7 constants/parameter files

### Low Priority (Phase 4-6)
- 50+ documentation files
- 20+ test/mock files
- Comments and inline documentation

---

## Infrastructure Requirements

### Before Code Changes (Pre-Phase 1)
1. **Deploy Telemetry Infrastructure**
   - telemetry.pezkuwi.network
   - Minimum 3 endpoints for redundancy
   - 99.9% uptime SLA

2. **Deploy RPC Infrastructure**
   - zagros-rpc.pezkuwi.network
   - pezkuwichain-rpc.pezkuwi.network
   - Load balancing and DDoS protection

3. **Deploy Bootnode Infrastructure**
   - 6-8 bootnodes per network
   - Geographically distributed
   - Automated health checks

4. **Deploy Documentation Site**
   - docs.pezkuwi.network
   - Migration guides for developers
   - API documentation

---

## Timeline & Effort Estimate

### Phase 0: Discovery & Documentation ✅ COMPLETE
- **Duration:** 1 day
- **Effort:** Discovery, analysis, documentation
- **Deliverables:** 3 comprehensive audit documents

### Upcoming Phases (From REBRANDING_HANDOFF.md)

**Phase 1: URL & Endpoint Replacement**
- Duration: 2-3 days
- Effort: Automated replacements with manual verification
- Risk: Low (external dependencies preserved)

**Phase 2: Polkadot → Pezkuwi Core Rebranding**
- Duration: 1-2 weeks
- Effort: Core branding, package names, crate names
- Risk: Medium (build system dependencies)

**Phase 3: Rococo → PezkuwiChain Rebranding**
- Duration: 1-2 weeks
- Effort: Runtime configs, chain specs, genesis
- Risk: Medium (runtime metadata changes)

**Phase 4: Westend → Zagros Rebranding**
- Duration: 1-2 weeks
- Effort: Testnet infrastructure, runtime configs
- Risk: Medium (parallel to Phase 3)

**Phase 5: Parachain → TeyrChain Rebranding**
- Duration: 1 week
- Effort: Parachain-specific terminology
- Risk: Low (isolated changes)

**Phase 6: Token Symbol Rebranding**
- Duration: 3-5 days
- Effort: DOT→HEZ, ROC→ZGR, WND→TYR
- Risk: Low (string replacements)

**Total Estimated Duration:** 8-12 weeks (excluding SS58 registration wait time)

---

## Success Criteria for Phase 0

✅ **All URLs cataloged and categorized** (6,477 refs)
✅ **All cryptographic parameters documented** (147+ params)
✅ **All protocol constants identified** (154+ constants)
✅ **Infrastructure requirements defined** (telemetry, RPC, bootnodes, docs)
✅ **Risk assessment completed** (9 critical findings)
✅ **File update list generated** (150+ files)
✅ **Timeline estimate provided** (8-12 weeks)
✅ **Critical decisions identified** (SS58, tokens, protocol IDs)

---

## Next Steps

### Immediate Actions (Week 1)
1. ✅ Review Phase 0 deliverables with project leads
2. **SS58 Prefix Registration**
   - Apply to ss58-registry for PezkuwiChain and Zagros
   - Prepare backup prefix options
3. **Token Symbol Verification**
   - Verify HEZ, ZGR, TYR availability
   - Register if available
4. **Infrastructure Planning**
   - Design 6-8 bootnode architecture per network
   - Plan telemetry and RPC deployment timeline
5. **Decision Confirmation**
   - Protocol IDs: Confirm naming convention
   - Parachain IDs: Confirm keeping existing IDs
   - Token decimals: Confirm 12 decimals for all

### Week 2-3: Pre-Phase 1 Preparation
1. Deploy telemetry infrastructure
2. Deploy RPC infrastructure
3. Deploy bootnode infrastructure
4. Set up docs.pezkuwi.network
5. Prepare Phase 1 automation scripts

### Week 4+: Begin Phase 1 Execution
- URL & endpoint replacements
- Systematic progression through Phases 2-6
- Continuous validation and testing

---

## Key Insights from Discovery

### ✅ Good News
- **Most constants are protocol-level and don't need rebranding**
- **Changes are well-isolated to network identity parameters**
- **No fundamental protocol modifications required**
- **Existing infrastructure can serve as templates**

### ⚠️ Important Considerations
- **SS58 prefix registration is critical path** (4-6 weeks)
- **Asset Hub runtime names have wallet compatibility implications**
- **Development keys must be eliminated from production genesis**
- **Infrastructure must be deployed before code changes**

### 🎯 Strategic Recommendations
1. **Parallel workstreams:** Infrastructure deployment + SS58 registration
2. **Phased rollout:** Testnet first (Zagros), then mainnet (Pezkuwi)
3. **Automated validation:** CI checks for development keys, metadata validation
4. **Comprehensive testing:** Each phase requires full integration testing
5. **Documentation-first:** Update docs before code changes

---

## Conclusion

Phase 0 discovery has successfully mapped the complete rebranding scope for polkadot-sdk-fresh. All URLs, cryptographic parameters, and protocol constants have been cataloged with precise file locations and rebranding requirements.

The project is ready to proceed to infrastructure deployment and Phase 1 execution once critical decisions are confirmed and SS58 prefix registration is initiated.

**Estimated Total Effort:** 8-12 weeks
**Critical Path:** SS58 prefix registration (4-6 weeks)
**Overall Risk Level:** MEDIUM (manageable with proper planning)
**Confidence Level:** HIGH (comprehensive discovery complete)

---

## Document References

- **REBRANDING_HANDOFF.md** - Master strategy document (6 phases)
- **PHASE0_URL_INVENTORY.md** - Complete URL catalog and categorization
- **PHASE0_CRYPTO_AUDIT.md** - Cryptographic parameters and security audit
- **PHASE0_PROTOCOL_CONSTANTS.md** - Protocol constants and runtime configurations

All documents located in: `/home/mamostehp/polkadot-sdk-fresh/`

---

**Phase 0 Status:** ✅ COMPLETE
**Ready for Phase 1:** Pending infrastructure deployment and critical decisions
**Git Branch:** rebranding-phase0-discovery
**Next Review:** Project leads confirmation of critical decisions
