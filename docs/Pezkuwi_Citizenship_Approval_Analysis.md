# Pezkuwi Blockchain Citizenship Approval Flow Analysis

## Executive Summary

The Pezkuwi blockchain implements a multi-step citizenship approval flow across three main pallets:
1. **identity-kyc**: Handles KYC (Know Your Customer) verification
2. **tiki**: Manages Tiki (roles) and citizenship NFT minting
3. **welati**: Implements governance with citizenship as a prerequisite

The flow is **DECOUPLED** - identity-kyc pallet does not directly call tiki pallet. Instead, tiki pallet monitors KYC status via hooks and event listeners.

---

## 1. FILE STRUCTURE

### 1.1 Identity-KYC Pallet
**Location**: `/home/mamostehp/Pezkuwi-SDK/pezkuwi/pallets/identity-kyc/`

**Key Files**:
- `src/lib.rs` - Main logic (176 lines)
- `src/types.rs` - Data structures and traits
- `src/tests.rs` - Test cases
- `src/mock.rs` - Test configuration
- `src/benchmarking.rs` - Weight benchmarks
- `src/weights.rs` - Pre-calculated weights

**Storage**:
- `Identities<T>`: Maps AccountId → IdentityInfo (name, email)
- `KycStatuses<T>`: Maps AccountId → KycLevel (NotStarted, Pending, Approved, Revoked)
- `PendingKycApplications<T>`: Maps AccountId → KycApplication (CIDs, notes)

### 1.2 Tiki Pallet (Role & Citizenship NFT)
**Location**: `/home/mamostehp/Pezkuwi-SDK/pezkuwi/pallets/tiki/`

**Key Files**:
- `src/lib.rs` - Main logic (617 lines)
- `src/ensure.rs` - Role-based origin validation
- `src/tests.rs` - Test cases
- `src/mock.rs` - Test configuration

**Storage**:
- `CitizenNft<T>`: Maps AccountId → NFT ItemId
- `UserTikis<T>`: Maps AccountId → BoundedVec<Tiki>
- `TikiHolder<T>`: Maps Tiki → AccountId (for unique roles)
- `NextItemId<T>`: Counter for NFT minting

**Enum - Tiki (40+ roles)**:
- Hemwelatî (Citizenship) - base role
- Serok (President), Parlementer, etc.
- Economic, Judicial, Executive roles

### 1.3 Welati Pallet (Governance)
**Location**: `/home/mamostehp/Pezkuwi-SDK/pezkuwi/pallets/welati/`

**Key Files**:
- `src/lib.rs` - Main logic (1183 lines)
- `src/types.rs` - Governance data structures (972 lines)
- `src/mock.rs` - Test configuration
- `src/tests.rs` - Test cases

**Key Components**:
- Elections system (Presidential, Parliamentary, etc.)
- Appointment system for officials
- Collective decision making
- Requires citizenship (Hemwelatî Tiki) to vote

---

## 2. CITIZENSHIP APPROVAL FLOW

### 2.1 Step-by-Step Process

```
[User] 
   ↓ 
1. set_identity() 
   └→ Creates IdentityInfo {name, email}
   └→ Event: IdentitySet
   ↓
2. apply_for_kyc()
   └→ Requires Identity to exist
   └→ Creates KycApplication {cids, notes}
   └→ Reserves deposit
   └→ Sets KycStatus = Pending
   └→ Event: KycApplied
   ↓
3. approve_kyc() [KYC Reviewer/Root]
   └→ Requires KycStatus = Pending
   └→ Unreserves deposit
   └→ Sets KycStatus = Approved
   └→ Removes PendingApplication
   └→ Event: KycApproved
   ↓
4. [AUTOMATIC via Hook] check_and_mint_citizen_nfts()
   └→ Called at every block start (on_initialize)
   └→ Checks all KYC-approved users without CitizenNft
   └→ Calls mint_citizen_nft_for_user()
   ↓
5. mint_citizen_nft_for_user()
   └→ Mints NFT to user via pallet-nfts
   └→ Locks NFT (non-transferable)
   └→ Auto-grants Hemwelatî (Citizenship) Tiki
   └→ Updates NFT metadata with roles and score
   └→ Event: CitizenNftMinted
   └→ Stores NFT ID in CitizenNft storage
   ↓
[User is now a Citizen]
   └→ Can participate in governance (voting)
   └→ Can receive additional roles (Tiki)
   └→ Score = 10 (Hemwelatî bonus)
```

