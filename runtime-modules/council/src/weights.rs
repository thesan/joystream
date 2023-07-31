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

//! Autogenerated weights for council
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-07-31, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("prod-test"), DB CACHE: 1024

// Executed Command:
// ./../target/release/joystream-node
// benchmark
// pallet
// --pallet=council
// --extrinsic=*
// --chain=prod-test
// --steps=50
// --repeat=20
// --execution=wasm
// --template=./../devops/joystream-pallet-weight-template.hbs
// --output=./../runtime-modules/council/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions needed for council.
pub trait WeightInfo {
	fn set_budget_increment() -> Weight;
	fn set_councilor_reward() -> Weight;
	fn funding_request(_i: u32, ) -> Weight;
	fn try_process_budget_refill_budget_only() -> Weight;
	fn try_process_budget_payout_council_members_only() -> Weight;
	fn try_progress_stage_idle() -> Weight;
	fn try_progress_stage_announcing_start_election(_i: u32, ) -> Weight;
	fn try_progress_stage_announcing_restart() -> Weight;
	fn announce_candidacy() -> Weight;
	fn release_candidacy_stake() -> Weight;
	fn set_candidacy_note(_i: u32, ) -> Weight;
	fn withdraw_candidacy() -> Weight;
	fn set_budget() -> Weight;
	fn plan_budget_refill() -> Weight;
	fn fund_council_budget() -> Weight;
	fn candidate_remark() -> Weight;
	fn councilor_remark() -> Weight;
}

