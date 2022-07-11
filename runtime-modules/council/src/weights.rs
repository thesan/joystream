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
//! DATE: 2022-06-28, STEPS: `5`, REPEAT: 5, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./scripts/../target/release/joystream-node
// benchmark
// pallet
// --pallet=council
// --extrinsic=*
// --chain=dev
// --steps=5
// --repeat=5
// --execution=wasm
// --template=./scripts/../devops/joystream-pallet-weight-template.hbs
// --output=.

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for council.
pub trait WeightInfo {
	fn set_budget_increment() -> Weight;
	fn set_councilor_reward() -> Weight;
	fn funding_request(i: u32, ) -> Weight;
	fn try_process_budget_refill_budget_only() -> Weight;
	fn try_process_budget_payout_council_members_only() -> Weight;
	fn try_progress_stage_idle() -> Weight;
	fn try_progress_stage_announcing_start_election(i: u32, ) -> Weight;
	fn try_progress_stage_announcing_restart() -> Weight;
	fn announce_candidacy() -> Weight;
	fn release_candidacy_stake() -> Weight;
	fn set_candidacy_note(i: u32, ) -> Weight;
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
	// Storage: unknown [0xaebd463ed9925c488c112434d61debc01cfe551387afc457060907bd88f3fe73] (r:0 w:1)
	fn set_budget_increment() -> Weight {
		(9_000_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0xaebd463ed9925c488c112434d61debc0ab08902841cf95786fe297d638ba90dc] (r:0 w:1)
	fn set_councilor_reward() -> Weight {
		(9_000_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0xaebd463ed9925c488c112434d61debc0f3928fc443e8d9cca27b4e39e5c29cac] (r:1 w:1)
	// Storage: unknown [0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9] (r:1 w:1)
	fn funding_request(i: u32, ) -> Weight {
		(3_201_000 as Weight)
			// Standard Error: 88_000
			.saturating_add((18_419_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(i as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(i as Weight)))
	}
	// Storage: unknown [0xaebd463ed9925c488c112434d61debc01cfe551387afc457060907bd88f3fe73] (r:1 w:0)
	// Storage: unknown [0xaebd463ed9925c488c112434d61debc0f3928fc443e8d9cca27b4e39e5c29cac] (r:1 w:1)
	// Storage: unknown [0xaebd463ed9925c488c112434d61debc00d40fdfa7a9fbeaecd4ef7340de7e35a] (r:0 w:1)
	fn try_process_budget_refill_budget_only() -> Weight {
		(17_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: unknown [0xaebd463ed9925c488c112434d61debc0ab08902841cf95786fe297d638ba90dc] (r:1 w:0)
	// Storage: unknown [0xaebd463ed9925c488c112434d61debc0f3928fc443e8d9cca27b4e39e5c29cac] (r:1 w:1)
	// Storage: unknown [0xaebd463ed9925c488c112434d61debc03c4161dd5c06ffffe50605fefac36ad8] (r:1 w:1)
	// Storage: unknown [0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9] (r:5 w:5)
	// Storage: unknown [0xaebd463ed9925c488c112434d61debc049d670f9f10c14d280a941872345c86a] (r:0 w:1)
	fn try_process_budget_payout_council_members_only() -> Weight {
		(72_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
	}
	// Storage: unknown [0xaebd463ed9925c488c112434d61debc05c84d490439bb889d4d6ffec931483ee] (r:1 w:1)
	// Storage: unknown [0xaebd463ed9925c488c112434d61debc0d5266cafd38af6684c76bb87c647c4be] (r:1 w:1)
	fn try_progress_stage_idle() -> Weight {
		(386_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: unknown [0xaebd463ed9925c488c112434d61debc05c84d490439bb889d4d6ffec931483ee] (r:1 w:1)
	// Storage: unknown [0xaebd463ed9925c488c112434d61debc0d5266cafd38af6684c76bb87c647c4be] (r:1 w:0)
	// Storage: unknown [0x7c7ee947105fdd14e3fa8953bc2a2f825c84d490439bb889d4d6ffec931483ee] (r:1 w:1)
	fn try_progress_stage_announcing_start_election(i: u32, ) -> Weight {
		(85_220_000 as Weight)
			// Standard Error: 66_000
			.saturating_add((59_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: unknown [0xaebd463ed9925c488c112434d61debc05c84d490439bb889d4d6ffec931483ee] (r:1 w:1)
	// Storage: unknown [0xaebd463ed9925c488c112434d61debc0d5266cafd38af6684c76bb87c647c4be] (r:1 w:1)
	fn try_progress_stage_announcing_restart() -> Weight {
		(96_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc13020dc69e85870ac7b4c755bb8753c2] (r:1 w:0)
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc8b18453086fa74ddec96f7b48109d8f4] (r:1 w:0)
	// Storage: unknown [0xc2261276cc9d1f8598ea4b6a74b15c2f218f26c73add634897550b4003b26bc6] (r:1 w:1)
	// Storage: unknown [0xaebd463ed9925c488c112434d61debc05c84d490439bb889d4d6ffec931483ee] (r:1 w:1)
	// Storage: unknown [0xaebd463ed9925c488c112434d61debc0948ece45793d7f15c9c0b9574ddbc665] (r:1 w:1)
	// Storage: unknown [0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9] (r:1 w:1)
	// Storage: unknown [0xaebd463ed9925c488c112434d61debc0d5266cafd38af6684c76bb87c647c4be] (r:1 w:0)
	fn announce_candidacy() -> Weight {
		(39_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc13020dc69e85870ac7b4c755bb8753c2] (r:1 w:0)
	// Storage: unknown [0xaebd463ed9925c488c112434d61debc0948ece45793d7f15c9c0b9574ddbc665] (r:1 w:1)
	// Storage: unknown [0xaebd463ed9925c488c112434d61debc0d5266cafd38af6684c76bb87c647c4be] (r:1 w:0)
	// Storage: unknown [0xc2261276cc9d1f8598ea4b6a74b15c2f218f26c73add634897550b4003b26bc6] (r:1 w:1)
	// Storage: unknown [0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9] (r:1 w:1)
	fn release_candidacy_stake() -> Weight {
		(28_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc13020dc69e85870ac7b4c755bb8753c2] (r:1 w:0)
	// Storage: unknown [0xaebd463ed9925c488c112434d61debc0948ece45793d7f15c9c0b9574ddbc665] (r:1 w:1)
	// Storage: unknown [0xaebd463ed9925c488c112434d61debc0d5266cafd38af6684c76bb87c647c4be] (r:1 w:0)
	// Storage: unknown [0xaebd463ed9925c488c112434d61debc05c84d490439bb889d4d6ffec931483ee] (r:1 w:0)
	fn set_candidacy_note(i: u32, ) -> Weight {
		(21_571_000 as Weight)
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc13020dc69e85870ac7b4c755bb8753c2] (r:1 w:0)
	// Storage: unknown [0xaebd463ed9925c488c112434d61debc0948ece45793d7f15c9c0b9574ddbc665] (r:1 w:1)
	// Storage: unknown [0xaebd463ed9925c488c112434d61debc05c84d490439bb889d4d6ffec931483ee] (r:1 w:1)
	// Storage: unknown [0xaebd463ed9925c488c112434d61debc0d5266cafd38af6684c76bb87c647c4be] (r:1 w:0)
	// Storage: unknown [0xc2261276cc9d1f8598ea4b6a74b15c2f218f26c73add634897550b4003b26bc6] (r:1 w:1)
	// Storage: unknown [0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9] (r:1 w:1)
	fn withdraw_candidacy() -> Weight {
		(32_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: unknown [0xaebd463ed9925c488c112434d61debc0f3928fc443e8d9cca27b4e39e5c29cac] (r:0 w:1)
	fn set_budget() -> Weight {
		(9_000_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0xaebd463ed9925c488c112434d61debc00d40fdfa7a9fbeaecd4ef7340de7e35a] (r:0 w:1)
	fn plan_budget_refill() -> Weight {
		(9_000_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc13020dc69e85870ac7b4c755bb8753c2] (r:1 w:0)
	// Storage: unknown [0xaebd463ed9925c488c112434d61debc0f3928fc443e8d9cca27b4e39e5c29cac] (r:1 w:1)
	// Storage: unknown [0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9] (r:1 w:1)
	fn fund_council_budget() -> Weight {
		(27_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc13020dc69e85870ac7b4c755bb8753c2] (r:1 w:0)
	// Storage: unknown [0xaebd463ed9925c488c112434d61debc0948ece45793d7f15c9c0b9574ddbc665] (r:1 w:0)
	// Storage: unknown [0xaebd463ed9925c488c112434d61debc0d5266cafd38af6684c76bb87c647c4be] (r:1 w:0)
	fn candidate_remark() -> Weight {
		(17_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
	}
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc13020dc69e85870ac7b4c755bb8753c2] (r:1 w:0)
	// Storage: unknown [0xaebd463ed9925c488c112434d61debc03c4161dd5c06ffffe50605fefac36ad8] (r:1 w:0)
	fn councilor_remark() -> Weight {
		(17_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
	}
}

// Default implementation for tests
impl WeightInfo for () {
	fn set_budget_increment() -> Weight {
		0
	}
	fn set_councilor_reward() -> Weight {
		0
	}
	fn funding_request(_i: u32, ) -> Weight {
		0
	}
	fn try_process_budget_refill_budget_only() -> Weight {
		0
	}
	fn try_process_budget_payout_council_members_only() -> Weight {
		0
	}
	fn try_progress_stage_idle() -> Weight {
		0
	}
	fn try_progress_stage_announcing_start_election(_i: u32, ) -> Weight {
		0
	}
	fn try_progress_stage_announcing_restart() -> Weight {
		0
	}
	fn announce_candidacy() -> Weight {
		0
	}
	fn release_candidacy_stake() -> Weight {
		0
	}
	fn set_candidacy_note(_i: u32, ) -> Weight {
		0
	}
	fn withdraw_candidacy() -> Weight {
		0
	}
	fn set_budget() -> Weight {
		0
	}
	fn plan_budget_refill() -> Weight {
		0
	}
	fn fund_council_budget() -> Weight {
		0
	}
	fn candidate_remark() -> Weight {
		0
	}
	fn councilor_remark() -> Weight {
		0
	}
}