### 2.2 Alternative Path: Force Mint (Admin Only)

```
[Admin/Root] 
   ↓
force_mint_citizen_nft(user)
   └→ Requires: AdminOrigin (Root)
   └→ Bypasses KYC check
   └→ Directly calls mint_citizen_nft_for_user()
   └→ Used for testing or emergency situations
```

### 2.3 Manual Citizenship Application (Alternative Path)

```
[User with approved KYC]
   ↓
apply_for_citizenship()
   └→ Checks KycStatus = Approved (via pallet_identity_kyc::Pallet)
   └→ Calls mint_citizen_nft_for_user()
   └→ Returns error if KYC not approved
```

---

## 3. KEY FUNCTIONS & EXTRINSICS

### 3.1 Identity-KYC Pallet

#### Extrinsics (Public Functions)

| Function | Origin | Purpose | State Change |
|----------|--------|---------|--------------|
| `set_identity()` | Signed | Register name & email | Creates IdentityInfo |
| `apply_for_kyc()` | Signed | Submit KYC documents (CIDs) | KycStatus: NotStarted → Pending |
| `approve_kyc()` | KycApprovalOrigin | Approve KYC application | KycStatus: Pending → Approved |
| `revoke_kyc()` | KycApprovalOrigin | Revoke approved KYC | KycStatus: Approved → Revoked |

#### Events

```rust
pub enum Event<T: Config> {
    IdentitySet { who: T::AccountId },
    KycApplied { who: T::AccountId },
    KycApproved { who: T::AccountId },
    KycRevoked { who: T::AccountId },
}
```

#### Traits (External Queries)

```rust
pub trait KycStatus<AccountId> {
    fn get_kyc_status(who: &AccountId) -> KycLevel;
}

pub trait IdentityInfoProvider<AccountId, MaxStringLength: Get<u32>> {
    fn get_identity_info(who: &AccountId) -> Option<IdentityInfo<MaxStringLength>>;
}
```

### 3.2 Tiki Pallet

#### Extrinsics (Public Functions)

| Function | Origin | Purpose | Precondition |
|----------|--------|---------|--------------|
| `grant_tiki()` | AdminOrigin | Admin grants a role | CitizenNft exists |
| `revoke_tiki()` | AdminOrigin | Admin revokes a role | Tiki assigned to user |
| `force_mint_citizen_nft()` | AdminOrigin | Admin bypasses KYC | None (override) |
| `grant_elected_role()` | AdminOrigin | Voting system grants role | Is elected role type |
| `grant_earned_role()` | AdminOrigin | Exam/test grants role | Is earned role type |
| `apply_for_citizenship()` | Signed | User requests citizenship NFT | KycStatus = Approved |
| `check_transfer_permission()` | Any | Blocks Tiki NFT transfers | collection = TikiCollectionId |

#### Hooks

```rust
fn on_initialize(block_number: BlockNumberFor<T>) -> Weight {
    Self::check_and_mint_citizen_nfts();  // Auto-mint after KYC
    T::DbWeight::get().reads_writes(10, 5)
}
```

#### Events

```rust
pub enum Event<T: Config> {
    CitizenNftMinted { who: T::AccountId, nft_id: u32 },
    TikiGranted { who: T::AccountId, tiki: Tiki },
    TikiRevoked { who: T::AccountId, tiki: Tiki },
    TransferBlocked { collection_id: T::CollectionId, item_id: u32, from, to },
}
```

#### Traits (External Queries)

```rust
pub trait TikiScoreProvider<AccountId> {
    fn get_tiki_score(who: &AccountId) -> u32;  // Sum of Tiki bonuses
}

pub trait TikiProvider<AccountId> {
    fn has_tiki(who: &AccountId, tiki: &Tiki) -> bool;
    fn get_user_tikis(who: &AccountId) -> Vec<Tiki>;
    fn is_citizen(who: &AccountId) -> bool;
}
```

