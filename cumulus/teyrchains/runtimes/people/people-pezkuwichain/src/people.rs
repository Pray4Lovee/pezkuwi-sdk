// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::*;
use crate::xcm_config::LocationToAccountId;
use codec::{Decode, Encode, MaxEncodedLen};
use enumflags2::{bitflags, BitFlags};
use frame_support::{
	parameter_types,
	traits::ConstU32,
	weights::Weight,
	CloneNoBound, EqNoBound, PartialEqNoBound, RuntimeDebugNoBound,
};
use frame_system::EnsureRoot;
use pallet_identity::{Data, IdentityInformationProvider};
use scale_info::TypeInfo;
use sp_runtime::{
	traits::{AccountIdConversion, Verify},
	RuntimeDebug,
};
use teyrchains_common::{impls::ToParentTreasury, DAYS, HOURS};
use testnet_teyrchains_constants::pezkuwichain::currency::UNITS;

parameter_types! {
	//   27 | Min encoded size of `Registration`
	// - 10 | Min encoded size of `IdentityInfo`
	// -----|
	//   17 | Min size without `IdentityInfo` (accounted for in byte deposit)
	pub const BasicDeposit: Balance = deposit(1, 17);
	pub const ByteDeposit: Balance = deposit(0, 1);
	pub const UsernameDeposit: Balance = deposit(0, 32);
	pub const SubAccountDeposit: Balance = deposit(1, 53);
	pub RelayTreasuryAccount: AccountId =
		teyrchains_common::TREASURY_PALLET_ID.into_account_truncating();
}

impl pallet_identity::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type BasicDeposit = BasicDeposit;
	type ByteDeposit = ByteDeposit;
	type UsernameDeposit = UsernameDeposit;
	type SubAccountDeposit = SubAccountDeposit;
	type MaxSubAccounts = ConstU32<100>;
	type IdentityInformation = IdentityInfo;
	type MaxRegistrars = ConstU32<20>;
	type Slashed = ToParentTreasury<RelayTreasuryAccount, LocationToAccountId, Runtime>;
	type ForceOrigin = EnsureRoot<Self::AccountId>;
	type RegistrarOrigin = EnsureRoot<Self::AccountId>;
	type OffchainSignature = Signature;
	type SigningPublicKey = <Signature as Verify>::Signer;
	type UsernameAuthorityOrigin = EnsureRoot<Self::AccountId>;
	type PendingUsernameExpiration = ConstU32<{ 7 * DAYS }>;
	type UsernameGracePeriod = ConstU32<{ 3 * DAYS }>;
	type MaxSuffixLength = ConstU32<7>;
	type MaxUsernameLength = ConstU32<32>;
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = ();
	type WeightInfo = weights::pallet_identity::WeightInfo<Runtime>;
}

/// The fields that we use to identify the owner of an account with. Each corresponds to a field
/// in the `IdentityInfo` struct.
#[bitflags]
#[repr(u64)]
#[derive(Clone, Copy, PartialEq, Eq, RuntimeDebug)]
pub enum IdentityField {
	Display,
	Legal,
	Web,
	Matrix,
	Email,
	PgpFingerprint,
	Image,
	Twitter,
	GitHub,
	Discord,
}

/// Information concerning the identity of the controller of an account.
#[derive(
	CloneNoBound,
	Encode,
	Decode,
	DecodeWithMemTracking,
	EqNoBound,
	MaxEncodedLen,
	PartialEqNoBound,
	RuntimeDebugNoBound,
	TypeInfo,
)]
#[codec(mel_bound())]
pub struct IdentityInfo {
	/// A reasonable display name for the controller of the account. This should be whatever the
	/// account is typically known as and should not be confusable with other entities, given
	/// reasonable context.
	///
	/// Stored as UTF-8.
	pub display: Data,

	/// The full legal name in the local jurisdiction of the entity. This might be a bit
	/// long-winded.
	///
	/// Stored as UTF-8.
	pub legal: Data,

