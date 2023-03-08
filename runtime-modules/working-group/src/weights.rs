// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for working_group
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-10-20, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./../target/release/joystream-node
// benchmark
// pallet
// --base-path=/mnt/disks/local-ssd/
// --pallet=working_group
// --extrinsic=*
// --chain=dev
// --steps=50
// --repeat=20
// --execution=wasm
// --template=./../devops/joystream-pallet-weight-template.hbs
// --output=./../runtime-modules/working-group/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions needed for working_group.
pub trait WeightInfo {
	fn on_initialize_leaving(_i: u32, ) -> Weight;
	fn on_initialize_rewarding_with_missing_reward(_i: u32, ) -> Weight;
	fn on_initialize_rewarding_with_missing_reward_cant_pay(_i: u32, ) -> Weight;
	fn on_initialize_rewarding_without_missing_reward(_i: u32, ) -> Weight;
	fn apply_on_opening(_i: u32, ) -> Weight;
	fn fill_opening_lead() -> Weight;
	fn fill_opening_worker(_i: u32, ) -> Weight;
	fn update_role_account() -> Weight;
	fn cancel_opening() -> Weight;
	fn withdraw_application() -> Weight;
	fn slash_stake(_i: u32, ) -> Weight;
	fn terminate_role_worker(_i: u32, ) -> Weight;
	fn terminate_role_lead(_i: u32, ) -> Weight;
	fn increase_stake() -> Weight;
	fn decrease_stake() -> Weight;
	fn spend_from_budget() -> Weight;
	fn fund_working_group_budget() -> Weight;
	fn update_reward_amount() -> Weight;
	fn set_status_text(_i: u32, ) -> Weight;
	fn update_reward_account() -> Weight;
	fn set_budget() -> Weight;
	fn add_opening(_i: u32, ) -> Weight;
	fn leave_role(_i: u32, ) -> Weight;
	fn lead_remark(_i: u32, ) -> Weight;
	fn worker_remark(_i: u32, ) -> Weight;
}