#### Internal Functions

```rust
fn check_and_mint_citizen_nfts()          // Hook: Auto-mint for KYC-approved users
fn mint_citizen_nft_for_user()            // Core minting logic
fn internal_grant_role()                  // Validates and assigns Tiki
fn internal_revoke_role()                 // Removes Tiki (but not Hemwelatî)
fn lock_nft_transfer()                    // Makes NFT non-transferable
fn update_nft_metadata()                  // Stores roles/score in NFT
fn get_tiki_score()                       // Calculates total score
fn get_bonus_for_tiki()                   // Per-role bonus points
fn can_grant_role_type()                  // Validates role assignment method
fn get_role_assignment_type()             // Returns RoleAssignmentType
fn is_citizen()                           // Checks if CitizenNft exists
fn is_unique_role()                       // One-holder roles only
```

### 3.3 Welati Pallet (Governance Integration)

#### Key Integration Points

```rust
// Citizenship is required for voting
pub trait CitizenInfo {
    fn citizen_count() -> u32;
}

// Tiki scores influence voting power
type TikiSource: TikiScoreProvider<Self::AccountId>;

// KYC status is checked for participation
type KycSource: KycStatus<Self::AccountId>;
```

---

## 4. TIKI SCORE SYSTEM

### 4.1 Hemwelatî (Citizenship) Score

```
Hemwelatî (Citizenship) = 10 points
```

### 4.2 Other Tiki Scores

| Role | Points | Assignment Type |
|------|--------|-----------------|
| Axa (Master) | 250 | Earned |
| Serok (President) | 200 | Elected |
| EndameDiwane (Justice) | 175 | Appointed |
| SerokiMeclise (Speaker) | 150 | Elected |
| Dadger (Judge) | 150 | Appointed |
| Parlementer (Parliamentarian) | 100 | Elected |
| Wezir (Minister) | 100 | Appointed |
| Bazargan (Merchant/Economic) | 60 | Appointed |
| Default roles | 5 | Variable |

### 4.3 Score Calculation

```rust
fn get_tiki_score(who: &AccountId) -> u32 {
    Self::user_tikis(who)
        .iter()
        .map(Self::get_bonus_for_tiki)
        .sum()
}
```

**Example**: User with [Hemwelatî, Parlementer, Wezir] = 10 + 100 + 100 = 210 points

---

## 5. ROLE ASSIGNMENT TYPES

```rust
pub enum RoleAssignmentType {
    Automatic,   // KYC → Hemwelatî (auto-granted)
    Appointed,   // Admin grants (Wezir, Dadger, etc.)
    Elected,     // Voting system (Serok, Parlementer, SerokiMeclise)
    Earned,      // Tests/exams (Axa, Mamoste, Rewsenbîr)
}
```

### Validation Rules

```rust
can_grant_role_type(tiki, assignment_type) -> bool {
    match (required_type, provided_type) {
        (Automatic, Automatic) => true,
        (Appointed, Appointed) => true,
        (Elected, Elected) => true,
        (Earned, Earned) => true,
        _ => false,  // Cross-type assignment rejected
    }
}
```

---

## 6. NFT IMPLEMENTATION

### 6.1 Citizenship NFT Properties

| Property | Value | Notes |
|----------|-------|-------|
| Collection ID | TikiCollectionId (default: 0) | Configured per runtime |
| Item ID | Sequential (NextItemId counter) | Auto-incremented |
| Owner | Citizen account | Non-transferable |
| Transferable | No | Locked via attribute |
| Metadata | JSON with roles and score | Updated on role change |

### 6.2 NFT Locking Mechanism

```rust
fn lock_nft_transfer(collection_id, item_id) {
    pallet_nfts::Pallet::set_attribute(
        RawOrigin::Root,
        collection_id,
        item_id,
        AttributeNamespace::Pallet,
        b"locked".to_vec(),
        b"true".to_vec(),
    )
}
```

### 6.3 NFT Metadata Format

