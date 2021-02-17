//! Autogenerated weights for pallet_merkle
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-02-15, STEPS: [20, ], REPEAT: 5, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB
//! CACHE: 128

// Executed Command:
// ./target/release/node-template
// benchmark
// --chain
// dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet_merkle
// --extrinsic
// *
// --steps
// 20
// --repeat
// 5
// --output
// ./pallets/merkle/src/

#![allow(unused_parens)]
#![allow(unused_imports)]

use crate::Config;
use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_merkle.
pub trait WeightInfo {
	fn create_group(n: u32) -> Weight;
	fn set_manager_required() -> Weight;
	fn set_manager() -> Weight;
	fn set_stopped() -> Weight;
	fn add_members(n: u32) -> Weight;
	fn verify_path(n: u32) -> Weight;
}

/// Weight functions for pallet_merkle.
pub struct Weights<T>(PhantomData<T>);
impl<T: frame_system::Config + Config> WeightInfo for Weights<T> {
	fn create_group(d: u32) -> Weight {
		(23_261_000 as Weight)
			// Standard Error: 22_000
			.saturating_add((740_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}

	fn set_manager_required() -> Weight {
		(15_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}

	fn set_manager() -> Weight {
		(20_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}

	fn set_stopped() -> Weight {
		(19_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}

	fn add_members(n: u32) -> Weight {
		(5_461_878_889_000 as Weight)
			// Standard Error: 8_395_614_000
			.saturating_add((283_628_366_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}

	fn verify_path(_d: u32) -> Weight {
		(6_145_869_533_000 as Weight).saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
}

// Implementation used for development
impl WeightInfo for () {
	fn create_group(_: u32) -> Weight {
		(60_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}

	fn set_manager_required() -> Weight {
		(17_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}

	fn set_manager() -> Weight {
		(21_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}

	fn set_stopped() -> Weight {
		(20_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}

	fn add_members(_: u32) -> Weight {
		(5_377_768_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}

	fn verify_path(_d: u32) -> Weight {
		(6_145_869_533_000 as Weight).saturating_add(RocksDbWeight::get().reads(1 as Weight))
	}
}
