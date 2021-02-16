#![cfg(feature = "runtime-benchmarks")]

use super::*;
use curve25519_gadgets::poseidon::Poseidon_hash_2;
use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_system::RawOrigin;
use utils::keys::Data;

use crate::Module as Merkle;

const MAX_DEPTH: u8 = 32;
const NUM_LEAVES: u32 = 10;
const VERIFY_DEPT: u8 = 10;

fn setup_tree<T: Config>(caller: T::AccountId, depth: u32) {
	let manager_required = true;
	<Merkle<T> as Group<T::AccountId, T::BlockNumber, T::GroupId>>::create_group(caller, manager_required, depth as u8);
}

fn get_proof(depth: u32) -> Vec<(bool, Data)> {
	let hasher = default_hasher();
	let mut d = Data::zero();
	let mut path = Vec::new();
	for i in 0..depth {
		path.push((true, d));
		d = Data(Poseidon_hash_2(d.0, d.0, &hasher));
	}
	path
}

benchmarks! {
	create_group {
		// Testing the function for all depths between 0 to 32
		// Creates a weight function that accepts tree depth
		// and calculates the weights on the run
		let d in 1 .. MAX_DEPTH as u32;
		let caller = whitelisted_caller();
	}: _(RawOrigin::Signed(caller), false, Some(d as u8))
	verify {
		let next_id: T::GroupId = Merkle::<T>::next_group_id();
		let curr_id = next_id - 1.into();
		let group: GroupTree = Groups::<T>::get(curr_id).unwrap();
		assert_eq!(group.depth, d as u8);
	}

	set_manager_required {
		let caller: T::AccountId = whitelisted_caller();
		// Tree is setup with manager required
		setup_tree::<T>(caller.clone(), 32);
	}:
	// Manager is not required initially
	_(RawOrigin::Signed(caller.clone()), 0.into(), false)
	verify {
		// Checking if manager is caller and is not required
		let group_id: T::GroupId = 0.into();
		let manager = Managers::<T>::get(group_id).unwrap();
		assert_eq!(manager.required, false);
		assert_eq!(manager.account_id, caller.into());
	}

	set_manager {
		let caller: T::AccountId = whitelisted_caller();
		// Making an account id for new admin
		let new_admin: T::AccountId = account("new_admin", 0, 0);
		setup_tree::<T>(caller.clone(), 32);
	}:
	// Transfering the admin role to `new_admin`
	_(RawOrigin::Signed(caller), 0.into(), new_admin.clone())
	verify {
		let group_id: T::GroupId = 0.into();
		let manager = Managers::<T>::get(group_id).unwrap();
		assert_eq!(manager.required, true);
		assert_eq!(manager.account_id, new_admin);
	}

	set_stopped {
		let caller: T::AccountId = whitelisted_caller();
		setup_tree::<T>(caller.clone(), 32);
	}:
	// Setting the stopped storage item, this doesnt't effect
	// any other functionality of the tree
	_(RawOrigin::Signed(caller.clone()), 0.into(), true)
	verify {
		let group_id: T::GroupId = 0.into();
		let stopped = Stopped::<T>::get(group_id);
		assert!(stopped);
	}

	add_members {
		// This means that the test will run `NUM_LEAVES` times
		// Each time it runs, new value of `n` will be set
		// This will make weights function based on number of leaves
		let n in 1 .. NUM_LEAVES;
		let caller: T::AccountId = whitelisted_caller();
		// Create leaves based on `n`
		let leaves = vec![Data::zero(); n as usize];

		setup_tree::<T>(caller.clone(), 32);
	}: _(RawOrigin::Signed(caller.clone()), 0.into(), leaves)
	verify {
		let group_id: T::GroupId = 0.into();
		let group_tree: GroupTree = Groups::<T>::get(group_id).unwrap();
		assert_eq!(group_tree.leaf_count, n);
	}

	verify_path {
		let d in 1 .. VERIFY_DEPT as u32;
		let caller: T::AccountId = whitelisted_caller();
		let leaf_data = Data::zero();
		setup_tree::<T>(caller.clone(), d);
		let path = get_proof(d);
	}: verify(RawOrigin::Signed(caller), 0.into(), leaf_data, path)
	verify {
	}
}

#[cfg(test)]
mod bench_tests {
	use super::*;
	use crate::mock::{new_test_ext, Test};
	use frame_support::assert_ok;

	#[test]
	fn test_create_group() {
		new_test_ext().execute_with(|| {
			assert_ok!(test_benchmark_create_group::<Test>());
		});
	}

	#[test]
	fn test_set_manager_required() {
		new_test_ext().execute_with(|| {
			assert_ok!(test_benchmark_set_manager_required::<Test>());
		});
	}

	#[test]
	fn test_set_manager() {
		new_test_ext().execute_with(|| {
			assert_ok!(test_benchmark_set_manager::<Test>());
		});
	}

	#[test]
	fn test_set_stopped() {
		new_test_ext().execute_with(|| {
			assert_ok!(test_benchmark_set_stopped::<Test>());
		});
	}

	#[test]
	fn test_add_members() {
		new_test_ext().execute_with(|| {
			assert_ok!(test_benchmark_add_members::<Test>());
		});
	}

	#[test]
	fn test_verify_path() {
		new_test_ext().execute_with(|| {
			assert_ok!(test_benchmark_verify_path::<Test>());
		});
	}
}