```json
{
    "citizen": true,
    "roles": 3,          // Number of Tiki held
    "score": 210         // Total Tiki score
}
```

**Updated when**: Role granted, role revoked, or citizenship minted

---

## 7. PALLET DEPENDENCIES & INTEGRATION

### 7.1 Dependency Graph

```
welati
  ├─ tiki (uses TikiScoreProvider, requires Hemwelatî for voting)
  ├─ identity-kyc (checks KycStatus)
  ├─ trust (uses TrustScoreProvider)
  └─ pallet-collective, pallet-scheduler, pallet-democracy

tiki
  ├─ identity-kyc (queries KycStatus for citizenship application)
  ├─ pallet-nfts (mints/locks citizen NFTs)
  └─ frame-system (basic dependencies)

identity-kyc
  ├─ pallet-balances (reserves deposit)
  └─ frame-system
```

### 7.2 Trait-Based Communication

**Identity-KYC → Tiki**: None directly. Tiki polls KYC status.

```rust
// In tiki's check_and_mint_citizen_nfts hook:
for (account, kyc_status) in pallet_identity_kyc::KycStatuses::<T>::iter() {
    if kyc_status == KycLevel::Approved {
        Self::mint_citizen_nft_for_user(&account)?;
    }
}
```

**Tiki → Welati**: Via trait implementations

```rust
impl<T: Config> TikiScoreProvider<T::AccountId> for Pallet<T> {
    fn get_tiki_score(who: &T::AccountId) -> u32 {
        let tikis = Self::user_tikis(who);
        tikis.iter().map(Self::get_bonus_for_tiki).sum()
    }
}
```

**Welati → Tiki**: Via extrinsic calls

```rust
// Welati calls tiki extrinsics for role grants
pallet_tiki::Pallet::<T>::grant_elected_role(
    origin,
    elected_user,
    Tiki::Parlementer
)?;
```

---

## 8. CURRENT IMPLEMENTATION STATUS

### 8.1 Implemented Features

✅ **Identity-KYC Pallet**
- Set identity (name, email)
- Apply for KYC with documents (IPFS CIDs)
- Approve/revoke KYC (by authorized origin)
- Deposit mechanism for applications
- KYC status tracking (4 states)

✅ **Tiki Pallet - Core Citizenship**
- Mint citizen NFTs on KYC approval (automatic hook)
- Manual citizenship application (after KYC approval)
- Force mint by admin
- Auto-grant Hemwelatî role with citizenship
- Non-transferable NFT locking

✅ **Tiki Pallet - Role Management**
- 40+ roles defined
- 4 role assignment types (Automatic, Appointed, Elected, Earned)
- Role score calculation system
- Unique role enforcement (e.g., only one Serok)
- Tiki score provider trait

✅ **Welati Pallet - Governance**
- Election system (Presidential, Parliamentary)
- Requires citizenship to vote
- Uses Tiki scores for weighted voting
- Integration with pallet-collective and pallet-democracy

✅ **NFT Management**
- Citizen NFT minting via pallet-nfts
- Metadata with roles and score
- Non-transferable enforcement
- Sequential item ID generation

✅ **Testing**
- Full test coverage for identity-kyc
- Citizenship flow tests in tiki
- Test utilities (setup_kyc_for_user, advance_blocks)

---

## 9. WHAT'S MISSING / GAPS IN IMPLEMENTATION

### 9.1 Critical Gaps

**1. No Cross-Pallet Event Listener**
```
❌ Identity-KYC does NOT emit event that Tiki listens for
   - KycApproved event is emitted but not consumed by Tiki
   - Tiki must poll KycStatuses via storage every block (inefficient)
   
RECOMMENDATION:
  - Add callback mechanism from identity-kyc to tiki
  - Or implement event-driven listener pattern
```

**2. No Automatic Governance Integration**
```
❌ When user becomes citizen, they must manually register to vote
   - Welati doesn't auto-create voter entry
   - No "citizen-created" event triggers voter registration
   
RECOMMENDATION:
  - On CitizenNftMinted event, auto-register voter in welati
  - Or add extrinsic to welati that checks citizenship onchain
```

