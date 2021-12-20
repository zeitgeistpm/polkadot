// Copyright 2017-2021 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.
//! Autogenerated weights for `pallet_bounties`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-12-01, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("kusama-dev"), DB CACHE: 128

// Executed Command:
// target/release/polkadot
// benchmark
// --chain=kusama-dev
// --steps=50
// --repeat=20
// --pallet=pallet_bounties
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --header=./file_header.txt
// --output=./runtime/kusama/src/weights/pallet_bounties.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_bounties`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_bounties::WeightInfo for WeightInfo<T> {
	// Storage: Bounties BountyCount (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Bounties BountyDescriptions (r:0 w:1)
	// Storage: Bounties Bounties (r:0 w:1)
	fn propose_bounty(d: u32, ) -> Weight {
		(38_664_000 as Weight)
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Bounties Bounties (r:1 w:1)
	// Storage: Bounties BountyApprovals (r:1 w:1)
	fn approve_bounty() -> Weight {
		(10_293_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Bounties Bounties (r:1 w:1)
	fn propose_curator() -> Weight {
		(8_255_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Bounties Bounties (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn unassign_curator() -> Weight {
		(58_181_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Bounties Bounties (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn accept_curator() -> Weight {
		(32_865_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Bounties Bounties (r:1 w:1)
	fn award_bounty() -> Weight {
		(23_160_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Bounties Bounties (r:1 w:1)
	// Storage: System Account (r:3 w:3)
	// Storage: Bounties BountyDescriptions (r:0 w:1)
	fn claim_bounty() -> Weight {
		(104_755_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	// Storage: Bounties Bounties (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Bounties BountyDescriptions (r:0 w:1)
	fn close_bounty_proposed() -> Weight {
		(57_771_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Bounties Bounties (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Bounties BountyDescriptions (r:0 w:1)
	fn close_bounty_active() -> Weight {
		(71_817_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Bounties Bounties (r:1 w:1)
	fn extend_bounty_expiry() -> Weight {
		(22_153_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Bounties BountyApprovals (r:1 w:1)
	// Storage: Bounties Bounties (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	fn spend_funds(b: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 29_000
			.saturating_add((55_584_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(b as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(b as Weight)))
	}
}