	/// A representative website held by the controller of the account.
	///
	/// NOTE: `https://` is automatically prepended.
	///
	/// Stored as UTF-8.
	pub web: Data,

	/// The Matrix (e.g. for Element) handle held by the controller of the account. Previously,
	/// this was called `riot`.
	///
	/// Stored as UTF-8.
	pub matrix: Data,

	/// The email address of the controller of the account.
	///
	/// Stored as UTF-8.
	pub email: Data,

	/// The PGP/GPG public key of the controller of the account.
	pub pgp_fingerprint: Option<[u8; 20]>,

	/// A graphic image representing the controller of the account. Should be a company,
	/// organization or project logo or a headshot in the case of a human.
	pub image: Data,

	/// The Twitter identity. The leading `@` character may be elided.
	pub twitter: Data,

	/// The GitHub username of the controller of the account.
	pub github: Data,

	/// The Discord username of the controller of the account.
	pub discord: Data,
}

impl IdentityInformationProvider for IdentityInfo {
	type FieldsIdentifier = u64;

	fn has_identity(&self, fields: Self::FieldsIdentifier) -> bool {
		self.fields().bits() & fields == fields
	}

	#[cfg(feature = "runtime-benchmarks")]
	fn create_identity_info() -> Self {
		let data = Data::Raw(alloc::vec![0; 32].try_into().unwrap());

		IdentityInfo {
			display: data.clone(),
			legal: data.clone(),
			web: data.clone(),
			matrix: data.clone(),
			email: data.clone(),
			pgp_fingerprint: Some([0; 20]),
			image: data.clone(),
			twitter: data.clone(),
			github: data.clone(),
			discord: data,
		}
	}

	#[cfg(feature = "runtime-benchmarks")]
	fn all_fields() -> Self::FieldsIdentifier {
		use enumflags2::BitFlag;
		IdentityField::all().bits()
	}
}

impl IdentityInfo {
	pub(crate) fn fields(&self) -> BitFlags<IdentityField> {
		let mut res = <BitFlags<IdentityField>>::empty();
		if !self.display.is_none() {
			res.insert(IdentityField::Display);
		}
		if !self.legal.is_none() {
			res.insert(IdentityField::Legal);
		}
		if !self.web.is_none() {
			res.insert(IdentityField::Web);
		}
		if !self.matrix.is_none() {
			res.insert(IdentityField::Matrix);
		}
		if !self.email.is_none() {
			res.insert(IdentityField::Email);
		}
		if self.pgp_fingerprint.is_some() {
			res.insert(IdentityField::PgpFingerprint);
		}
		if !self.image.is_none() {
			res.insert(IdentityField::Image);
		}
		if !self.twitter.is_none() {
			res.insert(IdentityField::Twitter);
		}
		if !self.github.is_none() {
			res.insert(IdentityField::GitHub);
		}
		if !self.discord.is_none() {
			res.insert(IdentityField::Discord);
		}
		res
	}
}

/// A `Default` identity. This is given to users who get a username but have not set an identity.
impl Default for IdentityInfo {
	fn default() -> Self {
		IdentityInfo {
			display: Data::None,
			legal: Data::None,
			web: Data::None,
			matrix: Data::None,
			email: Data::None,
			pgp_fingerprint: None,
			image: Data::None,
			twitter: Data::None,
			github: Data::None,
			discord: Data::None,
		}
	}
}

// =============================================================================
// PezkuwiChain Custom People Pallets Configuration
// =============================================================================
// NOTE: These configurations are placeholders. Full implementation requires
// additional pallet API alignment. See compile errors for specific issues.
// =============================================================================

parameter_types! {
	/// Deposit required for KYC application (spam prevention)
	pub const KycApplicationDeposit: Balance = UNITS; // 1 PEZ
	/// Maximum string length for identity fields
	pub const MaxStringLength: u32 = 128;
	/// Maximum CID (IPFS) length
	pub const MaxCidLength: u32 = 64;
}