**3. KYC Revocation Does Not Revoke Citizenship**
```
❌ If KYC is revoked (approved → revoked):
   - Citizen NFT is NOT destroyed
   - Hemwelatî Tiki is NOT removed
   - User remains a citizen with voting rights
   
RECOMMENDATION:
  - Add hook to revoke citizenship when KYC revoked
  - Consider: should revocation be permanent or reversible?
  - Design: burn NFT? Remove Hemwelatî role? Both?
```

**4. No Trust Score Integration in Citizenship**
```
❌ Citizenship is based ONLY on KYC completion
   - Trust score (from trust pallet) is not considered
   - No reputation requirement for citizenship
   
RECOMMENDATION:
  - Decide: should citizenship require trust score threshold?
  - If yes: add check in apply_for_citizenship()
  - If no: document why reputation not needed
```

**5. Insufficient Hooks in Identity-KYC**
```
❌ identity-kyc pallet has NO hooks
   - Cannot trigger tiki minting on approval
   - Tiki must poll (block-by-block scanning)
   
RECOMMENDATION:
  - Add Hooks trait to identity-kyc
  - Call after KYC approval: notify dependent pallets
  - Improves efficiency and decoupling
```

**6. No Explicit Citizenship Revocation Extrinsic**
```
❌ No direct way to revoke citizenship after granted
   - KYC revocation doesn't cascade to tiki
   - Admin cannot revoke Hemwelatî without technical knowledge
   
RECOMMENDATION:
  - Add revoke_citizenship() extrinsic to tiki
  - Should burn NFT and remove Hemwelatî role
  - Log event for audit trail
```

**7. No KYC Review Workflow**
```
❌ KYC approval is instant (single extrinsic)
   - No "pending review" state before final approval
   - No multiple reviewer support
   - No rejection reason tracking
   
RECOMMENDATION:
  - Design review workflow: submitted → under_review → approved/rejected
  - Track reviewer identity
  - Store rejection reason
  - Consider: should KYC be updatable after rejection?
```

### 9.2 Design Gaps

**8. No Identity Update Mechanism**
```
❌ Once identity is set, cannot be updated
   - No extrinsic to change name/email
   - User must clear and reset (if possible)
   
RECOMMENDATION:
  - Add update_identity() extrinsic
  - Consider: should update clear KYC status?
```

**9. No Citizenship Duration/Expiration**
```
❌ Citizenship is permanent once granted
   - No expiry date or renewal requirement
   - Different from real-world citizenship (e.g., passports)
   
RECOMMENDATION:
  - Design: should citizenship be perpetual or time-limited?
  - If time-limited: add expiry field, renewal mechanism
  - Consider implications on governance (can expired citizen still vote?)
```

**10. No Citizenship Levels or Degrees**
```
❌ Citizenship is binary (citizen or not)
   - No distinction between "pending" and "active" citizen
   - No probation period
   
RECOMMENDATION:
  - Consider: add intermediate state like "provisional_citizen"
  - Use state for voting restrictions during probation
```

**11. No Identity Privacy Options**
```
❌ Identity (name, email) is public on-chain
   - No encryption or hashing
   - No privacy for personal data (GDPR concern)
   
RECOMMENDATION:
  - Store identity hash only on-chain
  - Keep full identity off-chain (IPFS/centralized)
  - Link via hash verification
```

**12. No Citizenship Transfer/Inheritance**
```
❌ Citizen NFT cannot be transferred (by design)
   - No mechanism for account migration
   - If private key lost, citizenship is lost
   
RECOMMENDATION:
  - Design: add account migration process
  - Or: allow admin to re-issue NFT to new account
  - Include fallback recovery mechanism
```

### 9.3 Missing Extrinsics

| Missing Function | Purpose | Urgency |
|------------------|---------|---------|
| `revoke_citizenship()` | Admin-controlled citizenship removal | HIGH |
| `update_identity()` | User can change name/email | MEDIUM |
| `set_kyc_reviewer()` | Configure authorized KYC approvers | MEDIUM |
| `reject_kyc_application()` | Explicitly reject (not just approve) | MEDIUM |
| `migrate_citizenship()` | Move citizenship to new account | LOW |
| `verify_citizenship_offchain()` | ZK proof for privacy | LOW |

