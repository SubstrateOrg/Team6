use support::{decl_module, decl_storage, StorageValue, StorageMap, dispatch::Result, ensure,};
use codec::{Encode, Decode};
use runtime_io::blake2_128;
use system::ensure_signed;

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
		pub fn create(origin) -> Result {
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
			Ok(())
		}
		// Breed a new kitty
		pub fn breed(origin, father: u32, mother: u32) -> Result{
			let sender = ensure_signed(origin)?;
			let count = Self::kitties_count();
			
			let new_count = count.checked_add(1).ok_or("Overflow adding a new kitty to total supply")?;

			ensure!(Kitties::exists(father), "This father does not exist");
            ensure!(Kitties::exists(mother), "This mother does not exist");
			ensure!(father != mother, "error of one parent");

            let random_hash = (<system::Module<T>>::random_seed(), &sender, <system::Module<T>>::extrinsic_index(), <system::Module<T>>::block_number())
                .using_encoded(blake2_128);

            let kitty_father = Self::kitty(father);
            let kitty_mother = Self::kitty(mother);

            let mut final_dna = kitty_father.0;
            for (i, (dna_2_element, r)) in kitty_mother.0.iter().zip(random_hash.iter()).enumerate() {
                if r % 2 == 0 {
                    final_dna[i] = *dna_2_element;
                }
            }

			let kitty = Kitty(final_dna);

			Kitties::insert(count, kitty);
			KittiesCount::put(new_count);
			Ok(())
		}
	}
}