/// Weights for working_group using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Instance3WorkingGroup WorkerById (r:3 w:2)
	// Storage: Instance3WorkingGroup CurrentLead (r:1 w:1)
	// Storage: Instance3WorkingGroup Budget (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Instance3WorkingGroup ActiveWorkerCount (r:1 w:1)
	// Storage: Balances Locks (r:2 w:2)
	fn on_initialize_leaving(i: u32, ) -> Weight {
		Weight::from_ref_time(26_873_000)
			// Standard Error: 35_000
			.saturating_add(Weight::from_ref_time(55_730_000).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(i.into())))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(i.into())))
	}
	// Storage: Instance3WorkingGroup WorkerById (r:3 w:2)
	// Storage: Instance3WorkingGroup Budget (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	fn on_initialize_rewarding_with_missing_reward(i: u32, ) -> Weight {
		Weight::from_ref_time(17_787_000)
			// Standard Error: 55_000
			.saturating_add(Weight::from_ref_time(52_759_000).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(i.into())))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(i.into())))
	}
	// Storage: Instance3WorkingGroup WorkerById (r:3 w:2)
	// Storage: Instance3WorkingGroup Budget (r:1 w:0)
	fn on_initialize_rewarding_with_missing_reward_cant_pay(i: u32, ) -> Weight {
		Weight::from_ref_time(16_415_000)
			// Standard Error: 33_000
			.saturating_add(Weight::from_ref_time(23_020_000).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(i.into())))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(i.into())))
	}
	// Storage: Instance3WorkingGroup WorkerById (r:3 w:1)
	// Storage: Instance3WorkingGroup Budget (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	fn on_initialize_rewarding_without_missing_reward(i: u32, ) -> Weight {
		Weight::from_ref_time(38_161_000)
			// Standard Error: 33_000
			.saturating_add(Weight::from_ref_time(33_132_000).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(i.into())))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(i.into())))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: Instance3WorkingGroup OpeningById (r:1 w:0)
	// Storage: Membership StakingAccountIdMemberStatus (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Instance3WorkingGroup NextApplicationId (r:1 w:1)
	// Storage: Instance3WorkingGroup ApplicationById (r:0 w:1)
	fn apply_on_opening(i: u32, ) -> Weight {
		Weight::from_ref_time(60_759_000)
			// Standard Error: 3_000
			.saturating_add(Weight::from_ref_time(1_699_000).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	// Storage: Instance3WorkingGroup OpeningById (r:1 w:1)
	// Storage: Instance3WorkingGroup ActiveWorkerCount (r:1 w:1)
	// Storage: Instance3WorkingGroup CurrentLead (r:1 w:1)
	// Storage: Instance3WorkingGroup ApplicationById (r:1 w:1)
	// Storage: Instance3WorkingGroup NextWorkerId (r:1 w:1)
	// Storage: Instance3WorkingGroup WorkerById (r:0 w:1)
	fn fill_opening_lead() -> Weight {
		Weight::from_ref_time(49_500_000)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	// Storage: Instance3WorkingGroup OpeningById (r:1 w:1)
	// Storage: Instance3WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance3WorkingGroup WorkerById (r:1 w:1)
	// Storage: Instance3WorkingGroup ActiveWorkerCount (r:1 w:1)
	// Storage: Instance3WorkingGroup ApplicationById (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Instance3WorkingGroup NextWorkerId (r:1 w:1)
	fn fill_opening_worker(i: u32, ) -> Weight {
		Weight::from_ref_time(51_093_000)
			// Standard Error: 19_000
			.saturating_add(Weight::from_ref_time(15_263_000).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(i.into())))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(i.into())))
	}
	// Storage: Instance3WorkingGroup WorkerById (r:1 w:1)
	// Storage: Membership MembershipById (r:1 w:0)
	fn update_role_account() -> Weight {
		Weight::from_ref_time(28_550_000)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: Instance3WorkingGroup OpeningById (r:1 w:1)
	// Storage: Instance3WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance3WorkingGroup WorkerById (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn cancel_opening() -> Weight {
		Weight::from_ref_time(57_880_000)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: Instance3WorkingGroup ApplicationById (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn withdraw_application() -> Weight {
		Weight::from_ref_time(37_370_000)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: Instance3WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance3WorkingGroup WorkerById (r:2 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn slash_stake(i: u32, ) -> Weight {
		Weight::from_ref_time(77_648_000)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(660_000).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: Instance3WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance3WorkingGroup WorkerById (r:2 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Instance3WorkingGroup Budget (r:1 w:1)
	// Storage: Instance3WorkingGroup ActiveWorkerCount (r:1 w:1)
	fn terminate_role_worker(i: u32, ) -> Weight {
		Weight::from_ref_time(122_568_000)
			// Standard Error: 5_000
			.saturating_add(Weight::from_ref_time(1_271_000).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	// Storage: Instance3WorkingGroup CurrentLead (r:1 w:1)
	// Storage: Instance3WorkingGroup WorkerById (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Instance3WorkingGroup Budget (r:1 w:1)
	// Storage: Instance3WorkingGroup ActiveWorkerCount (r:1 w:1)
	fn terminate_role_lead(i: u32, ) -> Weight {
		Weight::from_ref_time(121_275_000)
			// Standard Error: 3_000
			.saturating_add(Weight::from_ref_time(1_264_000).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	// Storage: Instance3WorkingGroup WorkerById (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn increase_stake() -> Weight {
		Weight::from_ref_time(48_210_000)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: Instance3WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance3WorkingGroup WorkerById (r:2 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn decrease_stake() -> Weight {
		Weight::from_ref_time(57_370_000)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: Instance3WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance3WorkingGroup WorkerById (r:1 w:0)
	// Storage: Instance3WorkingGroup Budget (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn spend_from_budget() -> Weight {
		Weight::from_ref_time(42_270_000)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Instance3WorkingGroup Budget (r:1 w:1)
	fn fund_working_group_budget() -> Weight {
		Weight::from_ref_time(42_300_000)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: Instance3WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance3WorkingGroup WorkerById (r:2 w:1)
	fn update_reward_amount() -> Weight {
		Weight::from_ref_time(33_980_000)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: Instance3WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance3WorkingGroup WorkerById (r:1 w:0)
	// Storage: Instance3WorkingGroup StatusTextHash (r:0 w:1)
	fn set_status_text(i: u32, ) -> Weight {
		Weight::from_ref_time(27_279_000)
			// Standard Error: 6_000
			.saturating_add(Weight::from_ref_time(1_644_000).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: Instance3WorkingGroup WorkerById (r:1 w:1)
	fn update_reward_account() -> Weight {
		Weight::from_ref_time(24_380_000)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: Instance3WorkingGroup Budget (r:0 w:1)
	fn set_budget() -> Weight {
		Weight::from_ref_time(13_550_000)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: Instance3WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance3WorkingGroup WorkerById (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Instance3WorkingGroup NextOpeningId (r:1 w:1)
	// Storage: Instance3WorkingGroup OpeningById (r:0 w:1)
	fn add_opening(i: u32, ) -> Weight {
		Weight::from_ref_time(69_789_000)
			// Standard Error: 6_000
			.saturating_add(Weight::from_ref_time(1_729_000).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	// Storage: Instance3WorkingGroup WorkerById (r:1 w:1)
	fn leave_role(i: u32, ) -> Weight {
		Weight::from_ref_time(25_746_000)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(668_000).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: Instance3WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance3WorkingGroup WorkerById (r:1 w:0)
	fn lead_remark(i: u32, ) -> Weight {
		Weight::from_ref_time(24_744_000)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(735_000).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
	}
	// Storage: Instance3WorkingGroup WorkerById (r:1 w:0)
	fn worker_remark(i: u32, ) -> Weight {
		Weight::from_ref_time(23_822_000)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(735_000).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
}

// Default implementation for tests
impl WeightInfo for () {
	fn on_initialize_leaving(i: u32, ) -> Weight {
		Weight::from_ref_time(0)
	}
	fn on_initialize_rewarding_with_missing_reward(i: u32, ) -> Weight {
		Weight::from_ref_time(0)
	}
	fn on_initialize_rewarding_with_missing_reward_cant_pay(i: u32, ) -> Weight {
		Weight::from_ref_time(0)
	}
	fn on_initialize_rewarding_without_missing_reward(i: u32, ) -> Weight {
		Weight::from_ref_time(0)
	}
	fn apply_on_opening(i: u32, ) -> Weight {
		Weight::from_ref_time(0)
	}
	fn fill_opening_lead() -> Weight {
		Weight::from_ref_time(0)
	}
	fn fill_opening_worker(i: u32, ) -> Weight {
		Weight::from_ref_time(0)
	}
	fn update_role_account() -> Weight {
		Weight::from_ref_time(0)
	}
	fn cancel_opening() -> Weight {
		Weight::from_ref_time(0)
	}
	fn withdraw_application() -> Weight {
		Weight::from_ref_time(0)
	}
	fn slash_stake(i: u32, ) -> Weight {
		Weight::from_ref_time(0)
	}
	fn terminate_role_worker(i: u32, ) -> Weight {
		Weight::from_ref_time(0)
	}
	fn terminate_role_lead(i: u32, ) -> Weight {
		Weight::from_ref_time(0)
	}
	fn increase_stake() -> Weight {
		Weight::from_ref_time(0)
	}
	fn decrease_stake() -> Weight {
		Weight::from_ref_time(0)
	}
	fn spend_from_budget() -> Weight {
		Weight::from_ref_time(0)
	}
	fn fund_working_group_budget() -> Weight {
		Weight::from_ref_time(0)
	}
	fn update_reward_amount() -> Weight {
		Weight::from_ref_time(0)
	}
	fn set_status_text(i: u32, ) -> Weight {
		Weight::from_ref_time(0)
	}
	fn update_reward_account() -> Weight {
		Weight::from_ref_time(0)
	}
	fn set_budget() -> Weight {
		Weight::from_ref_time(0)
	}
	fn add_opening(i: u32, ) -> Weight {
		Weight::from_ref_time(0)
	}
	fn leave_role(i: u32, ) -> Weight {
		Weight::from_ref_time(0)
	}
	fn lead_remark(i: u32, ) -> Weight {
		Weight::from_ref_time(0)
	}
	fn worker_remark(i: u32, ) -> Weight {
		Weight::from_ref_time(0)
	}
}