### 9.4 Missing Tests

```
❌ No integration test for full flow:
   set_identity → apply_kyc → approve_kyc → check citizenship created
   
❌ No test for KYC revocation cascading to citizenship
   
❌ No test for concurrent KYC applications
   
❌ No test for role assignment after citizenship
   
❌ No test for governance participation rights
```

---

## 10. DATA FLOW DIAGRAM

```
┌─────────────────────────────────────────────────────────────┐
│                     USER JOURNEY                             │
└─────────────────────────────────────────────────────────────┘

User (Account)
    │
    ├─ 1. set_identity(name, email)
    │   └─> IdentityInfo stored in identity-kyc
    │       Event: IdentitySet
    │
    ├─ 2. apply_for_kyc(cids, notes)
    │   └─> KycApplication stored
    │       KycStatus = Pending
    │       Deposit reserved
    │       Event: KycApplied
    │
    │   [KYC Reviewer]
    ├─ 3. approve_kyc(user) [Root/KycApprovalOrigin]
    │   └─> KycStatus = Approved
    │       PendingApplication removed
    │       Deposit unreserved
    │       Event: KycApproved
    │
    ├─ 4. [AUTOMATIC] on_initialize() block hook (tiki pallet)
    │   └─> check_and_mint_citizen_nfts()
    │       └─> Scans all KYC-approved users
    │           └─> For each: mint_citizen_nft_for_user()
    │               ├─> Mint NFT via pallet-nfts
    │               ├─> Lock NFT (non-transferable)
    │               ├─> Grant Hemwelatî (10 points)
    │               ├─> Store CitizenNft mapping
    │               └─> Event: CitizenNftMinted
    │
    └─ 5. User is now CITIZEN ✓
        ├─> Can query: is_citizen()
        ├─> Can query: get_tiki_score() = 10
        ├─> Can receive additional Tiki (roles)
        ├─> Can vote in governance (welati checks citizenship)
        └─> Can query: get_user_tikis() = [Hemwelatî]

┌─────────────────────────────────────────────────────────────┐
│              STORAGE STATE CHANGES                           │
└─────────────────────────────────────────────────────────────┘

identity-kyc Storage:
  Identities[user]           None ──> IdentityInfo{name, email}
  KycStatuses[user]          NotStarted ──> Pending ──> Approved
  PendingKycApplications[u]  None ──> KycApplication ──> None

tiki Storage:
  CitizenNft[user]           None ──> ItemId
  UserTikis[user]            [] ──> [Hemwelatî]
  NextItemId                 0 ──> 1 ──> 2 ──> ...

pallet-nfts Storage:
  Item(collection=0, item)   None ──> Item{owner=user, locked}
  ItemMetadata               None ──> {"citizen":true,"roles":1,"score":10}

┌─────────────────────────────────────────────────────────────┐
│              EXTERNAL PALLETS AFFECTED                       │
└─────────────────────────────────────────────────────────────┘

pallet-nfts:
  ├─ Owns citizen NFT collection
  ├─ Stores metadata
  └─ Enforces transfer lock

pallet-balances:
  ├─ Reserves deposit when apply_for_kyc()
  └─ Unreserves deposit when approve_kyc()

welati (governance):
  ├─ Checks citizenship requirement for voting
  ├─ Uses TikiScoreProvider for voting weight
  ├─ Creates governance entries for citizens
  └─ Calls grant_tiki() for elected/appointed roles

referral, trust, staking-score:
  ├─ May query citizen status
  ├─ May award bonuses to citizens
  └─ Interact with Tiki score system
```

---

## 11. SECURITY CONSIDERATIONS

### 11.1 Implemented Safeguards

✅ **Deposit Mechanism**: KYC applicants must pay deposit (deters spam)
✅ **Origin Validation**: Only authorized origins can approve KYC
✅ **Non-Transferable NFTs**: Citizens cannot sell/trade their citizenship
✅ **State Machine**: KYC status follows strict state transitions
✅ **Unique Roles**: One-holder roles (Serok) enforced correctly
✅ **Role-Based Access**: grant_tiki requires admin origin
✅ **NFT Locking**: Citizenship NFT cannot be transferred

