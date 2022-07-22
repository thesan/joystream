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

//! Autogenerated weights for content
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-07-22, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./scripts/../target/release/joystream-node
// benchmark
// pallet
// --pallet=content
// --extrinsic=*
// --chain=dev
// --steps=50
// --repeat=20
// --execution=wasm
// --template=./scripts/../devops/joystream-pallet-weight-template.hbs
// --output=.

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for content.
pub trait WeightInfo {
	fn create_channel(_a: u32, _b: u32, _c: u32, _d: u32, ) -> Weight;
	fn create_curator_group(_a: u32, ) -> Weight;
	fn update_curator_group_permissions(_a: u32, ) -> Weight;
	fn set_curator_group_status() -> Weight;
	fn add_curator_to_group() -> Weight;
	fn remove_curator_from_group() -> Weight;
	fn create_video_without_nft(_a: u32, _b: u32, _c: u32, ) -> Weight;
	fn create_video_with_nft(_a: u32, _b: u32, _c: u32, _d: u32, ) -> Weight;
	fn update_video_without_assets_without_nft() -> Weight;
	fn update_video_with_assets_without_nft(_a: u32, _b: u32, _c: u32, _d: u32, ) -> Weight;
	fn update_video_without_assets_with_nft(_a: u32, ) -> Weight;
	fn update_video_with_assets_with_nft(_a: u32, _b: u32, _c: u32, _d: u32, _e: u32, ) -> Weight;
}

