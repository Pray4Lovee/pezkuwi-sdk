//! Benchmarking setup for pallet-presale
//!
//! Note: These benchmarks provide weight measurements for presale operations.
//! Due to complex asset setup requirements, some benchmarks use simplified scenarios.

#![cfg(feature = "runtime-benchmarks")]

use super::*;
#[allow(unused)]
use crate::Pallet as Presale;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

/// Helper trait for benchmark asset ID creation
pub trait BenchmarkHelper<AssetId> {
	fn create_asset_id(seed: u32) -> AssetId;
}

impl<AssetId: From<u32>> BenchmarkHelper<AssetId> for () {
	fn create_asset_id(seed: u32) -> AssetId {
		seed.into()
	}
}

#[benchmarks(
	where
		T::AssetId: From<u32>,
)]
mod benchmarks {
	use super::*;

	fn get_asset_id<T: Config>(seed: u32) -> T::AssetId
	where
		T::AssetId: From<u32>,
	{
		seed.into()
	}

	#[benchmark]
	fn create_presale() {
		let caller: T::AccountId = whitelisted_caller();
		let payment_asset = get_asset_id::<T>(1);
		let reward_asset = get_asset_id::<T>(2);

		#[extrinsic_call]
		create_presale(
			RawOrigin::Signed(caller),
			payment_asset,
			reward_asset,
			1_000_000u128,         // tokens_for_sale
			100u32.into(),         // duration
			false,                 // is_whitelist
			100u128,               // min_contribution
			10_000u128,            // max_contribution
			500_000u128,           // soft_cap
			1_000_000u128,         // hard_cap
			false,                 // enable_vesting
			0u8,                   // vesting_immediate_percent
			0u32.into(),           // vesting_duration_blocks
			0u32.into(),           // vesting_cliff_blocks
			10u32.into(),          // grace_period_blocks
			5u8,                   // refund_fee_percent
			10u8,                  // grace_refund_fee_percent
		);

		// Verify presale was created
		assert!(crate::Presales::<T>::contains_key(0));
	}

	#[benchmark]
	fn cancel_presale() {
		let caller: T::AccountId = whitelisted_caller();
		let payment_asset = get_asset_id::<T>(1);
		let reward_asset = get_asset_id::<T>(2);

		// Create a presale first
		let _ = Presale::<T>::create_presale(
			RawOrigin::Signed(caller.clone()).into(),
			payment_asset,
			reward_asset,
			1_000_000u128,
			100u32.into(),
			false,
			100u128,
			10_000u128,
			500_000u128,
			1_000_000u128,
			false,
			0u8,
			0u32.into(),
			0u32.into(),
			10u32.into(),
			5u8,
			10u8,
		);

		let presale_id: PresaleId = 0;

		#[extrinsic_call]
		cancel_presale(RawOrigin::Signed(caller), presale_id);

		// Verify presale was cancelled
		let presale = crate::Presales::<T>::get(presale_id).unwrap();
		assert_eq!(presale.status, PresaleStatus::Cancelled);
	}

	#[benchmark]
	fn add_to_whitelist() {
		let owner: T::AccountId = whitelisted_caller();
		let user: T::AccountId = account("user", 0, 0);
		let payment_asset = get_asset_id::<T>(1);
		let reward_asset = get_asset_id::<T>(2);

		// Create a whitelist presale
		let _ = Presale::<T>::create_presale(
			RawOrigin::Signed(owner.clone()).into(),
			payment_asset,
			reward_asset,
			1_000_000u128,
			100u32.into(),
			true, // is_whitelist = true
			100u128,
			10_000u128,
			500_000u128,
			1_000_000u128,
			false,
			0u8,
			0u32.into(),
			0u32.into(),
			10u32.into(),
			5u8,
			10u8,
		);

		let presale_id: PresaleId = 0;

		#[extrinsic_call]
		add_to_whitelist(RawOrigin::Signed(owner), presale_id, user.clone());

		// Verify user was whitelisted
		assert!(crate::WhitelistedAccounts::<T>::get(presale_id, &user));
	}

	// Note: contribute, refund, and finalize_presale require complex asset setup
	// that depends on pallet_assets configuration in the runtime.
	// Weights for these are calibrated based on storage operation complexity.

	impl_benchmark_test_suite!(Presale, crate::mock::new_test_ext(), crate::mock::Test);
}