/// Noop implementation for OnKycApproved hook
pub struct OnKycApprovedHook;
impl pallet_identity_kyc::types::OnKycApproved<AccountId> for OnKycApprovedHook {
	fn on_kyc_approved(_who: &AccountId, _referrer: &AccountId) {
		// Hook implementation - referral pallet integration
	}
}

/// Noop implementation for OnCitizenshipRevoked hook
pub struct OnCitizenshipRevokedHook;
impl pallet_identity_kyc::types::OnCitizenshipRevoked<AccountId> for OnCitizenshipRevokedHook {
	fn on_citizenship_revoked(_who: &AccountId) {
		// Penalty logic can be added here
	}
}

/// Citizen NFT provider - noop implementation for now
pub struct CitizenNftProviderImpl;
impl pallet_identity_kyc::types::CitizenNftProvider<AccountId> for CitizenNftProviderImpl {
	fn mint_citizen_nft(_who: &AccountId) -> Result<(), sp_runtime::DispatchError> {
		Ok(())
	}

	fn mint_citizen_nft_confirmed(_who: &AccountId) -> Result<(), sp_runtime::DispatchError> {
		Ok(())
	}

	fn burn_citizen_nft(_who: &AccountId) -> Result<(), sp_runtime::DispatchError> {
		Ok(())
	}
}

impl pallet_identity_kyc::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type GovernanceOrigin = EnsureRoot<AccountId>;
	type WeightInfo = pallet_identity_kyc::weights::SubstrateWeight<Runtime>;
	type OnKycApproved = OnKycApprovedHook;
	type OnCitizenshipRevoked = OnCitizenshipRevokedHook;
	type CitizenNftProvider = CitizenNftProviderImpl;
	type KycApplicationDeposit = KycApplicationDeposit;
	type MaxStringLength = MaxStringLength;
	type MaxCidLength = MaxCidLength;
}

// =============================================================================
// Perwerde (Education) Pallet Configuration
// =============================================================================

parameter_types! {
	pub const MaxCourseNameLength: u32 = 128;
	pub const MaxCourseDescLength: u32 = 512;
	pub const MaxCourseLinkLength: u32 = 256;
	pub const MaxStudentsPerCourse: u32 = 1000;
	pub const MaxCoursesPerStudent: u32 = 50;
}

/// Admin origin that returns the account ID
pub struct PerwerdeAdminOrigin;
impl frame_support::traits::EnsureOrigin<RuntimeOrigin> for PerwerdeAdminOrigin {
	type Success = AccountId;
	fn try_origin(o: RuntimeOrigin) -> Result<Self::Success, RuntimeOrigin> {
		frame_system::ensure_root(o.clone())
			.map(|_| sp_keyring::Sr25519Keyring::Alice.to_account_id())
			.map_err(|_| o)
	}
	#[cfg(feature = "runtime-benchmarks")]
	fn try_successful_origin() -> Result<RuntimeOrigin, ()> {
		Ok(RuntimeOrigin::root())
	}
}

impl pallet_perwerde::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type AdminOrigin = PerwerdeAdminOrigin;
	type WeightInfo = pallet_perwerde::weights::SubstrateWeight<Runtime>;
	type MaxCourseNameLength = MaxCourseNameLength;
	type MaxCourseDescLength = MaxCourseDescLength;
	type MaxCourseLinkLength = MaxCourseLinkLength;
	type MaxStudentsPerCourse = MaxStudentsPerCourse;
	type MaxCoursesPerStudent = MaxCoursesPerStudent;
}

// =============================================================================
// Referral Pallet Configuration
// =============================================================================

parameter_types! {
	/// Default referrer account (genesis/system account)
	pub DefaultReferrer: AccountId = sp_keyring::Sr25519Keyring::Alice.to_account_id();
	/// Penalty per revocation (trust score reduction)
	pub const PenaltyPerRevocation: u32 = 10;
}

impl pallet_referral::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_referral::weights::SubstrateWeight<Runtime>;
	type DefaultReferrer = DefaultReferrer;
	type PenaltyPerRevocation = PenaltyPerRevocation;
}

