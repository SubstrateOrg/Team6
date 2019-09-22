use support::{decl_module, decl_storage, StorageValue, StorageMap};
use codec::{Encode, Decode};
use runtime_io::blake2_128;
use system::ensure_signed;

/*
V2需求 繁殖⼩小猫
选择两只现有的猫作为⽗父⺟母
⼩小猫必须继承⽗父⺟母的基因
同样的⽗父⺟母⽣生出来的⼩小猫不不能相同
*/

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
			let payload = (<system::Module<T>>::random_seed(), sender, <system::Module<T>>::extrinsic_index(), <system::Module<T>>::block_number());
			let dna = payload.using_encoded(blake2_128);
			Self::do_create(dna)
		}
		/// Breed a new kitty from mother and farther
		pub fn breed(origin, mother: u32, farther, u32) -> Result {
			let sender = ensure_signed(origin)?;
			let payload = (mother + farther + <system::Module<T>>::random_seed());
			let dna = payload.using_encoded(blake2_128);
			Self::do_create(dna)
		}
	}
}

impl<T: Trait> Module<T> {
	fn do_create(dna: [u8; 16]) -> Result {
			let count = Self::kitties_count();
			if count == u32::max_value() {
				return Err("Kitties count overflow");
			}
			let kitty = Kitty(dna);
			Kitties::insert(count, kitty);
			KittiesCount::put(count + 1);
			Ok(())
	}
}