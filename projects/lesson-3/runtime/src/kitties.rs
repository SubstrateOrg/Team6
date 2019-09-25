use support::{decl_module, decl_storage, StorageValue, StorageMap};
use codec::{Encode, Decode};
use runtime_io::blake2_128;
use system::ensure_signed;
use rstd::cmp;


pub trait Trait: system::Trait {
}

#[derive(Encode, Decode, Default)]
pub struct Kitty(pub [u8; 16]);

decl_storage! {
	trait Store for Module<T: Trait> as Kitties {
		/// Stores all the kitties, key is the kitty id / index
		pub Kitties get(kitty): map u32 => Kitty;
		/// Stores the total number of kitties. i.e. the next kitty index
		pub KittiesCount get(kitties_count): u32;
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		/// Create a new kitty
		pub fn create(origin) {
			let sender = ensure_signed(origin)?;
			let count = Self::kitties_count();
			if count == u32::max_value() {
				return Err("Kitties count overflow");
			}
			let payload = (<system::Module<T>>::random_seed(), sender, <system::Module<T>>::extrinsic_index(), <system::Module<T>>::block_number());
			let dna = payload.using_encoded(blake2_128);
			let kitty = Kitty(dna);
			Kitties::insert(count, kitty);
			KittiesCount::put(count + 1);
		}


		fn breed_kitty(origin, kitty_id_1: T::Hash, kitty_id_2: T::Hash) -> Result{
            let sender = ensure_signed(origin)?;

            ensure!(<Kitties<T>>::exists(kitty_id_1), "This cat 1 does not exist");
            ensure!(<Kitties<T>>::exists(kitty_id_2), "This cat 2 does not exist");

            let nonce = <Nonce<T>>::get();
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
                price: <T::Balance as As<u64>>::sa(0),
                gen: cmp::max(kitty_1.gen, kitty_2.gen) + 1,
            };

            Self::mint(sender, random_hash, new_kitty)?;

            <Nonce<T>>::mutate(|n| *n += 1);
	}
}