// =============================================================================
// NFTs Pallet Configuration (required by Tiki)
// =============================================================================

parameter_types! {
	pub const NftsCollectionDeposit: Balance = 10 * UNITS;
	pub const NftsItemDeposit: Balance = UNITS / 100;
	pub const NftsMetadataDepositBase: Balance = deposit(1, 129);
	pub const NftsAttributeDepositBase: Balance = deposit(1, 0);
	pub const NftsDepositPerByte: Balance = deposit(0, 1);
	pub NftsPalletFeatures: pallet_nfts::PalletFeatures = pallet_nfts::PalletFeatures::all_enabled();
	pub const NftsMaxDeadlineDuration: BlockNumber = 12 * 30 * DAYS;
	pub const NftsMaxAttributesPerCall: u32 = 10;
}

impl pallet_nfts::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type CollectionId = u32;
	type ItemId = u32;
	type Currency = Balances;
	type ForceOrigin = EnsureRoot<AccountId>;
	type CreateOrigin = frame_support::traits::AsEnsureOriginWithArg<frame_system::EnsureSigned<AccountId>>;
	type Locker = ();
	type CollectionDeposit = NftsCollectionDeposit;
	type ItemDeposit = NftsItemDeposit;
	type MetadataDepositBase = NftsMetadataDepositBase;
	type AttributeDepositBase = NftsAttributeDepositBase;
	type DepositPerByte = NftsDepositPerByte;
	type StringLimit = ConstU32<256>;
	type KeyLimit = ConstU32<64>;
	type ValueLimit = ConstU32<256>;
	type ApprovalsLimit = ConstU32<20>;
	type ItemAttributesApprovalsLimit = ConstU32<30>;
	type MaxTips = ConstU32<10>;
	type MaxDeadlineDuration = NftsMaxDeadlineDuration;
	type MaxAttributesPerCall = NftsMaxAttributesPerCall;
	type Features = NftsPalletFeatures;
	type OffchainSignature = Signature;
	type OffchainPublic = <Signature as sp_runtime::traits::Verify>::Signer;
	#[cfg(feature = "runtime-benchmarks")]
	type Helper = ();
	type WeightInfo = pallet_nfts::weights::SubstrateWeight<Runtime>;
	type BlockNumberProvider = frame_system::Pallet<Runtime>;
}

// =============================================================================
// Tiki (Role NFT) Pallet Configuration
// =============================================================================

parameter_types! {
	/// Collection ID for Tiki (Role) NFTs - Collection 0 is reserved for citizenship/roles
	pub const TikiCollectionId: u32 = 0;
	/// Maximum number of roles a user can hold
	pub const MaxTikisPerUser: u32 = 20;
}

impl pallet_tiki::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type AdminOrigin = EnsureRoot<AccountId>;
	type WeightInfo = pallet_tiki::weights::SubstrateWeight<Runtime>;
	type TikiCollectionId = TikiCollectionId;
	type MaxTikisPerUser = MaxTikisPerUser;
	type Tiki = pallet_tiki::Tiki;
}

// =============================================================================
// Staking Score Pallet Configuration
// =============================================================================

parameter_types! {
	/// Update interval for staking scores (blocks)
	pub const StakingScoreUpdateInterval: BlockNumber = HOURS;
}

/// Staking info provider - noop implementation for People parachain
/// On People chain, we don't have direct access to staking info from relay chain.
/// This is a placeholder that returns None, meaning users won't get staking-based scores here.
pub struct StakingInfoProvider;
impl pallet_staking_score::StakingInfoProvider<AccountId, Balance> for StakingInfoProvider {
	fn get_staking_details(
		_who: &AccountId,
	) -> Option<pallet_staking_score::StakingDetails<Balance>> {
		// People parachain doesn't have direct staking - return None
		None
	}
}

impl pallet_staking_score::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_staking_score::weights::SubstrateWeight<Runtime>;
	type Balance = Balance;
	type StakingInfo = StakingInfoProvider;
}

// =============================================================================
// Collective Pallet Configuration (for governance)
// =============================================================================

