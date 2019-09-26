use support::{decl_module, decl_storage, StorageValue, StorageMap, dispatch::Result};
use codec::{Encode, Decode};
use runtime_io::blake2_128;
use system::ensure_signed;
use rstd::prelude::Vec;

/*
V1需求
	链上存储加密猫数据
	每个用户可以拥有零到多只猫
	遍历所有加密猫
	每只猫只有⼀一个主⼈
	遍历⽤户拥有的所有猫
	每只猫都有⾃己的dna，为128bit的数据
	设计如何生成dna (伪代码算法)

V2需求 
	繁殖小⼩猫,选择两只现有的猫作为⽗母
	⼩小猫必须继承⽗母的基因
	同样的⽗母⽣生出来的⼩小猫不能相同

V3需求
	重构 create，使⽤用新的帮助函数 
	完成 combine_dna
	transfer kitty 转移猫 
	要求复杂度必须优于 O(n)
	创建新的polkadot apps项⽬
	设计如何在substrate中实现树形结构
*/

pub trait Trait: system::Trait {
}

#[derive(Encode, Decode, Clone, Default)]
pub struct Kitty<T> where T: Trait {
	dna: [u8; 16],
	owner: T::AccountId,
	price: u32,
}

decl_storage! {
	trait Store for Module<T: Trait> as kitties {
		/// Stores all the kitties, key is the kitty id / index
		pub Kitties get(kitty): map u32 => Kitty<T>;
		/// Stores the total number of kitties. i.e. the next kitty index
		pub KittiesCount get(kitties_count): u32;
		pub OwnerKitties get(owner_kitties): map T::AccountId => Option<Vec<Kitty>>;
	}
}

impl <T> OwnerKitties<T> where T:Trait {
	fn add_kitty(owner: T::AccountId, kitty: Kitty<T>) {
		let mut kitties;
		if let Some(one) = Self::get(owner.clone()) {
			kitties = one;
		} else {
			kitties = Vec::new();
		}
		kitties.push(kitty);
		Self::insert(owner, kitties);
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		/// Create a new kitty
		pub fn create(origin) -> Result {
			let sender = ensure_signed(origin)?;
			let payload = (<system::Module<T>>::random_seed(), sender, <system::Module<T>>::extrinsic_index(), <system::Module<T>>::block_number());
			let dna = payload.using_encoded(blake2_128);
			Self::do_create(sender, dna)?;
			Ok(())
		}
		/// Breed a new kitty from mother and father
		pub fn breed(origin, mother: u32, father: u32) -> Result {
			let sender = ensure_signed(origin)?;
			match Kitties::get(mother.Clone()) {
				Some(one) => one,
				None => Err("mother does not exist")
			}
			match Kitties::get(father.Clone()) {
				Some(one) => one,
				None => Err("father does not exist")
			}
			let payload = (mother, father, <system::Module<T>>::random_seed());
			let dna = payload.using_encoded(blake2_128);
			Self::do_create(sender, dna)?;
			Ok(())
		}

	}
}

impl<T: Trait> Module<T> {
	fn do_create(sender: T::AccountId, dna: [u8; 16]) -> Result {
			let count = Self::kitties_count();
			if count == u32::max_value() {
				return Err("Kitties count overflow");
			}
			let kitty = Kitty{
				dna: dna,
				owner: sender.clone(),
				price: 1.into(),
			};
			Kitties::insert(count, kitty);
			KittiesCount::put(count + 1);
			<OwnerKitties<T>>::add_kitty(sender.clone(), kitty);
			Ok(())
	}
}