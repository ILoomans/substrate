// This file is part of Substrate.

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

//! Autogenerated weights for pallet_identity
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-03-01, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm5`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/substrate
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_identity
// --no-storage-info
// --no-median-slopes
// --no-min-squares
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./frame/identity/src/weights.rs
// --header=./HEADER-APACHE2
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_identity.
pub trait WeightInfo {
	fn add_registrar(r: u32, ) -> Weight;
	fn set_identity(r: u32, x: u32, ) -> Weight;
	fn set_subs_new(s: u32, ) -> Weight;
	fn set_subs_old(p: u32, ) -> Weight;
	fn clear_identity(r: u32, s: u32, x: u32, ) -> Weight;
	fn request_judgement(r: u32, x: u32, ) -> Weight;
	fn cancel_request(r: u32, x: u32, ) -> Weight;
	fn set_fee(r: u32, ) -> Weight;
	fn set_account_id(r: u32, ) -> Weight;
	fn set_fields(r: u32, ) -> Weight;
	fn provide_judgement(r: u32, x: u32, ) -> Weight;
	fn kill_identity(r: u32, s: u32, x: u32, ) -> Weight;
	fn add_sub(s: u32, ) -> Weight;
	fn rename_sub(s: u32, ) -> Weight;
	fn remove_sub(s: u32, ) -> Weight;
	fn quit_sub(s: u32, ) -> Weight;
}

/// Weights for pallet_identity using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: Identity Registrars (r:1 w:1)
	/// Proof: Identity Registrars (max_values: Some(1), max_size: Some(1141), added: 1636, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 19]`.
	fn add_registrar(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `64 + r * (57 ±0)`
		//  Estimated: `2626`
		// Minimum execution time: 12_642 nanoseconds.
		Weight::from_ref_time(13_265_544)
			.saturating_add(Weight::from_proof_size(2626))
			// Standard Error: 1_619
			.saturating_add(Weight::from_ref_time(113_117).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Identity IdentityOf (r:1 w:1)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7538), added: 10013, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `x` is `[0, 100]`.
	fn set_identity(r: u32, x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `474 + r * (5 ±0)`
		//  Estimated: `11003`
		// Minimum execution time: 30_990 nanoseconds.
		Weight::from_ref_time(30_240_104)
			.saturating_add(Weight::from_proof_size(11003))
			// Standard Error: 2_438
			.saturating_add(Weight::from_ref_time(111_362).saturating_mul(r.into()))
			// Standard Error: 475
			.saturating_add(Weight::from_ref_time(447_815).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Identity IdentityOf (r:1 w:0)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7538), added: 10013, mode: MaxEncodedLen)
	/// Storage: Identity SubsOf (r:1 w:1)
	/// Proof: Identity SubsOf (max_values: None, max_size: Some(3258), added: 5733, mode: MaxEncodedLen)
	/// Storage: Identity SuperOf (r:100 w:100)
	/// Proof: Identity SuperOf (max_values: None, max_size: Some(114), added: 2589, mode: MaxEncodedLen)
	/// The range of component `s` is `[0, 100]`.
	fn set_subs_new(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `101`
		//  Estimated: `18716 + s * (2589 ±0)`
		// Minimum execution time: 10_293 nanoseconds.
		Weight::from_ref_time(24_208_604)
			.saturating_add(Weight::from_proof_size(18716))
			// Standard Error: 4_471
			.saturating_add(Weight::from_ref_time(3_061_564).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(s.into())))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(s.into())))
			.saturating_add(Weight::from_proof_size(2589).saturating_mul(s.into()))
	}
	/// Storage: Identity IdentityOf (r:1 w:0)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7538), added: 10013, mode: MaxEncodedLen)
	/// Storage: Identity SubsOf (r:1 w:1)
	/// Proof: Identity SubsOf (max_values: None, max_size: Some(3258), added: 5733, mode: MaxEncodedLen)
	/// Storage: Identity SuperOf (r:0 w:100)
	/// Proof: Identity SuperOf (max_values: None, max_size: Some(114), added: 2589, mode: MaxEncodedLen)
	/// The range of component `p` is `[0, 100]`.
	fn set_subs_old(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `226 + p * (32 ±0)`
		//  Estimated: `17726`
		// Minimum execution time: 9_976 nanoseconds.
		Weight::from_ref_time(23_810_753)
			.saturating_add(Weight::from_proof_size(17726))
			// Standard Error: 3_590
			.saturating_add(Weight::from_ref_time(1_277_958).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(p.into())))
	}
	/// Storage: Identity SubsOf (r:1 w:1)
	/// Proof: Identity SubsOf (max_values: None, max_size: Some(3258), added: 5733, mode: MaxEncodedLen)
	/// Storage: Identity IdentityOf (r:1 w:1)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7538), added: 10013, mode: MaxEncodedLen)
	/// Storage: Identity SuperOf (r:0 w:100)
	/// Proof: Identity SuperOf (max_values: None, max_size: Some(114), added: 2589, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `s` is `[0, 100]`.
	/// The range of component `x` is `[0, 100]`.
	fn clear_identity(_r: u32, s: u32, x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `533 + r * (5 ±0) + s * (32 ±0) + x * (66 ±0)`
		//  Estimated: `17726`
		// Minimum execution time: 50_393 nanoseconds.
		Weight::from_ref_time(31_961_708)
			.saturating_add(Weight::from_proof_size(17726))
			// Standard Error: 1_520
			.saturating_add(Weight::from_ref_time(1_266_297).saturating_mul(s.into()))
			// Standard Error: 1_520
			.saturating_add(Weight::from_ref_time(215_731).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(s.into())))
	}
	/// Storage: Identity Registrars (r:1 w:0)
	/// Proof: Identity Registrars (max_values: Some(1), max_size: Some(1141), added: 1636, mode: MaxEncodedLen)
	/// Storage: Identity IdentityOf (r:1 w:1)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7538), added: 10013, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `x` is `[0, 100]`.
	fn request_judgement(r: u32, x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `431 + r * (57 ±0) + x * (66 ±0)`
		//  Estimated: `13629`
		// Minimum execution time: 32_755 nanoseconds.
		Weight::from_ref_time(33_109_363)
			.saturating_add(Weight::from_proof_size(13629))
			// Standard Error: 6_341
			.saturating_add(Weight::from_ref_time(58_498).saturating_mul(r.into()))
			// Standard Error: 1_237
			.saturating_add(Weight::from_ref_time(460_586).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Identity IdentityOf (r:1 w:1)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7538), added: 10013, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `x` is `[0, 100]`.
	fn cancel_request(r: u32, x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `430 + x * (66 ±0)`
		//  Estimated: `11003`
		// Minimum execution time: 28_599 nanoseconds.
		Weight::from_ref_time(27_631_249)
			.saturating_add(Weight::from_proof_size(11003))
			// Standard Error: 5_938
			.saturating_add(Weight::from_ref_time(116_642).saturating_mul(r.into()))
			// Standard Error: 1_158
			.saturating_add(Weight::from_ref_time(470_061).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Identity Registrars (r:1 w:1)
	/// Proof: Identity Registrars (max_values: Some(1), max_size: Some(1141), added: 1636, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 19]`.
	fn set_fee(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `121 + r * (57 ±0)`
		//  Estimated: `2626`
		// Minimum execution time: 8_264 nanoseconds.
		Weight::from_ref_time(8_688_917)
			.saturating_add(Weight::from_proof_size(2626))
			// Standard Error: 1_418
			.saturating_add(Weight::from_ref_time(117_704).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Identity Registrars (r:1 w:1)
	/// Proof: Identity Registrars (max_values: Some(1), max_size: Some(1141), added: 1636, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 19]`.
	fn set_account_id(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `121 + r * (57 ±0)`
		//  Estimated: `2626`
		// Minimum execution time: 8_408 nanoseconds.
		Weight::from_ref_time(8_882_124)
			.saturating_add(Weight::from_proof_size(2626))
			// Standard Error: 1_460
			.saturating_add(Weight::from_ref_time(114_491).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Identity Registrars (r:1 w:1)
	/// Proof: Identity Registrars (max_values: Some(1), max_size: Some(1141), added: 1636, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 19]`.
	fn set_fields(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `121 + r * (57 ±0)`
		//  Estimated: `2626`
		// Minimum execution time: 8_239 nanoseconds.
		Weight::from_ref_time(8_601_853)
			.saturating_add(Weight::from_proof_size(2626))
			// Standard Error: 1_522
			.saturating_add(Weight::from_ref_time(105_927).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Identity Registrars (r:1 w:0)
	/// Proof: Identity Registrars (max_values: Some(1), max_size: Some(1141), added: 1636, mode: MaxEncodedLen)
	/// Storage: Identity IdentityOf (r:1 w:1)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7538), added: 10013, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 19]`.
	/// The range of component `x` is `[0, 100]`.
	fn provide_judgement(r: u32, x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `509 + r * (57 ±0) + x * (66 ±0)`
		//  Estimated: `13629`
		// Minimum execution time: 24_979 nanoseconds.
		Weight::from_ref_time(23_620_419)
			.saturating_add(Weight::from_proof_size(13629))
			// Standard Error: 3_599
			.saturating_add(Weight::from_ref_time(161_638).saturating_mul(r.into()))
			// Standard Error: 665
			.saturating_add(Weight::from_ref_time(758_650).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Identity SubsOf (r:1 w:1)
	/// Proof: Identity SubsOf (max_values: None, max_size: Some(3258), added: 5733, mode: MaxEncodedLen)
	/// Storage: Identity IdentityOf (r:1 w:1)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7538), added: 10013, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Identity SuperOf (r:0 w:100)
	/// Proof: Identity SuperOf (max_values: None, max_size: Some(114), added: 2589, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `s` is `[0, 100]`.
	/// The range of component `x` is `[0, 100]`.
	fn kill_identity(r: u32, s: u32, x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `772 + r * (5 ±0) + s * (32 ±0) + x * (66 ±0)`
		//  Estimated: `21319`
		// Minimum execution time: 67_381 nanoseconds.
		Weight::from_ref_time(46_699_663)
			.saturating_add(Weight::from_proof_size(21319))
			// Standard Error: 8_023
			.saturating_add(Weight::from_ref_time(60_414).saturating_mul(r.into()))
			// Standard Error: 1_566
			.saturating_add(Weight::from_ref_time(1_254_260).saturating_mul(s.into()))
			// Standard Error: 1_566
			.saturating_add(Weight::from_ref_time(222_834).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(s.into())))
	}
	/// Storage: Identity IdentityOf (r:1 w:0)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7538), added: 10013, mode: MaxEncodedLen)
	/// Storage: Identity SuperOf (r:1 w:1)
	/// Proof: Identity SuperOf (max_values: None, max_size: Some(114), added: 2589, mode: MaxEncodedLen)
	/// Storage: Identity SubsOf (r:1 w:1)
	/// Proof: Identity SubsOf (max_values: None, max_size: Some(3258), added: 5733, mode: MaxEncodedLen)
	/// The range of component `s` is `[0, 99]`.
	fn add_sub(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `507 + s * (36 ±0)`
		//  Estimated: `21305`
		// Minimum execution time: 28_263 nanoseconds.
		Weight::from_ref_time(33_249_692)
			.saturating_add(Weight::from_proof_size(21305))
			// Standard Error: 1_405
			.saturating_add(Weight::from_ref_time(72_932).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: Identity IdentityOf (r:1 w:0)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7538), added: 10013, mode: MaxEncodedLen)
	/// Storage: Identity SuperOf (r:1 w:1)
	/// Proof: Identity SuperOf (max_values: None, max_size: Some(114), added: 2589, mode: MaxEncodedLen)
	/// The range of component `s` is `[1, 100]`.
	fn rename_sub(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `623 + s * (3 ±0)`
		//  Estimated: `14582`
		// Minimum execution time: 13_801 nanoseconds.
		Weight::from_ref_time(16_176_484)
			.saturating_add(Weight::from_proof_size(14582))
			// Standard Error: 872
			.saturating_add(Weight::from_ref_time(16_387).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Identity IdentityOf (r:1 w:0)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7538), added: 10013, mode: MaxEncodedLen)
	/// Storage: Identity SuperOf (r:1 w:1)
	/// Proof: Identity SuperOf (max_values: None, max_size: Some(114), added: 2589, mode: MaxEncodedLen)
	/// Storage: Identity SubsOf (r:1 w:1)
	/// Proof: Identity SubsOf (max_values: None, max_size: Some(3258), added: 5733, mode: MaxEncodedLen)
	/// The range of component `s` is `[1, 100]`.
	fn remove_sub(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `702 + s * (35 ±0)`
		//  Estimated: `21305`
		// Minimum execution time: 32_242 nanoseconds.
		Weight::from_ref_time(35_470_674)
			.saturating_add(Weight::from_proof_size(21305))
			// Standard Error: 1_181
			.saturating_add(Weight::from_ref_time(61_101).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: Identity SuperOf (r:1 w:1)
	/// Proof: Identity SuperOf (max_values: None, max_size: Some(114), added: 2589, mode: MaxEncodedLen)
	/// Storage: Identity SubsOf (r:1 w:1)
	/// Proof: Identity SubsOf (max_values: None, max_size: Some(3258), added: 5733, mode: MaxEncodedLen)
	/// The range of component `s` is `[0, 99]`.
	fn quit_sub(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `628 + s * (37 ±0)`
		//  Estimated: `10302`
		// Minimum execution time: 20_571 nanoseconds.
		Weight::from_ref_time(23_211_865)
			.saturating_add(Weight::from_proof_size(10302))
			// Standard Error: 1_076
			.saturating_add(Weight::from_ref_time(63_829).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: Identity Registrars (r:1 w:1)
	/// Proof: Identity Registrars (max_values: Some(1), max_size: Some(1141), added: 1636, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 19]`.
	fn add_registrar(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `64 + r * (57 ±0)`
		//  Estimated: `2626`
		// Minimum execution time: 12_642 nanoseconds.
		Weight::from_ref_time(13_265_544)
			.saturating_add(Weight::from_proof_size(2626))
			// Standard Error: 1_619
			.saturating_add(Weight::from_ref_time(113_117).saturating_mul(r.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Identity IdentityOf (r:1 w:1)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7538), added: 10013, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `x` is `[0, 100]`.
	fn set_identity(r: u32, x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `474 + r * (5 ±0)`
		//  Estimated: `11003`
		// Minimum execution time: 30_990 nanoseconds.
		Weight::from_ref_time(30_240_104)
			.saturating_add(Weight::from_proof_size(11003))
			// Standard Error: 2_438
			.saturating_add(Weight::from_ref_time(111_362).saturating_mul(r.into()))
			// Standard Error: 475
			.saturating_add(Weight::from_ref_time(447_815).saturating_mul(x.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Identity IdentityOf (r:1 w:0)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7538), added: 10013, mode: MaxEncodedLen)
	/// Storage: Identity SubsOf (r:1 w:1)
	/// Proof: Identity SubsOf (max_values: None, max_size: Some(3258), added: 5733, mode: MaxEncodedLen)
	/// Storage: Identity SuperOf (r:100 w:100)
	/// Proof: Identity SuperOf (max_values: None, max_size: Some(114), added: 2589, mode: MaxEncodedLen)
	/// The range of component `s` is `[0, 100]`.
	fn set_subs_new(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `101`
		//  Estimated: `18716 + s * (2589 ±0)`
		// Minimum execution time: 10_293 nanoseconds.
		Weight::from_ref_time(24_208_604)
			.saturating_add(Weight::from_proof_size(18716))
			// Standard Error: 4_471
			.saturating_add(Weight::from_ref_time(3_061_564).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(s.into())))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(s.into())))
			.saturating_add(Weight::from_proof_size(2589).saturating_mul(s.into()))
	}
	/// Storage: Identity IdentityOf (r:1 w:0)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7538), added: 10013, mode: MaxEncodedLen)
	/// Storage: Identity SubsOf (r:1 w:1)
	/// Proof: Identity SubsOf (max_values: None, max_size: Some(3258), added: 5733, mode: MaxEncodedLen)
	/// Storage: Identity SuperOf (r:0 w:100)
	/// Proof: Identity SuperOf (max_values: None, max_size: Some(114), added: 2589, mode: MaxEncodedLen)
	/// The range of component `p` is `[0, 100]`.
	fn set_subs_old(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `226 + p * (32 ±0)`
		//  Estimated: `17726`
		// Minimum execution time: 9_976 nanoseconds.
		Weight::from_ref_time(23_810_753)
			.saturating_add(Weight::from_proof_size(17726))
			// Standard Error: 3_590
			.saturating_add(Weight::from_ref_time(1_277_958).saturating_mul(p.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(p.into())))
	}
	/// Storage: Identity SubsOf (r:1 w:1)
	/// Proof: Identity SubsOf (max_values: None, max_size: Some(3258), added: 5733, mode: MaxEncodedLen)
	/// Storage: Identity IdentityOf (r:1 w:1)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7538), added: 10013, mode: MaxEncodedLen)
	/// Storage: Identity SuperOf (r:0 w:100)
	/// Proof: Identity SuperOf (max_values: None, max_size: Some(114), added: 2589, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `s` is `[0, 100]`.
	/// The range of component `x` is `[0, 100]`.
	fn clear_identity(_r: u32, s: u32, x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `533 + r * (5 ±0) + s * (32 ±0) + x * (66 ±0)`
		//  Estimated: `17726`
		// Minimum execution time: 50_393 nanoseconds.
		Weight::from_ref_time(31_961_708)
			.saturating_add(Weight::from_proof_size(17726))
			// Standard Error: 1_520
			.saturating_add(Weight::from_ref_time(1_266_297).saturating_mul(s.into()))
			// Standard Error: 1_520
			.saturating_add(Weight::from_ref_time(215_731).saturating_mul(x.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(s.into())))
	}
	/// Storage: Identity Registrars (r:1 w:0)
	/// Proof: Identity Registrars (max_values: Some(1), max_size: Some(1141), added: 1636, mode: MaxEncodedLen)
	/// Storage: Identity IdentityOf (r:1 w:1)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7538), added: 10013, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `x` is `[0, 100]`.
	fn request_judgement(r: u32, x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `431 + r * (57 ±0) + x * (66 ±0)`
		//  Estimated: `13629`
		// Minimum execution time: 32_755 nanoseconds.
		Weight::from_ref_time(33_109_363)
			.saturating_add(Weight::from_proof_size(13629))
			// Standard Error: 6_341
			.saturating_add(Weight::from_ref_time(58_498).saturating_mul(r.into()))
			// Standard Error: 1_237
			.saturating_add(Weight::from_ref_time(460_586).saturating_mul(x.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Identity IdentityOf (r:1 w:1)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7538), added: 10013, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `x` is `[0, 100]`.
	fn cancel_request(r: u32, x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `430 + x * (66 ±0)`
		//  Estimated: `11003`
		// Minimum execution time: 28_599 nanoseconds.
		Weight::from_ref_time(27_631_249)
			.saturating_add(Weight::from_proof_size(11003))
			// Standard Error: 5_938
			.saturating_add(Weight::from_ref_time(116_642).saturating_mul(r.into()))
			// Standard Error: 1_158
			.saturating_add(Weight::from_ref_time(470_061).saturating_mul(x.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Identity Registrars (r:1 w:1)
	/// Proof: Identity Registrars (max_values: Some(1), max_size: Some(1141), added: 1636, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 19]`.
	fn set_fee(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `121 + r * (57 ±0)`
		//  Estimated: `2626`
		// Minimum execution time: 8_264 nanoseconds.
		Weight::from_ref_time(8_688_917)
			.saturating_add(Weight::from_proof_size(2626))
			// Standard Error: 1_418
			.saturating_add(Weight::from_ref_time(117_704).saturating_mul(r.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Identity Registrars (r:1 w:1)
	/// Proof: Identity Registrars (max_values: Some(1), max_size: Some(1141), added: 1636, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 19]`.
	fn set_account_id(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `121 + r * (57 ±0)`
		//  Estimated: `2626`
		// Minimum execution time: 8_408 nanoseconds.
		Weight::from_ref_time(8_882_124)
			.saturating_add(Weight::from_proof_size(2626))
			// Standard Error: 1_460
			.saturating_add(Weight::from_ref_time(114_491).saturating_mul(r.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Identity Registrars (r:1 w:1)
	/// Proof: Identity Registrars (max_values: Some(1), max_size: Some(1141), added: 1636, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 19]`.
	fn set_fields(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `121 + r * (57 ±0)`
		//  Estimated: `2626`
		// Minimum execution time: 8_239 nanoseconds.
		Weight::from_ref_time(8_601_853)
			.saturating_add(Weight::from_proof_size(2626))
			// Standard Error: 1_522
			.saturating_add(Weight::from_ref_time(105_927).saturating_mul(r.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Identity Registrars (r:1 w:0)
	/// Proof: Identity Registrars (max_values: Some(1), max_size: Some(1141), added: 1636, mode: MaxEncodedLen)
	/// Storage: Identity IdentityOf (r:1 w:1)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7538), added: 10013, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 19]`.
	/// The range of component `x` is `[0, 100]`.
	fn provide_judgement(r: u32, x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `509 + r * (57 ±0) + x * (66 ±0)`
		//  Estimated: `13629`
		// Minimum execution time: 24_979 nanoseconds.
		Weight::from_ref_time(23_620_419)
			.saturating_add(Weight::from_proof_size(13629))
			// Standard Error: 3_599
			.saturating_add(Weight::from_ref_time(161_638).saturating_mul(r.into()))
			// Standard Error: 665
			.saturating_add(Weight::from_ref_time(758_650).saturating_mul(x.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Identity SubsOf (r:1 w:1)
	/// Proof: Identity SubsOf (max_values: None, max_size: Some(3258), added: 5733, mode: MaxEncodedLen)
	/// Storage: Identity IdentityOf (r:1 w:1)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7538), added: 10013, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Identity SuperOf (r:0 w:100)
	/// Proof: Identity SuperOf (max_values: None, max_size: Some(114), added: 2589, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `s` is `[0, 100]`.
	/// The range of component `x` is `[0, 100]`.
	fn kill_identity(r: u32, s: u32, x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `772 + r * (5 ±0) + s * (32 ±0) + x * (66 ±0)`
		//  Estimated: `21319`
		// Minimum execution time: 67_381 nanoseconds.
		Weight::from_ref_time(46_699_663)
			.saturating_add(Weight::from_proof_size(21319))
			// Standard Error: 8_023
			.saturating_add(Weight::from_ref_time(60_414).saturating_mul(r.into()))
			// Standard Error: 1_566
			.saturating_add(Weight::from_ref_time(1_254_260).saturating_mul(s.into()))
			// Standard Error: 1_566
			.saturating_add(Weight::from_ref_time(222_834).saturating_mul(x.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(s.into())))
	}
	/// Storage: Identity IdentityOf (r:1 w:0)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7538), added: 10013, mode: MaxEncodedLen)
	/// Storage: Identity SuperOf (r:1 w:1)
	/// Proof: Identity SuperOf (max_values: None, max_size: Some(114), added: 2589, mode: MaxEncodedLen)
	/// Storage: Identity SubsOf (r:1 w:1)
	/// Proof: Identity SubsOf (max_values: None, max_size: Some(3258), added: 5733, mode: MaxEncodedLen)
	/// The range of component `s` is `[0, 99]`.
	fn add_sub(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `507 + s * (36 ±0)`
		//  Estimated: `21305`
		// Minimum execution time: 28_263 nanoseconds.
		Weight::from_ref_time(33_249_692)
			.saturating_add(Weight::from_proof_size(21305))
			// Standard Error: 1_405
			.saturating_add(Weight::from_ref_time(72_932).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: Identity IdentityOf (r:1 w:0)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7538), added: 10013, mode: MaxEncodedLen)
	/// Storage: Identity SuperOf (r:1 w:1)
	/// Proof: Identity SuperOf (max_values: None, max_size: Some(114), added: 2589, mode: MaxEncodedLen)
	/// The range of component `s` is `[1, 100]`.
	fn rename_sub(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `623 + s * (3 ±0)`
		//  Estimated: `14582`
		// Minimum execution time: 13_801 nanoseconds.
		Weight::from_ref_time(16_176_484)
			.saturating_add(Weight::from_proof_size(14582))
			// Standard Error: 872
			.saturating_add(Weight::from_ref_time(16_387).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Identity IdentityOf (r:1 w:0)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7538), added: 10013, mode: MaxEncodedLen)
	/// Storage: Identity SuperOf (r:1 w:1)
	/// Proof: Identity SuperOf (max_values: None, max_size: Some(114), added: 2589, mode: MaxEncodedLen)
	/// Storage: Identity SubsOf (r:1 w:1)
	/// Proof: Identity SubsOf (max_values: None, max_size: Some(3258), added: 5733, mode: MaxEncodedLen)
	/// The range of component `s` is `[1, 100]`.
	fn remove_sub(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `702 + s * (35 ±0)`
		//  Estimated: `21305`
		// Minimum execution time: 32_242 nanoseconds.
		Weight::from_ref_time(35_470_674)
			.saturating_add(Weight::from_proof_size(21305))
			// Standard Error: 1_181
			.saturating_add(Weight::from_ref_time(61_101).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: Identity SuperOf (r:1 w:1)
	/// Proof: Identity SuperOf (max_values: None, max_size: Some(114), added: 2589, mode: MaxEncodedLen)
	/// Storage: Identity SubsOf (r:1 w:1)
	/// Proof: Identity SubsOf (max_values: None, max_size: Some(3258), added: 5733, mode: MaxEncodedLen)
	/// The range of component `s` is `[0, 99]`.
	fn quit_sub(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `628 + s * (37 ±0)`
		//  Estimated: `10302`
		// Minimum execution time: 20_571 nanoseconds.
		Weight::from_ref_time(23_211_865)
			.saturating_add(Weight::from_proof_size(10302))
			// Standard Error: 1_076
			.saturating_add(Weight::from_ref_time(63_829).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
}