/// Weights for council using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Council BudgetIncrement (r:0 w:1)
	// Proof: Council BudgetIncrement (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	fn set_budget_increment() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_000 nanoseconds.
		Weight::from_parts(7_000_000, 0u64)
			.saturating_add(Weight::from_proof_size(0))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: Council CouncilorReward (r:0 w:1)
	// Proof: Council CouncilorReward (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	fn set_councilor_reward() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_000 nanoseconds.
		Weight::from_parts(7_000_000, 0u64)
			.saturating_add(Weight::from_proof_size(0))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: Council Budget (r:1 w:1)
	// Proof: Council Budget (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	// Storage: System Account (r:100 w:100)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `i` is `[1, 100]`.
	fn funding_request(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `714 + i * (2 ±0)`
		//  Estimated: `511 + i * (2603 ±0)`
		// Minimum execution time: 30_000 nanoseconds.
		Weight::from_parts(9_946_559, 0u64)
			.saturating_add(Weight::from_proof_size(511))
			// Standard Error: 5_136
			.saturating_add(Weight::from_parts(19_998_149, 0u64).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(i.into())))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(i.into())))
			.saturating_add(Weight::from_proof_size(2603).saturating_mul(i.into()))
	}
	// Storage: Council BudgetIncrement (r:1 w:0)
	// Proof: Council BudgetIncrement (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	// Storage: Council Budget (r:1 w:1)
	// Proof: Council Budget (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	// Storage: Council NextBudgetRefill (r:0 w:1)
	// Proof: Council NextBudgetRefill (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn try_process_budget_refill_budget_only() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `276`
		//  Estimated: `1022`
		// Minimum execution time: 13_000 nanoseconds.
		Weight::from_parts(14_000_000, 0u64)
			.saturating_add(Weight::from_proof_size(1022))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: Council CouncilorReward (r:1 w:0)
	// Proof: Council CouncilorReward (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	// Storage: Council Budget (r:1 w:1)
	// Proof: Council Budget (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	// Storage: Council CouncilMembers (r:1 w:1)
	// Proof: Council CouncilMembers (max_values: Some(1), max_size: Some(325), added: 820, mode: MaxEncodedLen)
	// Storage: System Account (r:3 w:3)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Council NextRewardPayments (r:0 w:1)
	// Proof: Council NextRewardPayments (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn try_process_budget_payout_council_members_only() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1169`
		//  Estimated: `9651`
		// Minimum execution time: 45_000 nanoseconds.
		Weight::from_parts(46_000_000, 0u64)
			.saturating_add(Weight::from_proof_size(9651))
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	// Storage: Council Stage (r:1 w:1)
	// Proof: Council Stage (max_values: Some(1), max_size: Some(13), added: 508, mode: MaxEncodedLen)
	// Storage: Council AnnouncementPeriodNr (r:1 w:1)
	// Proof: Council AnnouncementPeriodNr (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	fn try_progress_stage_idle() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `270`
		//  Estimated: `1011`
		// Minimum execution time: 9_000 nanoseconds.
		Weight::from_parts(10_000_000, 0u64)
			.saturating_add(Weight::from_proof_size(1011))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: Council Stage (r:1 w:1)
	// Proof: Council Stage (max_values: Some(1), max_size: Some(13), added: 508, mode: MaxEncodedLen)
	// Storage: Council AnnouncementPeriodNr (r:1 w:0)
	// Proof: Council AnnouncementPeriodNr (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	// Storage: Instance1Referendum Stage (r:1 w:1)
	// Proof: Instance1Referendum Stage (max_values: Some(1), max_size: Some(94), added: 589, mode: MaxEncodedLen)
	/// The range of component `i` is `[3, 103]`.
	fn try_progress_stage_announcing_start_election(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `379`
		//  Estimated: `1600`
		// Minimum execution time: 15_000 nanoseconds.
		Weight::from_parts(16_653_162, 0u64)
			.saturating_add(Weight::from_proof_size(1600))
			// Standard Error: 774
			.saturating_add(Weight::from_parts(7_175, 0u64).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: Council Stage (r:1 w:1)
	// Proof: Council Stage (max_values: Some(1), max_size: Some(13), added: 508, mode: MaxEncodedLen)
	// Storage: Council AnnouncementPeriodNr (r:1 w:1)
	// Proof: Council AnnouncementPeriodNr (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	fn try_progress_stage_announcing_restart() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `274`
		//  Estimated: `1011`
		// Minimum execution time: 9_000 nanoseconds.
		Weight::from_parts(10_000_000, 0u64)
			.saturating_add(Weight::from_proof_size(1011))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Proof: Membership MembershipById (max_values: None, max_size: Some(125), added: 2600, mode: MaxEncodedLen)
	// Storage: Membership StakingAccountIdMemberStatus (r:1 w:0)
	// Proof: Membership StakingAccountIdMemberStatus (max_values: None, max_size: Some(57), added: 2532, mode: MaxEncodedLen)
	// Storage: Balances Locks (r:1 w:1)
	// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	// Storage: Council Stage (r:1 w:1)
	// Proof: Council Stage (max_values: Some(1), max_size: Some(13), added: 508, mode: MaxEncodedLen)
	// Storage: Council Candidates (r:1 w:1)
	// Proof: Council Candidates (max_values: None, max_size: Some(161), added: 2636, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Council AnnouncementPeriodNr (r:1 w:0)
	// Proof: Council AnnouncementPeriodNr (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	fn announce_candidacy() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `877`
		//  Estimated: `15156`
		// Minimum execution time: 38_000 nanoseconds.
		Weight::from_parts(39_000_000, 0u64)
			.saturating_add(Weight::from_proof_size(15156))
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Proof: Membership MembershipById (max_values: None, max_size: Some(125), added: 2600, mode: MaxEncodedLen)
	// Storage: Council Candidates (r:1 w:1)
	// Proof: Council Candidates (max_values: None, max_size: Some(161), added: 2636, mode: MaxEncodedLen)
	// Storage: Council AnnouncementPeriodNr (r:1 w:0)
	// Proof: Council AnnouncementPeriodNr (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	// Storage: Balances Locks (r:1 w:1)
	// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn release_candidacy_stake() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1074`
		//  Estimated: `12116`
		// Minimum execution time: 29_000 nanoseconds.
		Weight::from_parts(30_000_000, 0u64)
			.saturating_add(Weight::from_proof_size(12116))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Proof: Membership MembershipById (max_values: None, max_size: Some(125), added: 2600, mode: MaxEncodedLen)
	// Storage: Council Candidates (r:1 w:1)
	// Proof: Council Candidates (max_values: None, max_size: Some(161), added: 2636, mode: MaxEncodedLen)
	// Storage: Council AnnouncementPeriodNr (r:1 w:0)
	// Proof: Council AnnouncementPeriodNr (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	// Storage: Council Stage (r:1 w:0)
	// Proof: Council Stage (max_values: Some(1), max_size: Some(13), added: 508, mode: MaxEncodedLen)
	/// The range of component `i` is `[0, 100]`.
	fn set_candidacy_note(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `744`
		//  Estimated: `6247`
		// Minimum execution time: 20_000 nanoseconds.
		Weight::from_parts(21_760_948, 0u64)
			.saturating_add(Weight::from_proof_size(6247))
			// Standard Error: 1_606
			.saturating_add(Weight::from_parts(1_487_056, 0u64).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Proof: Membership MembershipById (max_values: None, max_size: Some(125), added: 2600, mode: MaxEncodedLen)
	// Storage: Council Candidates (r:1 w:1)
	// Proof: Council Candidates (max_values: None, max_size: Some(161), added: 2636, mode: MaxEncodedLen)
	// Storage: Council Stage (r:1 w:1)
	// Proof: Council Stage (max_values: Some(1), max_size: Some(13), added: 508, mode: MaxEncodedLen)
	// Storage: Council AnnouncementPeriodNr (r:1 w:0)
	// Proof: Council AnnouncementPeriodNr (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	// Storage: Balances Locks (r:1 w:1)
	// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn withdraw_candidacy() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1074`
		//  Estimated: `12624`
		// Minimum execution time: 31_000 nanoseconds.
		Weight::from_parts(32_000_000, 0u64)
			.saturating_add(Weight::from_proof_size(12624))
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	// Storage: Council Budget (r:0 w:1)
	// Proof: Council Budget (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	fn set_budget() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_000 nanoseconds.
		Weight::from_parts(7_000_000, 0u64)
			.saturating_add(Weight::from_proof_size(0))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: Council NextBudgetRefill (r:0 w:1)
	// Proof: Council NextBudgetRefill (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn plan_budget_refill() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_000 nanoseconds.
		Weight::from_parts(6_000_000, 0u64)
			.saturating_add(Weight::from_proof_size(0))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Proof: Membership MembershipById (max_values: None, max_size: Some(125), added: 2600, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Council Budget (r:1 w:1)
	// Proof: Council Budget (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	fn fund_council_budget() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `730`
		//  Estimated: `5714`
		// Minimum execution time: 28_000 nanoseconds.
		Weight::from_parts(28_000_000, 0u64)
			.saturating_add(Weight::from_proof_size(5714))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Proof: Membership MembershipById (max_values: None, max_size: Some(125), added: 2600, mode: MaxEncodedLen)
	// Storage: Council Candidates (r:1 w:0)
	// Proof: Council Candidates (max_values: None, max_size: Some(161), added: 2636, mode: MaxEncodedLen)
	// Storage: Council AnnouncementPeriodNr (r:1 w:0)
	// Proof: Council AnnouncementPeriodNr (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	fn candidate_remark() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `744`
		//  Estimated: `5739`
		// Minimum execution time: 16_000 nanoseconds.
		Weight::from_parts(17_000_000, 0u64)
			.saturating_add(Weight::from_proof_size(5739))
			.saturating_add(T::DbWeight::get().reads(3_u64))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Proof: Membership MembershipById (max_values: None, max_size: Some(125), added: 2600, mode: MaxEncodedLen)
	// Storage: Council CouncilMembers (r:1 w:0)
	// Proof: Council CouncilMembers (max_values: Some(1), max_size: Some(325), added: 820, mode: MaxEncodedLen)
	fn councilor_remark() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1006`
		//  Estimated: `3420`
		// Minimum execution time: 17_000 nanoseconds.
		Weight::from_parts(18_000_000, 0u64)
			.saturating_add(Weight::from_proof_size(3420))
			.saturating_add(T::DbWeight::get().reads(2_u64))
	}
}

// Default implementation for tests
impl WeightInfo for () {
	fn set_budget_increment() -> Weight {
		Weight::from_parts(0, 0)
	}
	fn set_councilor_reward() -> Weight {
		Weight::from_parts(0, 0)
	}
	fn funding_request(i: u32, ) -> Weight {
		Weight::from_parts(0, 0)
	}
	fn try_process_budget_refill_budget_only() -> Weight {
		Weight::from_parts(0, 0)
	}
	fn try_process_budget_payout_council_members_only() -> Weight {
		Weight::from_parts(0, 0)
	}
	fn try_progress_stage_idle() -> Weight {
		Weight::from_parts(0, 0)
	}
	fn try_progress_stage_announcing_start_election(i: u32, ) -> Weight {
		Weight::from_parts(0, 0)
	}
	fn try_progress_stage_announcing_restart() -> Weight {
		Weight::from_parts(0, 0)
	}
	fn announce_candidacy() -> Weight {
		Weight::from_parts(0, 0)
	}
	fn release_candidacy_stake() -> Weight {
		Weight::from_parts(0, 0)
	}
	fn set_candidacy_note(i: u32, ) -> Weight {
		Weight::from_parts(0, 0)
	}
	fn withdraw_candidacy() -> Weight {
		Weight::from_parts(0, 0)
	}
	fn set_budget() -> Weight {
		Weight::from_parts(0, 0)
	}
	fn plan_budget_refill() -> Weight {
		Weight::from_parts(0, 0)
	}
	fn fund_council_budget() -> Weight {
		Weight::from_parts(0, 0)
	}
	fn candidate_remark() -> Weight {
		Weight::from_parts(0, 0)
	}
	fn councilor_remark() -> Weight {
		Weight::from_parts(0, 0)
	}
}
