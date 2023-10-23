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

//! Autogenerated weights for project_token
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-10-18, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("prod-test"), DB CACHE: 1024

// Executed Command:
// ./../target/release/joystream-node
// benchmark
// pallet
// --pallet=project_token
// --extrinsic=*
// --chain=prod-test
// --steps=50
// --repeat=20
// --execution=wasm
// --template=./../devops/joystream-pallet-weight-template.hbs
// --output=./../runtime-modules/project-token/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions needed for project_token.
pub trait WeightInfo {
	fn transfer(_o: u32, _m: u32, ) -> Weight;
	fn dust_account() -> Weight;
	fn join_whitelist(_h: u32, ) -> Weight;
	fn purchase_tokens_on_sale() -> Weight;
	fn participate_in_split() -> Weight;
	fn exit_revenue_split() -> Weight;
	fn set_frozen_status() -> Weight;
	fn burn() -> Weight;
}

/// Weights for project_token using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Token PalletFrozen (r:1 w:0)
	// Proof: Token PalletFrozen (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	// Storage: Membership MembershipById (r:1025 w:0)
	// Proof: Membership MembershipById (max_values: None, max_size: Some(125), added: 2600, mode: MaxEncodedLen)
	// Storage: Token TokenInfoById (r:1 w:1)
	// Proof: Token TokenInfoById (max_values: None, max_size: Some(339), added: 2814, mode: MaxEncodedLen)
	// Storage: Token AccountInfoByTokenAndMember (r:1025 w:1025)
	// Proof: Token AccountInfoByTokenAndMember (max_values: None, max_size: Some(489), added: 2964, mode: MaxEncodedLen)
	// Storage: Token BloatBond (r:1 w:0)
	// Proof: Token BloatBond (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	// Storage: System Account (r:2 w:2)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `o` is `[1, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	fn transfer(o: u32, _m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1800 + o * (165 ±0)`
		//  Estimated: `14591 + o * (5564 ±0)`
		// Minimum execution time: 143_320 nanoseconds.
		Weight::from_parts(144_240_000, 0u64)
			.saturating_add(Weight::from_proof_size(14591))
			// Standard Error: 16_496
			.saturating_add(Weight::from_parts(10_584_246, 0u64).saturating_mul(o.into()))
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(o.into())))
			.saturating_add(T::DbWeight::get().writes(4_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(o.into())))
			.saturating_add(Weight::from_proof_size(5564).saturating_mul(o.into()))
	}
	// Storage: Token PalletFrozen (r:1 w:0)
	// Proof: Token PalletFrozen (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	// Storage: Token TokenInfoById (r:1 w:1)
	// Proof: Token TokenInfoById (max_values: None, max_size: Some(339), added: 2814, mode: MaxEncodedLen)
	// Storage: Token AccountInfoByTokenAndMember (r:1 w:1)
	// Proof: Token AccountInfoByTokenAndMember (max_values: None, max_size: Some(489), added: 2964, mode: MaxEncodedLen)
	// Storage: Membership MembershipById (r:1 w:0)
	// Proof: Membership MembershipById (max_values: None, max_size: Some(125), added: 2600, mode: MaxEncodedLen)
	// Storage: System Account (r:2 w:2)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn dust_account() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1179`
		//  Estimated: `14080`
		// Minimum execution time: 48_750 nanoseconds.
		Weight::from_parts(49_850_000, 0u64)
			.saturating_add(Weight::from_proof_size(14080))
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	// Storage: Token PalletFrozen (r:1 w:0)
	// Proof: Token PalletFrozen (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	// Storage: Membership MembershipById (r:1 w:0)
	// Proof: Membership MembershipById (max_values: None, max_size: Some(125), added: 2600, mode: MaxEncodedLen)
	// Storage: Token TokenInfoById (r:1 w:1)
	// Proof: Token TokenInfoById (max_values: None, max_size: Some(339), added: 2814, mode: MaxEncodedLen)
	// Storage: Token AccountInfoByTokenAndMember (r:1 w:1)
	// Proof: Token AccountInfoByTokenAndMember (max_values: None, max_size: Some(489), added: 2964, mode: MaxEncodedLen)
	// Storage: Token BloatBond (r:1 w:0)
	// Proof: Token BloatBond (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	// Storage: System Account (r:2 w:2)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `h` is `[1, 10]`.
	fn join_whitelist(h: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1173`
		//  Estimated: `14591`
		// Minimum execution time: 62_730 nanoseconds.
		Weight::from_parts(63_196_984, 0u64)
			.saturating_add(Weight::from_proof_size(14591))
			// Standard Error: 11_202
			.saturating_add(Weight::from_parts(898_552, 0u64).saturating_mul(h.into()))
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	// Storage: Token PalletFrozen (r:1 w:0)
	// Proof: Token PalletFrozen (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	// Storage: Membership MembershipById (r:1 w:0)
	// Proof: Membership MembershipById (max_values: None, max_size: Some(125), added: 2600, mode: MaxEncodedLen)
	// Storage: Token TokenInfoById (r:1 w:1)
	// Proof: Token TokenInfoById (max_values: None, max_size: Some(339), added: 2814, mode: MaxEncodedLen)
	// Storage: Token SalePlatformFee (r:1 w:0)
	// Proof: Token SalePlatformFee (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: Token AccountInfoByTokenAndMember (r:1 w:1)
	// Proof: Token AccountInfoByTokenAndMember (max_values: None, max_size: Some(489), added: 2964, mode: MaxEncodedLen)
	// Storage: Token BloatBond (r:1 w:0)
	// Proof: Token BloatBond (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	// Storage: System Account (r:3 w:3)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn purchase_tokens_on_sale() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1441`
		//  Estimated: `17693`
		// Minimum execution time: 86_070 nanoseconds.
		Weight::from_parts(87_520_000, 0u64)
			.saturating_add(Weight::from_proof_size(17693))
			.saturating_add(T::DbWeight::get().reads(9_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	// Storage: Token PalletFrozen (r:1 w:0)
	// Proof: Token PalletFrozen (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	// Storage: Membership MembershipById (r:1 w:0)
	// Proof: Membership MembershipById (max_values: None, max_size: Some(125), added: 2600, mode: MaxEncodedLen)
	// Storage: Token TokenInfoById (r:1 w:1)
	// Proof: Token TokenInfoById (max_values: None, max_size: Some(339), added: 2814, mode: MaxEncodedLen)
	// Storage: Token AccountInfoByTokenAndMember (r:1 w:1)
	// Proof: Token AccountInfoByTokenAndMember (max_values: None, max_size: Some(489), added: 2964, mode: MaxEncodedLen)
	// Storage: System Account (r:2 w:2)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn participate_in_split() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1589`
		//  Estimated: `14080`
		// Minimum execution time: 59_400 nanoseconds.
		Weight::from_parts(60_310_000, 0u64)
			.saturating_add(Weight::from_proof_size(14080))
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	// Storage: Token PalletFrozen (r:1 w:0)
	// Proof: Token PalletFrozen (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	// Storage: Membership MembershipById (r:1 w:0)
	// Proof: Membership MembershipById (max_values: None, max_size: Some(125), added: 2600, mode: MaxEncodedLen)
	// Storage: Token TokenInfoById (r:1 w:0)
	// Proof: Token TokenInfoById (max_values: None, max_size: Some(339), added: 2814, mode: MaxEncodedLen)
	// Storage: Token AccountInfoByTokenAndMember (r:1 w:1)
	// Proof: Token AccountInfoByTokenAndMember (max_values: None, max_size: Some(489), added: 2964, mode: MaxEncodedLen)
	fn exit_revenue_split() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1211`
		//  Estimated: `8874`
		// Minimum execution time: 33_961 nanoseconds.
		Weight::from_parts(35_640_000, 0u64)
			.saturating_add(Weight::from_proof_size(8874))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: Token PalletFrozen (r:0 w:1)
	// Proof: Token PalletFrozen (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	fn set_frozen_status() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 7_370 nanoseconds.
		Weight::from_parts(7_650_000, 0u64)
			.saturating_add(Weight::from_proof_size(0))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: Token PalletFrozen (r:1 w:0)
	// Proof: Token PalletFrozen (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	// Storage: Membership MembershipById (r:1 w:0)
	// Proof: Membership MembershipById (max_values: None, max_size: Some(125), added: 2600, mode: MaxEncodedLen)
	// Storage: Token TokenInfoById (r:1 w:1)
	// Proof: Token TokenInfoById (max_values: None, max_size: Some(339), added: 2814, mode: MaxEncodedLen)
	// Storage: Token AccountInfoByTokenAndMember (r:1 w:1)
	// Proof: Token AccountInfoByTokenAndMember (max_values: None, max_size: Some(489), added: 2964, mode: MaxEncodedLen)
	fn burn() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1095`
		//  Estimated: `8874`
		// Minimum execution time: 34_340 nanoseconds.
		Weight::from_parts(35_220_000, 0u64)
			.saturating_add(Weight::from_proof_size(8874))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}

// Default implementation for tests
impl WeightInfo for () {
	fn transfer(o: u32, _m: u32, ) -> Weight {
		Weight::from_parts(0, 0)
	}
	fn dust_account() -> Weight {
		Weight::from_parts(0, 0)
	}
	fn join_whitelist(h: u32, ) -> Weight {
		Weight::from_parts(0, 0)
	}
	fn purchase_tokens_on_sale() -> Weight {
		Weight::from_parts(0, 0)
	}
	fn participate_in_split() -> Weight {
		Weight::from_parts(0, 0)
	}
	fn exit_revenue_split() -> Weight {
		Weight::from_parts(0, 0)
	}
	fn set_frozen_status() -> Weight {
		Weight::from_parts(0, 0)
	}
	fn burn() -> Weight {
		Weight::from_parts(0, 0)
	}
}