/// Weights for content using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc13020dc69e85870ac7b4c755bb8753c2] (r:2 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b87aa6eccf0cc6941ba2e31cdb5870e3229] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b872d56750ffbaedbf3dd8dd3900c756381] (r:1 w:1)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad235899829563c4064c97520e04fce94e] (r:1 w:1)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad97a953f295d54035e7cdf8d29308e498] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad25e0482900c98a1856a1e4878ed6eac6] (r:1 w:0)
	// Storage: unknown [0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad3323e092df90358439e7c6649f66d93f] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6adbaa937139b20b131cc2fcb0072c015f8] (r:20 w:20)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad33807bc23ee2cb31454339d2b2c6b0a8] (r:101 w:101)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6add6fd5c94c285d60cbe96c66929f01c31] (r:1 w:1)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad047d54b603e8604dc1c09c8e0fdc9dc8] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6adb7612c99e31defd01cd5a28e9967e208] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6adade883233841e9338c8e73f6b9f74890] (r:0 w:10)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b8727cd538b4597adba02e6593ace94e804] (r:0 w:1)
	fn create_channel(a: u32, b: u32, c: u32, d: u32, ) -> Weight {
		(160_164_000 as Weight)
			// Standard Error: 413_000
			.saturating_add((7_602_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 212_000
			.saturating_add((10_238_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 23_000
			.saturating_add((12_760_000 as Weight).saturating_mul(c as Weight))
			// Standard Error: 413_000
			.saturating_add((9_243_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(11 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(a as Weight)))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(b as Weight)))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(c as Weight)))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(b as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(c as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(d as Weight)))
	}
	// Storage: unknown [0x43c6aad67b8d5d8180583e494c8ec4573091994c5737d8f16ba1c53919a94bf2] (r:1 w:0)
	// Storage: unknown [0x43c6aad67b8d5d8180583e494c8ec457b88c49b6e6ccae735eb57de6295caf6a] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b870c0ce290812b08a3418d76f63fc3b322] (r:1 w:1)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b87af96ab8605611c2e562953c9f5fafe05] (r:0 w:1)
	fn create_curator_group(a: u32, ) -> Weight {
		(35_613_000 as Weight)
			// Standard Error: 9_000
			.saturating_add((1_931_000 as Weight).saturating_mul(a as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: unknown [0x43c6aad67b8d5d8180583e494c8ec4573091994c5737d8f16ba1c53919a94bf2] (r:1 w:0)
	// Storage: unknown [0x43c6aad67b8d5d8180583e494c8ec457b88c49b6e6ccae735eb57de6295caf6a] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b87af96ab8605611c2e562953c9f5fafe05] (r:1 w:1)
	fn update_curator_group_permissions(a: u32, ) -> Weight {
		(124_225_000 as Weight)
			// Standard Error: 31_000
			.saturating_add((1_631_000 as Weight).saturating_mul(a as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0x43c6aad67b8d5d8180583e494c8ec4573091994c5737d8f16ba1c53919a94bf2] (r:1 w:0)
	// Storage: unknown [0x43c6aad67b8d5d8180583e494c8ec457b88c49b6e6ccae735eb57de6295caf6a] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b87af96ab8605611c2e562953c9f5fafe05] (r:1 w:1)
	fn set_curator_group_status() -> Weight {
		(119_660_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0x43c6aad67b8d5d8180583e494c8ec4573091994c5737d8f16ba1c53919a94bf2] (r:1 w:0)
	// Storage: unknown [0x43c6aad67b8d5d8180583e494c8ec457b88c49b6e6ccae735eb57de6295caf6a] (r:2 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b87af96ab8605611c2e562953c9f5fafe05] (r:1 w:1)
	fn add_curator_to_group() -> Weight {
		(191_163_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0x43c6aad67b8d5d8180583e494c8ec4573091994c5737d8f16ba1c53919a94bf2] (r:1 w:0)
	// Storage: unknown [0x43c6aad67b8d5d8180583e494c8ec457b88c49b6e6ccae735eb57de6295caf6a] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b87af96ab8605611c2e562953c9f5fafe05] (r:1 w:1)
	fn remove_curator_from_group() -> Weight {
		(183_886_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b8727cd538b4597adba02e6593ace94e804] (r:1 w:1)
	// Storage: unknown [0x43c6aad67b8d5d8180583e494c8ec457b88c49b6e6ccae735eb57de6295caf6a] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b87af96ab8605611c2e562953c9f5fafe05] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6add6fd5c94c285d60cbe96c66929f01c31] (r:1 w:1)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b875c7260a41224aace7b4b98b0edcfd652] (r:1 w:1)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b874a4356070c8776bf14beca4fff00aa2d] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad235899829563c4064c97520e04fce94e] (r:1 w:1)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad97a953f295d54035e7cdf8d29308e498] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad25e0482900c98a1856a1e4878ed6eac6] (r:1 w:0)
	// Storage: unknown [0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9] (r:2 w:2)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad047d54b603e8604dc1c09c8e0fdc9dc8] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6adb7612c99e31defd01cd5a28e9967e208] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6adbaa937139b20b131cc2fcb0072c015f8] (r:20 w:20)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad33807bc23ee2cb31454339d2b2c6b0a8] (r:101 w:101)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6adade883233841e9338c8e73f6b9f74890] (r:0 w:1)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b870af9e4882a7d4dfc4aa025e76973af62] (r:0 w:1)
	fn create_video_without_nft(a: u32, b: u32, c: u32, ) -> Weight {
		(299_347_000 as Weight)
			// Standard Error: 129_000
			.saturating_add((8_021_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 175_000
			.saturating_add((12_188_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 19_000
			.saturating_add((9_199_000 as Weight).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(13 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(b as Weight)))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(c as Weight)))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(a as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(b as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(c as Weight)))
	}
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b8727cd538b4597adba02e6593ace94e804] (r:1 w:1)
	// Storage: unknown [0x43c6aad67b8d5d8180583e494c8ec457b88c49b6e6ccae735eb57de6295caf6a] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b87af96ab8605611c2e562953c9f5fafe05] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6add6fd5c94c285d60cbe96c66929f01c31] (r:1 w:1)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b875c7260a41224aace7b4b98b0edcfd652] (r:1 w:1)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b878fcac5fb69cd7149f5d142817326cd4f] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b87f9ad4eaa35a4c52d9289acbc42eba9d9] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b8728ddfed5d1473440d52323ba831817ae] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b877eeddc9ade82616dd2b2522920104f47] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b87a7a293d9925f4ae46443ea58e41d0904] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b871ce624c36fa09833f33e5287f370d756] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b8799806df27cdcf1eb83a25d651bf93c2d] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b87cbb19eafcf52ef3196a3966a6214aa9d] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b872b3448b5048347b84cf9031e0e5dd85d] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b871b10931eafb6faa5fa01f0cf89f95940] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b8701f92f535ddd83122720f4e9929b95b2] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b87f65b6d352abb4d7727263feb7398e759] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b874a4356070c8776bf14beca4fff00aa2d] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad235899829563c4064c97520e04fce94e] (r:1 w:1)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad97a953f295d54035e7cdf8d29308e498] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad25e0482900c98a1856a1e4878ed6eac6] (r:1 w:0)
	// Storage: unknown [0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9] (r:2 w:2)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b87d2c14024f1b303fdc87019c4c1facfde] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad047d54b603e8604dc1c09c8e0fdc9dc8] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6adb7612c99e31defd01cd5a28e9967e208] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6adbaa937139b20b131cc2fcb0072c015f8] (r:20 w:20)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad33807bc23ee2cb31454339d2b2c6b0a8] (r:101 w:101)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b8713013e1b58f6706b9bc1d1f2461e2668] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b8780475d76dbd965b5ffe4c9edf3b044a5] (r:1 w:1)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b87163a6537c0073cca32731acb69cf63e2] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b87fbf3e09a262eab22b5614cc059547717] (r:1 w:1)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6adade883233841e9338c8e73f6b9f74890] (r:0 w:1)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b870af9e4882a7d4dfc4aa025e76973af62] (r:0 w:1)
	fn create_video_with_nft(a: u32, b: u32, c: u32, d: u32, ) -> Weight {
		(384_750_000 as Weight)
			// Standard Error: 124_000
			.saturating_add((7_794_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 169_000
			.saturating_add((11_848_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 18_000
			.saturating_add((9_262_000 as Weight).saturating_mul(c as Weight))
			// Standard Error: 37_000
			.saturating_add((266_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(30 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(b as Weight)))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(c as Weight)))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(a as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(b as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(c as Weight)))
	}
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b870af9e4882a7d4dfc4aa025e76973af62] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b8727cd538b4597adba02e6593ace94e804] (r:1 w:0)
	// Storage: unknown [0x43c6aad67b8d5d8180583e494c8ec457b88c49b6e6ccae735eb57de6295caf6a] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b87af96ab8605611c2e562953c9f5fafe05] (r:1 w:0)
	fn update_video_without_assets_without_nft() -> Weight {
		(223_366_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
	}
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b870af9e4882a7d4dfc4aa025e76973af62] (r:1 w:1)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b8727cd538b4597adba02e6593ace94e804] (r:1 w:0)
	// Storage: unknown [0x43c6aad67b8d5d8180583e494c8ec457b88c49b6e6ccae735eb57de6295caf6a] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b87af96ab8605611c2e562953c9f5fafe05] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6add6fd5c94c285d60cbe96c66929f01c31] (r:1 w:1)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad235899829563c4064c97520e04fce94e] (r:1 w:1)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad25e0482900c98a1856a1e4878ed6eac6] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad97a953f295d54035e7cdf8d29308e498] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad047d54b603e8604dc1c09c8e0fdc9dc8] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6adb7612c99e31defd01cd5a28e9967e208] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6adade883233841e9338c8e73f6b9f74890] (r:20 w:21)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6adbaa937139b20b131cc2fcb0072c015f8] (r:20 w:20)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad33807bc23ee2cb31454339d2b2c6b0a8] (r:101 w:101)
	// Storage: unknown [0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9] (r:2 w:2)
	fn update_video_with_assets_without_nft(a: u32, b: u32, c: u32, d: u32, ) -> Weight {
		(339_635_000 as Weight)
			// Standard Error: 79_000
			.saturating_add((6_178_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 79_000
			.saturating_add((8_972_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 107_000
			.saturating_add((10_876_000 as Weight).saturating_mul(c as Weight))
			// Standard Error: 11_000
			.saturating_add((9_164_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(12 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(b as Weight)))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(c as Weight)))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(d as Weight)))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(a as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(b as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(c as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(d as Weight)))
	}
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b870af9e4882a7d4dfc4aa025e76973af62] (r:1 w:1)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b8727cd538b4597adba02e6593ace94e804] (r:1 w:1)
	// Storage: unknown [0x43c6aad67b8d5d8180583e494c8ec457b88c49b6e6ccae735eb57de6295caf6a] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b87af96ab8605611c2e562953c9f5fafe05] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b878fcac5fb69cd7149f5d142817326cd4f] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b87f9ad4eaa35a4c52d9289acbc42eba9d9] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b8728ddfed5d1473440d52323ba831817ae] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b877eeddc9ade82616dd2b2522920104f47] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b87a7a293d9925f4ae46443ea58e41d0904] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b871ce624c36fa09833f33e5287f370d756] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b8799806df27cdcf1eb83a25d651bf93c2d] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b87cbb19eafcf52ef3196a3966a6214aa9d] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b872b3448b5048347b84cf9031e0e5dd85d] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b871b10931eafb6faa5fa01f0cf89f95940] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b8701f92f535ddd83122720f4e9929b95b2] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b87f65b6d352abb4d7727263feb7398e759] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b87d2c14024f1b303fdc87019c4c1facfde] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b8713013e1b58f6706b9bc1d1f2461e2668] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b8780475d76dbd965b5ffe4c9edf3b044a5] (r:1 w:1)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b87163a6537c0073cca32731acb69cf63e2] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b87fbf3e09a262eab22b5614cc059547717] (r:1 w:1)
	fn update_video_without_assets_with_nft(a: u32, ) -> Weight {
		(287_882_000 as Weight)
			// Standard Error: 28_000
			.saturating_add((308_000 as Weight).saturating_mul(a as Weight))
			.saturating_add(T::DbWeight::get().reads(21 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b870af9e4882a7d4dfc4aa025e76973af62] (r:1 w:1)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b8727cd538b4597adba02e6593ace94e804] (r:1 w:1)
	// Storage: unknown [0x43c6aad67b8d5d8180583e494c8ec457b88c49b6e6ccae735eb57de6295caf6a] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b87af96ab8605611c2e562953c9f5fafe05] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6add6fd5c94c285d60cbe96c66929f01c31] (r:1 w:1)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad235899829563c4064c97520e04fce94e] (r:1 w:1)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b878fcac5fb69cd7149f5d142817326cd4f] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b87f9ad4eaa35a4c52d9289acbc42eba9d9] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b8728ddfed5d1473440d52323ba831817ae] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b877eeddc9ade82616dd2b2522920104f47] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b87a7a293d9925f4ae46443ea58e41d0904] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b871ce624c36fa09833f33e5287f370d756] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b8799806df27cdcf1eb83a25d651bf93c2d] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b87cbb19eafcf52ef3196a3966a6214aa9d] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b872b3448b5048347b84cf9031e0e5dd85d] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b871b10931eafb6faa5fa01f0cf89f95940] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b8701f92f535ddd83122720f4e9929b95b2] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b87f65b6d352abb4d7727263feb7398e759] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b87d2c14024f1b303fdc87019c4c1facfde] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad25e0482900c98a1856a1e4878ed6eac6] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad97a953f295d54035e7cdf8d29308e498] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad047d54b603e8604dc1c09c8e0fdc9dc8] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6adb7612c99e31defd01cd5a28e9967e208] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6adade883233841e9338c8e73f6b9f74890] (r:20 w:21)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6adbaa937139b20b131cc2fcb0072c015f8] (r:20 w:20)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad33807bc23ee2cb31454339d2b2c6b0a8] (r:101 w:101)
	// Storage: unknown [0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9] (r:2 w:2)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b8713013e1b58f6706b9bc1d1f2461e2668] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b8780475d76dbd965b5ffe4c9edf3b044a5] (r:1 w:1)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b87163a6537c0073cca32731acb69cf63e2] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b87fbf3e09a262eab22b5614cc059547717] (r:1 w:1)
	fn update_video_with_assets_with_nft(a: u32, b: u32, c: u32, d: u32, e: u32, ) -> Weight {
		(209_099_000 as Weight)
			// Standard Error: 245_000
			.saturating_add((10_189_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 245_000
			.saturating_add((12_577_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 337_000
			.saturating_add((14_516_000 as Weight).saturating_mul(c as Weight))
			// Standard Error: 35_000
			.saturating_add((9_476_000 as Weight).saturating_mul(d as Weight))
			// Standard Error: 73_000
			.saturating_add((669_000 as Weight).saturating_mul(e as Weight))
			.saturating_add(T::DbWeight::get().reads(29 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(b as Weight)))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(c as Weight)))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(d as Weight)))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(a as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(b as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(c as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(d as Weight)))
	}
}

// Default implementation for tests
impl WeightInfo for () {
	fn create_channel(a: u32, b: u32, c: u32, d: u32, ) -> Weight {
		0
	}
	fn create_curator_group(a: u32, ) -> Weight {
		0
	}
	fn update_curator_group_permissions(a: u32, ) -> Weight {
		0
	}
	fn set_curator_group_status() -> Weight {
		0
	}
	fn add_curator_to_group() -> Weight {
		0
	}
	fn remove_curator_from_group() -> Weight {
		0
	}
	fn create_video_without_nft(a: u32, b: u32, c: u32, ) -> Weight {
		0
	}
	fn create_video_with_nft(a: u32, b: u32, c: u32, d: u32, ) -> Weight {
		0
	}
	fn update_video_without_assets_without_nft() -> Weight {
		0
	}
	fn update_video_with_assets_without_nft(a: u32, b: u32, c: u32, d: u32, ) -> Weight {
		0
	}
	fn update_video_without_assets_with_nft(a: u32, ) -> Weight {
		0
	}
	fn update_video_with_assets_with_nft(a: u32, b: u32, c: u32, d: u32, e: u32, ) -> Weight {
		0
	}
}
