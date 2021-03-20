
//! Auxiliary Proof of Work Pallet
//!
//! This pallet allows blockchain users to submit proofs of work on the parent block (or maybe a
//! few recent parents).
//!
//! Why?
//! It's cool
//! To avoid long range attacks
//! To not waste orphaned work (like GHOST)
//! To help forks resolve in a more continuous manner.
//!
//! Imagine if Babe is the primary leader election mechanism.
//! When multiple authors are selected there is a fork. That fork is resolved
//! at the very earliest, at the next slot.  When the next
//! slot arrives, the fork may be suddenly resolved. Or both forks may continue to grow if multiple
//! authors are selected again.
//!
//! To an observer (a user or author) of the network the state of
//! the network during forking conditions can be thought of as a quantum superposition of possible next states.
//! The auxiliary PoW alllows aux miners (CAUTION! aux "miners" are not block authors)
//! to signal their support for one of the forks before the next block is included, and incentivises
//! block authors to choose that fork to collect their transaction rewards.
//!
//! This also helps future authors coordinate better so that two honest miners don't author on
//! different forks by chance.
//!
//!
//! Miners could be incentivized. Think about rewarding "points"
//! Or aux mining could be required for casting votes or something. "If you wanna participate in _this_ election, you need to have stake on _this_ chain"
//! Include an unsigned version of note work?
//! Question: Is this still useful in conjunction with normal proof of work? I don't see why not?
//!      Imagine whale primary miners colluding to author a chain that the plebians didn't like (maybe they're getting sensored, or fees are artificially driven up.)
//!      Grass-roots non-colluding miners could start mining an alternate chain with way less primary hashrate and aux pow miners (the plebs) can signal their support for the second chain that they consider fair
//!
//! This is similar to and possible inspired by a paper I read called nested blocks or something like that.



#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, ensure, traits::Get};
use frame_system::{ensure_none};
use codec::{Encode, Decode};
use sp_runtime::RuntimeDebug;
use sp_core::{U256, H256};
use sha3::{Digest, Sha3_256};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// The seal that is submitted to prove that work was done. The beneficiary is included here to
/// prevent front-running.
#[derive(Encode, Decode, RuntimeDebug, Eq, PartialEq, Hash, Clone)]
pub struct Seal<Hash, AccountId> {
	parent: Hash,
	beneficiary: AccountId,
	nonce: u64,
}
type SealOf<T> = Seal<<T as frame_system::Config>::Hash, <T as frame_system::Config>::AccountId>;

/// Auxiliary PoW's configuration trait.
pub trait Config: frame_system::Config {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
	/// Minimum number of leading zero bits the hash must have to be accepted.
	type MinLeftZeros: Get<u32>;
}

decl_storage! {
	trait Store for Module<T: Config> as AuxPow {
		/// All of the work from genesis until now.
		AccumulatedWork: u64;//TODO is that gonna be big enough? We'll see.
	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId {
		/// Some auxiliary proof of work was included in the chain.
		/// Data are: New Work, Accumulated Work, Beneficiary
		WorkNoted(u64, u64, AccountId),
	}
);

decl_error! {
	pub enum Error for Module<T: Config> {
		/// The Parent block supplied is not this blocks parent.
		IncorrectParent,
		/// The number of zeros the submitter claimed is below the minimum.
		ClaimedWorkInsufficient,
		/// The actual hash does not have as many zeros as claimed.
		WorkHarderNextTime,
	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		/// Submit an auxiliary proof of work.
		#[weight = 0]
		pub fn note_work(origin, seal: SealOf<T>, zeros_of_work: u32) -> dispatch::DispatchResult {
			ensure_none(origin)?;

			// Make sure they're above the minimum difficulty
			ensure!(
				zeros_of_work >= T::MinLeftZeros::get(),
				Error::<T>::ClaimedWorkInsufficient
			);

			// Make sure they are mining on the parent
			ensure!(
				seal.parent == frame_system::Module::<T>::parent_hash(),
				Error::<T>::IncorrectParent
			);

			// Ensure the seal has as many zeros as they claimed
			seal.using_encoded(|bytes| {

				// Do the hashing
				let result = U256::from(&H256::from_slice(Sha3_256::digest(bytes).as_slice())[..]);

				// Hmm this comparison doesn't see to work.
				// In recipes, Wei checks the threshold by multiplying and checking for overflow.
				ensure!(
					result < (U256::max_value() >> zeros_of_work),
					Error::<T>::WorkHarderNextTime
				);
				Ok(result)
			});

			// Update accumulated work.
			let delta_work = u64::pow(2, zeros_of_work);
			let new_work = AccumulatedWork::get() + delta_work;
			AccumulatedWork::put(new_work);

			//TODO reward (or maybe just note via a hook) the beneficiary

			// Emit an event.
			Self::deposit_event(RawEvent::WorkNoted(delta_work, new_work, seal.beneficiary));

			Ok(())
		}
	}
}