parameter_types! {
	pub const CouncilMotionDuration: BlockNumber = 7 * DAYS;
	pub const CouncilMaxProposals: u32 = 100;
	pub const CouncilMaxMembers: u32 = 100;
	pub MaxProposalWeight: Weight = sp_runtime::Perbill::from_percent(50) * RuntimeBlockWeights::get().max_block;
}

type CouncilCollective = pallet_collective::Instance1;
impl pallet_collective::Config<CouncilCollective> for Runtime {
	type RuntimeOrigin = RuntimeOrigin;
	type Proposal = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type MotionDuration = CouncilMotionDuration;
	type MaxProposals = CouncilMaxProposals;
	type MaxMembers = CouncilMaxMembers;
	type DefaultVote = pallet_collective::PrimeDefaultVote;
	type WeightInfo = pallet_collective::weights::SubstrateWeight<Runtime>;
	type SetMembersOrigin = EnsureRoot<AccountId>;
	type MaxProposalWeight = MaxProposalWeight;
	type DisapproveOrigin = EnsureRoot<AccountId>;
	type KillOrigin = EnsureRoot<AccountId>;
	type Consideration = ();
}

// =============================================================================
// Trust Score Pallet Configuration
// =============================================================================

parameter_types! {
	/// Base multiplier for trust score calculation
	pub const ScoreMultiplierBase: u128 = 100;
	/// Update interval for trust scores (roughly 1 day in blocks)
	pub const TrustUpdateInterval: BlockNumber = DAYS;
	/// Maximum batch size for trust score updates
	pub const TrustMaxBatchSize: u32 = 100;
}

/// Staking score source for Trust pallet
/// Uses the StakingScore pallet to get composite staking scores
pub struct StakingScoreSource;
impl pallet_trust::StakingScoreProvider<AccountId, BlockNumber> for StakingScoreSource {
	fn get_staking_score(who: &AccountId) -> (pallet_staking_score::RawScore, BlockNumber) {
		// Delegate to StakingScore pallet
		<StakingScore as pallet_staking_score::StakingScoreProvider<AccountId, BlockNumber>>::get_staking_score(who)
	}
}

/// Referral score source for Trust pallet
pub struct ReferralScoreSource;
impl pallet_trust::ReferralScoreProvider<AccountId> for ReferralScoreSource {
	fn get_referral_score(who: &AccountId) -> u32 {
		Referral::referral_count(who)
	}
}

/// Perwerde (education) score source for Trust pallet
pub struct PerwerdeScoreSource;
impl pallet_trust::PerwerdeScoreProvider<AccountId> for PerwerdeScoreSource {
	fn get_perwerde_score(_who: &AccountId) -> u32 {
		0 // Placeholder - Perwerde pallet integration needed
	}
}

/// Tiki score source for Trust pallet
pub struct TikiScoreSource;
impl pallet_trust::TikiScoreProvider<AccountId> for TikiScoreSource {
	fn get_tiki_score(who: &AccountId) -> u32 {
		<Tiki as pallet_tiki::TikiScoreProvider<AccountId>>::get_tiki_score(who)
	}
}

/// Citizenship status source for Trust pallet
pub struct CitizenshipSource;
impl pallet_trust::CitizenshipStatusProvider<AccountId> for CitizenshipSource {
	fn is_citizen(who: &AccountId) -> bool {
		IdentityKyc::is_citizen(who)
	}
}

impl pallet_trust::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_trust::weights::SubstrateWeight<Runtime>;
	type Score = u128;
	type ScoreMultiplierBase = ScoreMultiplierBase;
	type UpdateInterval = TrustUpdateInterval;
	type MaxBatchSize = TrustMaxBatchSize;
	type StakingScoreSource = StakingScoreSource;
	type ReferralScoreSource = ReferralScoreSource;
	type PerwerdeScoreSource = PerwerdeScoreSource;
	type TikiScoreSource = TikiScoreSource;
	type CitizenshipSource = CitizenshipSource;
}