### 11.2 Potential Risks

⚠️ **Block Scanning Inefficiency**: on_initialize scans ALL users every block
  - Mitigation: Consider pagination/batching for high user counts

⚠️ **Storage Iteration Cost**: Iterating PendingKycApplications is expensive
  - Mitigation: Add indexed lookups or event-based notification

⚠️ **No Slashing for KYC Revocation**: Lost citizenship but no stake burn
  - Mitigation: Consider economic penalties for bad actors

⚠️ **Identity Privacy**: Personal data on-chain (name, email)
  - Mitigation: Use hashing or off-chain storage

⚠️ **No Review Audit Trail**: Who approved KYC is not logged
  - Mitigation: Add reviewer field to KycApplication

---

## 12. RECOMMENDATIONS FOR IMPROVEMENT

### Immediate (CRITICAL)

1. **Add Citizenship Revocation**
   - Extrinsic: `revoke_citizenship(user)`
   - Behavior: burn NFT, remove Hemwelatî, emit event
   - Called when: KYC revoked or admin action

2. **Implement Hook in identity-kyc**
   - Notify tiki pallet on KYC approval (not just polling)
   - Add after-dispatch hook to approve_kyc()
   - Improves efficiency and clarity of flow

3. **Add Integration Tests**
   - Full end-to-end: identity → kyc → citizenship → role assignment
   - Test KYC revocation cascade
   - Test role assignment after citizenship

### Short-term (HIGH PRIORITY)

4. **Design KYC Review Workflow**
   - Multi-step: submitted → under_review → approved/rejected
   - Track reviewer identity
   - Allow rejection with reason

5. **Add Identity Update Mechanism**
   - Extrinsic: `update_identity(name, email)`
   - Decision: clear KYC on update?

6. **Implement Citizenship Expiry**
   - Add expiry_block to citizenship
   - Auto-revoke expired citizens
   - Renewal mechanism before expiry

### Medium-term (QUALITY)

7. **Privacy Enhancement**
   - Hash-based identity storage
   - Off-chain identity verification
   - Zero-knowledge proofs (optional)

8. **Governance Auto-Registration**
   - On CitizenNftMinted, register voter in welati
   - Auto-initialize voting rights
   - Check eligibility in welati when needed

9. **Account Recovery Mechanism**
   - Extrinsic: `migrate_citizenship(old_account, new_account)`
   - Requires: authorization (proof of control)
   - Transfers NFT ownership

10. **Enhanced Metrics**
    - Track KYC approve/reject rates
    - Monitor citizenship growth
    - Audit logs for all changes

---

## 13. CALL SEQUENCE EXAMPLES

### Example 1: Complete Citizenship Path

```bash
# Day 1: User starts KYC
Alice.set_identity("Alice Smith", "alice@example.com")
  Event: IdentitySet { who: Alice }

Alice.apply_for_kyc(
  cids: ["QmHash1", "QmHash2"],
  notes: "ID document and proof of residence"
)
  Event: KycApplied { who: Alice }
  Storage: KycStatus[Alice] = Pending
  Storage: KycApplications[Alice] = {...}
  Balance: reserved(Alice, 100)

# Day 5: KYC Reviewer approves
Root.approve_kyc(Alice)
  Event: KycApproved { who: Alice }
  Storage: KycStatus[Alice] = Approved
  Storage: PendingApplications[Alice] removed
  Balance: unreserved(Alice, 100)

# Block 100 (automatic):
on_initialize(block=100)
  check_and_mint_citizen_nfts()
    if KycStatus[Alice] == Approved and CitizenNft[Alice] == None:
      mint_citizen_nft_for_user(Alice)
        pallet_nfts::mint(collection=0, item=42, owner=Alice)
        lock_nft_transfer(0, 42)
        grant Hemwelatî role
        Storage: CitizenNft[Alice] = 42
        Storage: UserTikis[Alice] = [Hemwelatî]
        update_nft_metadata() → {"citizen":true,"roles":1,"score":10}
        Event: CitizenNftMinted { who: Alice, nft_id: 42 }

# Now Alice can:
Alice.participate_in_governance()
Bob.grant_tiki(Alice, Tiki::Wezir)  // Admin grants role
Alice.query_is_citizen() → true
Alice.query_score() → 110 (10 + 100)
```

