use support::{ensure, dispatch::Result, StorageValue, StorageMap, decl_module, decl_storage, decl_event};
use sr_primitives::traits::Hash;
use system::ensure_signed;
use codec::{Encode, Decode};
use runtime_io::blake2_128;
use rstd::cmp;

#[derive(Debug, Encode, Decode, Default, Copy, Clone, PartialEq)]
pub struct Kitty<Hash, Balance>  {
	id: Hash,
	dna: [u8; 16],
	price: Balance,
	gen: u64,
}

pub trait Trait: balances::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
	trait Store for Module<T: Trait> as KittyStorage {
		Kitties get(kitty): map T::Hash => Kitty<T::Hash, T::Balance>;
		KittyOwner get(owner_of): map T::Hash => Option<T::AccountId>;

		AllKittiesArray get(kitty_by_index): map u64 => T::Hash;
		AllKittiesCount get(all_kitties_count): u64;
		AllKittiesIndex: map T::Hash => u64;

		OwnedKittiesArray get(kitty_of_owner_by_index): map (T::AccountId, u64) => T::Hash;
		OwnedKittiesCount get(owned_kitty_count): map T::AccountId => u64;
		OwnedKittiesIndex: map T::Hash => u64;

		Nonce: u64;
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;
		fn create_kitty(origin) -> Result {
			let sender = ensure_signed(origin)?;

			let owned_kitty_count = Self::owned_kitty_count(&sender);

			let new_owned_kitty_count = owned_kitty_count.checked_add(1)
				.ok_or("Overflow adding a new kitty to account balance")?;

			let all_kitties_count = Self::all_kitties_count();

			let new_all_kitties_count = all_kitties_count.checked_add(1)
				.ok_or("Overflow adding a new kitty to total supply")?;

			let nonce = Nonce::get();
			let random_hash = (<system::Module<T>>::random_seed(), &sender, nonce).using_encoded(<T as system::Trait>::Hashing::hash);

			let payload = (<system::Module<T>>::random_seed(), &sender, <system::Module<T>>::extrinsic_index(), <system::Module<T>>::block_number());
			let dna = payload.using_encoded(blake2_128);

			ensure!(!<KittyOwner<T>>::exists(random_hash), "Kitty already exists");

			let new_kitty = Kitty {
				id: random_hash,
				dna: dna,
				price: 0.into(),
				gen: 0,
			};

			<Kitties<T>>::insert(random_hash, new_kitty);
			<KittyOwner<T>>::insert(random_hash, &sender);

			<AllKittiesArray<T>>::insert(all_kitties_count, random_hash);
			AllKittiesCount::put(new_all_kitties_count);
			<AllKittiesIndex<T>>::insert(random_hash, all_kitties_count);

			<OwnedKittiesArray<T>>::insert((sender.clone(), owned_kitty_count), random_hash);
			<OwnedKittiesCount<T>>::insert(&sender, new_owned_kitty_count);
			<OwnedKittiesIndex<T>>::insert(random_hash, owned_kitty_count);

			Nonce::mutate(|n| *n += 1);

			Self::deposit_event(RawEvent::Created(sender, random_hash));

			Ok(())
		}

		fn breed_kitty(origin, kitty_id_1: T::Hash, kitty_id_2: T::Hash) -> Result{
			let sender = ensure_signed(origin)?;

			ensure!(<Kitties<T>>::exists(kitty_id_1), "This cat 1 does not exist");
			ensure!(<Kitties<T>>::exists(kitty_id_2), "This cat 2 does not exist");

			let nonce = Nonce::get();
			let random_hash = (<system::Module<T>>::random_seed(), &sender, nonce)
				.using_encoded(<T as system::Trait>::Hashing::hash);

			let kitty_1 = Self::kitty(kitty_id_1);
			let kitty_2 = Self::kitty(kitty_id_2);

			let mut final_dna = kitty_1.dna;
			for (i, (dna_2_element, r)) in kitty_2.dna.as_ref().iter().zip(random_hash.as_ref().iter()).enumerate() {
				if r % 2 == 0 {
					final_dna.as_mut()[i] = *dna_2_element;
				}
			}

			let new_kitty = Kitty {
				id: random_hash,
				dna: final_dna,
				price: 0.into(),
				gen: cmp::max(kitty_1.gen, kitty_2.gen) + 1,
			};

			Self::mint(sender, random_hash, new_kitty)?;

			Nonce::mutate(|n| *n += 1);

			Ok(())
		}
	}
}

decl_event!(
	pub enum Event<T> where
		<T as system::Trait>::AccountId,
		<T as system::Trait>::Hash
	{
		Created(AccountId, Hash),
	}
);

impl<T: Trait> Module<T> {
	fn mint(to: T::AccountId, kitty_id: T::Hash, new_kitty: Kitty<T::Hash, T::Balance>) -> Result {
		ensure!(!<KittyOwner<T>>::exists(kitty_id), "Kitty already exists");

		let owned_kitty_count = Self::owned_kitty_count(&to);

		let new_owned_kitty_count = owned_kitty_count.checked_add(1)
			.ok_or("Overflow adding a new kitty to account balance")?;

		let all_kitties_count = Self::all_kitties_count();

		let new_all_kitties_count = all_kitties_count.checked_add(1)
			.ok_or("Overflow adding a new kitty to total supply")?;

		<Kitties<T>>::insert(kitty_id, new_kitty);
		<KittyOwner<T>>::insert(kitty_id, &to);

		<AllKittiesArray<T>>::insert(all_kitties_count, kitty_id);
		AllKittiesCount::put(new_all_kitties_count);
		<AllKittiesIndex<T>>::insert(kitty_id, all_kitties_count);

		<OwnedKittiesArray<T>>::insert((to.clone(), owned_kitty_count), kitty_id);
		<OwnedKittiesCount<T>>::insert(&to, new_owned_kitty_count);
		<OwnedKittiesIndex<T>>::insert(kitty_id, owned_kitty_count);

		Self::deposit_event(RawEvent::Created(to, kitty_id));

		Ok(())
	}
}
