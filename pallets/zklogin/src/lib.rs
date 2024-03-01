// Copyright 2024 Moonbeam foundation
// This file is part of Moonbeam.

// Moonbeam is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Moonbeam is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Moonbeam.  If not, see <http://www.gnu.org/licenses/>.

//! # Lazy Migration Pallet

#![allow(non_camel_case_types)]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod weights;
pub use weights::WeightInfo;
mod types;

use frame_support::pallet;

pub use pallet::*;

#[pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_std::vec::Vec;
	use types::ImageId;

	/// Pallet for web2 based zk login
	#[pallet::pallet]
	pub struct Pallet<T>(PhantomData<T>);

	/// Configuration trait of this pallet.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type WeightInfo: WeightInfo;
	}

	#[pallet::storage]
	/// The current image id which should produce zklogin proofs
	type ImageIdValue<T: Config> = StorageValue<_, ImageId, ValueQuery>;

	/// Events of this pallet.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Receipt verified
		ReceiptVerified,
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Invalid image id
		InvalidImageId,
		/// Invalid receipt
		InvalidReceipt,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Set the image id which should produce zklogin proofs
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn set_image_id(origin: OriginFor<T>, image_id: ImageId) -> DispatchResult {
			// TODO should be only executed by a governance origin
			ensure_root(origin)?;

			ImageIdValue::<T>::put(image_id);

			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(0)]
		pub fn verify_proof(origin: OriginFor<T>, receipt: Vec<u8>) -> DispatchResult {
			ensure_signed(origin)?;

			let receipt: risc0_zkvm::Receipt =
				serde_json::from_slice(&receipt).map_err(|_| Error::<T>::InvalidReceipt)?;

			let image_id: [u32; 8] = ImageIdValue::<T>::get()
				.as_slice()
				.try_into()
				.map_err(|_| Error::<T>::InvalidImageId)?;

			receipt
				.verify(image_id)
				.expect("receipt verification failed");

			Self::deposit_event(Event::ReceiptVerified);

			Ok(())
		}
	}
}
