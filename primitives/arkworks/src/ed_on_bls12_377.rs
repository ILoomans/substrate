// This file is part of Substrate.

// Copyright (C) 2017-2022 Parity Technologies (UK) Ltd.
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

//! Hashing Functions.

#![warn(missing_docs)]

use ark_ec::{
	models::CurveConfig, twisted_edwards, twisted_edwards::TECurveConfig, VariableBaseMSM,
};
use ark_ed_on_bls12_377::{EdwardsConfig, EdwardsProjective};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, Compress, Validate};
use ark_std::io::Cursor;
use sp_std::{vec, vec::Vec};

/// Compute a scalar multiplication on G2 through arkworks
pub fn mul_projective(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8> {
	let cursor = Cursor::new(base);
	let base = twisted_edwards::Projective::<EdwardsConfig>::deserialize_with_mode(
		cursor,
		Compress::Yes,
		Validate::No,
	)
	.unwrap();
	let cursor = Cursor::new(scalar);
	let scalar = Vec::<u64>::deserialize_with_mode(cursor, Compress::Yes, Validate::No).unwrap();
	let res = <EdwardsConfig as TECurveConfig>::mul_projective(&base, &scalar);
	let mut serialized = vec![0; res.serialized_size(Compress::Yes)];
	let mut cursor = Cursor::new(&mut serialized[..]);
	res.serialize_with_mode(&mut cursor, Compress::Yes).unwrap();
	serialized
}

/// Compute a scalar multiplication through arkworks
pub fn mul_affine(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8> {
	let cursor = Cursor::new(base);
	let base = twisted_edwards::Affine::<EdwardsConfig>::deserialize_with_mode(
		cursor,
		Compress::Yes,
		Validate::No,
	)
	.unwrap();
	let cursor = Cursor::new(scalar);
	let scalar = Vec::<u64>::deserialize_with_mode(cursor, Compress::Yes, Validate::No).unwrap();
	let res = <EdwardsConfig as TECurveConfig>::mul_affine(&base, &scalar);
	let mut serialized = vec![0; res.serialized_size(Compress::Yes)];
	let mut cursor = Cursor::new(&mut serialized[..]);
	res.serialize_with_mode(&mut cursor, Compress::Yes).unwrap();
	serialized
}

/// Compute a multi scalar multiplication on G! through arkworks
pub fn msm(bases: Vec<Vec<u8>>, scalars: Vec<Vec<u8>>) -> Vec<u8> {
	let bases: Vec<_> = bases
		.iter()
		.map(|a| {
			let cursor = Cursor::new(a);
			twisted_edwards::Affine::<EdwardsConfig>::deserialize_with_mode(
				cursor,
				Compress::Yes,
				Validate::No,
			)
			.unwrap()
		})
		.collect();
	let scalars: Vec<_> = scalars
		.iter()
		.map(|a| {
			let cursor = Cursor::new(a);
			<EdwardsConfig as CurveConfig>::ScalarField::deserialize_with_mode(
				cursor,
				Compress::Yes,
				Validate::No,
			)
			.unwrap()
		})
		.collect();

	let result = <EdwardsProjective as VariableBaseMSM>::msm(&bases, &scalars).unwrap();
	let mut serialized = vec![0; result.serialized_size(Compress::Yes)];
	let mut cursor = Cursor::new(&mut serialized[..]);
	result.serialize_with_mode(&mut cursor, Compress::Yes).unwrap();
	serialized
}