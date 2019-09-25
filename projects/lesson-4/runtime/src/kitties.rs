use support::{decl_module, decl_storage, ensure, StorageValue, StorageMap, dispatch::Result, Parameter};
use sr_primitives::traits::{SimpleArithmetic, Bounded};
use codec::{Encode, Decode};
use runtime_io::blake2_128;
use system::ensure_signed;
use rstd::result;

pub trait Trait: system::Trait {
	type KittyIndex: Parameter + SimpleArithmetic + Bounded + Default + Copy;
}

#[derive(Encode, Decode)]
pub struct Kitty(pub [u8; 16]);

decl_storage! {
	trait Store for Module<T: Trait> as Kitties {
		/// Stores all the kitties, key is the kitty id / index
		pub Kitties get(kitty): map T::KittyIndex => Option<Kitty>;
		/// Stores the total number of kitties. i.e. the next kitty index
		pub KittiesCount get(kitties_count): T::KittyIndex;

		/// Get kitty ID by account ID and user kitty index
		pub OwnedKitties get(owned_kitties): map (T::AccountId, T::KittyIndex) => T::KittyIndex;
		/// Get number of kitties by account ID
		pub OwnedKittiesCount get(owned_kitties_count): map T::AccountId => T::KittyIndex;

		/// Get Owner by kitty index
		pub KittiesOwned get(kitties_owned): map T::KittyIndex => (T::AccountId, T::KittyIndex);
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		/// Create a new kitty
		pub fn create(origin) {
			let sender = ensure_signed(origin)?;

			Self::do_create(sender)?;
		}

		/// Breed kitties
		pub fn breed(origin, kitty_id_1: T::KittyIndex, kitty_id_2: T::KittyIndex) {
			let sender = ensure_signed(origin)?;

			Self::do_breed(sender, kitty_id_1, kitty_id_2)?;
		}

		/// Transfer kitty
		pub fn transfer(origin, to: T::AccountId, kitty_id: T::KittyIndex) {
			let sender = ensure_signed(origin)?;

			Self::do_transfer(&sender, &to, kitty_id)?;
		}
	}
}

fn combine_dna(dna1: u8, dna2: u8, selector: u8) -> u8 {
	// 作业：实现combine_dna
	// 伪代码：
	// selector.map_bits(|bit, index| if (bit == 1) { dna1 & (1 << index) } else { dna2 & (1 << index) })
	// 注意 map_bits这个方法不存在。只要能达到同样效果，不局限算法
	// 测试数据：dna1 = 0b11110000, dna2 = 0b11001100, selector = 0b10101010, 返回值 0b11100100
	let mut dna: u8 = 0;
	for index in 0..8 {
		if ((selector >> index) & 1) == 1 {
			dna |= dna1 & (1 << index);
		} else {
			dna |= dna2 & (1 << index);
		}
	}
	return dna;
}

impl<T: Trait> Module<T> {
	fn random_value(sender: &T::AccountId) -> [u8; 16] {
		let payload = (<system::Module<T>>::random_seed(), sender, <system::Module<T>>::extrinsic_index(), <system::Module<T>>::block_number());
		payload.using_encoded(blake2_128)
	}

	fn next_kitty_id() -> result::Result<T::KittyIndex, &'static str> {
		let kitty_id = Self::kitties_count();
		if kitty_id == T::KittyIndex::max_value() {
			return Err("Kitties count overflow");
		}
		Ok(kitty_id)
	}

	fn insert_kitty(owner: T::AccountId, kitty_id: T::KittyIndex, kitty: Kitty) {
		// Create and store kitty
		<Kitties<T>>::insert(kitty_id, kitty);
		<KittiesCount<T>>::put(kitty_id + 1.into());

		// Store the ownership information
		let user_kitties_id = Self::owned_kitties_count(owner.clone());
		<OwnedKitties<T>>::insert((owner.clone(), user_kitties_id), kitty_id);
		<OwnedKittiesCount<T>>::insert(owner.clone(), user_kitties_id + 1.into());

		// Store Kitty owned
		<KittiesOwned<T>>::insert(kitty_id, (owner, user_kitties_id));
	}

	fn do_create(sender: T::AccountId) -> Result {
		let kitty_id = Self::next_kitty_id()?;

		// Generate a random 128bit value
		let dna = Self::random_value(&sender);

		// Create and store kitty
		Self::insert_kitty(sender, kitty_id, Kitty(dna));

		Ok(())
	}

	fn do_breed(sender: T::AccountId, kitty_id_1: T::KittyIndex, kitty_id_2: T::KittyIndex) -> Result {
		let kitty1 = Self::kitty(kitty_id_1);
		let kitty2 = Self::kitty(kitty_id_2);

		ensure!(kitty1.is_some(), "Invalid kitty_id_1");
		ensure!(kitty2.is_some(), "Invalid kitty_id_2");
		ensure!(kitty_id_1 != kitty_id_2, "Needs different parent");

		let kitty_id = Self::next_kitty_id()?;

		let kitty1_dna = kitty1.unwrap().0;
		let kitty2_dna = kitty2.unwrap().0;

		// Generate a random 128bit value
		let selector = Self::random_value(&sender);
		let mut new_dna = [0u8; 16];

		// Combine parents and selector to create new kitty
		for i in 0..kitty1_dna.len() {
			new_dna[i] = combine_dna(kitty1_dna[i], kitty2_dna[i], selector[i]);
		}

		Self::insert_kitty(sender, kitty_id, Kitty(new_dna));

		Ok(())
	}

	fn do_transfer(sender: &T::AccountId, to: &T::AccountId, kitty_id: T::KittyIndex) -> Result {
		let kitty = Self::kitty(kitty_id);

		ensure!(kitty.is_some(), "Invalid kitty_id");

		let kitties_count_sender = Self::owned_kitties_count(sender);
		let kitties_count_to = Self::owned_kitties_count(to);

		if sender == to {
			return Err("Can't transfer to self");
		}

		// method 2: O(n)
		if false {
			let mut index: T::KittyIndex = 0.into();
			while index < kitties_count_sender {
				if <OwnedKitties<T>>::get((sender.clone(), index)) == kitty_id {
					if index != kitties_count_sender - 1.into() {
						<OwnedKitties<T>>::swap((sender.clone(), index), (sender.clone(), kitties_count_sender - 1.into()));
						break;
					}
				} else {
					if index == kitties_count_sender - 1.into() {
						return Err("No owner for this kitty");
					}
				}

				index += 1.into();
			}
		}

		// method 2: O(1)
		if true {
			let kitties_owned = Self::kitties_owned(kitty_id);
			if sender.clone() != kitties_owned.0 {
				return Err("No owner for this kitty");
			}

			if kitties_owned.1 != kitties_count_sender - 1.into() {
				<OwnedKitties<T>>::swap((sender.clone(), kitties_owned.1), (sender.clone(), kitties_count_sender - 1.into()));
			}

			<KittiesOwned<T>>::insert(kitty_id, (to.clone(), kitties_count_to));
		}

		<OwnedKitties<T>>::remove((sender.clone(), kitties_count_sender - 1.into()));
		<OwnedKittiesCount<T>>::insert(sender.clone(), kitties_count_sender - 1.into());

		<OwnedKitties<T>>::insert((to.clone(), kitties_count_to), kitty_id);
		<OwnedKittiesCount<T>>::insert(to, kitties_count_to + 1.into());

		Ok(())
	}
}