### Example 2: KYC Rejection (Missing)

```bash
# This scenario is NOT CURRENTLY IMPLEMENTED!
Root.reject_kyc(Charlie, "Insufficient documentation")
  # No such extrinsic exists
  # Workaround: admin must revoke_kyc() which isn't for rejected apps
```

### Example 3: Citizenship Revocation (Missing)

```bash
# This scenario is NOT CURRENTLY IMPLEMENTED!
Root.revoke_citizenship(Alice)
  # No such extrinsic exists
  # Currently: only KYC can be revoked (which doesn't affect citizenship NFT)
```

---

## 14. GLOSSARY

| Term | Definition |
|------|-----------|
| **Hemwelatî** | Citizenship role (10 points), auto-granted with citizen NFT |
| **Tiki** | Role/badge system (40+ roles, 4 assignment types) |
| **Citizen** | User who has been KYC-approved and has citizen NFT |
| **KYC** | Know Your Customer (identity verification process) |
| **CID** | Content Identifier (IPFS reference to KYC documents) |
| **NFT** | Non-Fungible Token (citizen NFT is non-transferable) |
| **Origin** | Authentication source (Root, Signed, Custom) |
| **Pallet** | Substrate module/component (similar to smart contract) |
| **Hook** | Auto-executed function at block start (on_initialize) |
| **Extrinsic** | User-callable blockchain transaction |
| **Storage** | On-chain persistent data structure |
| **Trait** | Interface for inter-pallet communication |
| **Welati** | Governance/state administration pallet |

---

## 15. APPENDIX: Code References

### Key Code Locations

**Identity-KYC Approval Flow**
- File: `/home/mamostehp/Pezkuwi-SDK/pezkuwi/pallets/identity-kyc/src/lib.rs`
- Lines: 129-147 (approve_kyc extrinsic)

**Citizenship NFT Minting Hook**
- File: `/home/mamostehp/Pezkuwi-SDK/pezkuwi/pallets/tiki/src/lib.rs`
- Lines: 146-154 (on_initialize hook)
- Lines: 296-310 (check_and_mint_citizen_nfts function)
- Lines: 313-347 (mint_citizen_nft_for_user function)

**Citizenship Application Extrinsic**
- File: `/home/mamostehp/Pezkuwi-SDK/pezkuwi/pallets/tiki/src/lib.rs`
- Lines: 250-267 (apply_for_citizenship)

**Role Assignment Types**
- File: `/home/mamostehp/Pezkuwi-SDK/pezkuwi/pallets/tiki/src/lib.rs`
- Lines: 460-476 (get_role_assignment_type)
- Lines: 478-492 (can_grant_role_type)

**Tiki Score Calculation**
- File: `/home/mamostehp/Pezkuwi-SDK/pezkuwi/pallets/tiki/src/lib.rs`
- Lines: 554-617 (get_bonus_for_tiki, score mapping)
- Lines: 532-537 (TikiScoreProvider trait implementation)

**Welati Governance Integration**
- File: `/home/mamostehp/Pezkuwi-SDK/pezkuwi/pallets/welati/src/lib.rs`
- Lines: 35-36 (use pallet_tiki)
- Lines: 69-70 (TikiSource trait)

**Tests**
- File: `/home/mamostehp/Pezkuwi-SDK/pezkuwi/pallets/tiki/src/tests.rs`
- Lines: 85-98 (apply_for_citizenship_works_with_kyc)
- Lines: 101-111 (apply_for_citizenship_fails_without_kyc)

---

**Document Version**: 1.0
**Last Updated**: 2024-11-12
**Analysis Scope**: Very Thorough
**Thoroughness Level**: Architecture + Code Deep Dive + Gap Analysis

