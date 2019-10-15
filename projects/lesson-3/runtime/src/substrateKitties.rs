use support::{decl_storage, decl_module, decl_event, StorageValue, StorageMap, dispatch::Result, ensure};
use system::ensure_signed;
use sr_primitives::traits::{
    Hash,
};
use codec::{Decode, Encode};
use byteorder::{ByteOrder, BigEndian};

// The balance module's configure trait.
pub trait Trait: balances::Trait {
    /// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// Refers to CryptoKittie's contract code
// support we have a struct: Kitty
// and we have a storage of map: mapping (Hash => Kitty) Kitties
#[derive(Debug, Encode, Decode, Default, Clone, PartialEq)]
// TODO: ??? why add this
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Kitty<Hash, Balance> {
    id: Hash,
    dna: u128,
    birthTime: u64,
    price: Balance, 
    generation: u64,
}

// This module's storage items.
decl_storage! {
	trait Store for Module<T: Trait> as KittyStorage {
		// Kitties storage
        Kitties get(kitty_by_id): map T::Hash => Kitty<T::Hash, T::Balance>;
        // For account id index
        KittyOwnership get(owner): map T::Hash => Option<T::AccountId>;
        // For kitties amount of account
        KittyOwned get(owner_id): map T::AccountId => u64;

        // For all kitties
        KittiesAmountOfAll get(kitties_amount): u64;
        // Kitties list
        KittiesListOfAll get(kitty_index): map u64 => T::Hash;

	}
}

// create module.
decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        fn deposit_event() = default;

        pub fn create_kitty(origin) -> Result {
            // check sender
            let sender = ensure_signed(origin)?;

            // get sender have
            // let sender_have = <KittyOwned<T>>::get(&sender);
            let sender_have = Self::owner_id(&sender);

            // set new id of account legal kitty
            let new_kitty_id = sender_have + 1;

            // calc dna of new kitty
            let id_hash = (&sender, new_kitty_id).using_encoded(<T as system::Trait>::Hashing::hash);

            let dna_hash_array = id_hash.as_ref();
			let dna_hash = BigEndian::read_u128(&dna_hash_array[0..16]);

            

            // new kitty
            let new_kitty = Kitty {
                id: id_hash,
                dna: dna_hash,
                // birthTime: BlockTimeStamp, ?? how to get block timestamp
                birthTime: 0,
                price: 0, 
                generation: 0,
            };

            Self::mint_kitty(sender, new_kitty)?;

            Ok(())

        }

        // pub fn drop_kitty(origin, kittyId: u64, dna: Hash) -> Result {
            // Drop kitty by account and id
            // The dna is used to ensure

            // sender
            // let sender = ensure_signed(origin)?;

            // ensure kitty's dna exists

            // ensure dna is right

            // drop kitty, but id not changed ?????? not good
            // TODO: 

            // Ok(())
        // }

        pub fn breed_kitty(origin, parents_hash_1: T::Hash, parents_hash_2: T::Hash) -> Result {
            // breed kitty from two kitties.

            // ensure sender.
            let sender = ensure_signed(origin)?;

            // get parents from storage
            let parents_id_1 = Self::kitty_by_id(parents_hash_1);
            let parents_id_2 = Self::kitty_by_id(parents_hash_2);

            // init child kitty's dna
            let mut child_kitty_dna = parents_id_1.dna;
            for (index, (parents_id_2_dna_ele, r)) in parents_id_2.dna.as_ref().iter().zip(dna_hash.as_ref().iter()).enumerate() {
                if *parents_id_2_dna_ele %2 == 0 {
                    child_kitty_dna.as_mut()[i] = *dna_2_element;
                }
            }

            // get sender have
            let sender_have = Self::owner_id(&sender);

            // set new id of account legal kitty
            let new_kitty_id = sender_have + 1;

            // calc id hash
            let id_hash = (&sender, new_kitty_id).using_encoded(<T as system::Trait>::Hashing::hash);

            // init child kitty
            let child_kitty = Kitty {
                id: id_hash,
                dna: child_kitty_dna,
                // birthTime: BlockTimeStamp, ?? how to get block timestamp
                birthTime: 0,
                price: 0, 
                generation: 0,
            }

            Self::mint_kitty(sender, child_kitty)?;

            Ok(())
            

        }
    }
}

decl_event!(
	pub enum Event<T> where <T as system::Trait>::AccountId,<T as system::Trait>::Hash {
		// Just a dummy event.
		// Event `Something` is declared with a parameter of the type `u64` and `AccountId`
		// To emit this event, we call the deposit funtion, from our runtime funtions
        CreateKitty(AccountId, Hash),
	}
);

// add mint kitty impl
impl<T: Trait> Module<T> {
    fn mint_kitty(owner: T::AccountId, new_kitty: Kitty<T::Hash, T::Balance>) -> Result {
        // get kitty id
        let id_hash = new_kitty.id;
        // get kitty dna
        let dna = new_kitty.dna;
        // require dna not added
        ensure!(!<KittyOwnership<T>>::exists(id_hash), "Kitty Exists.");
        
        // store kitty
        Kitties::insert(id_hash, child_kitty);
        <KittyOwnership<T>>::insert(id_hash, &owner);
        <KittyOwned<T>>::insert(&owner, new_kitty_id);

        // all kitties
        let all_kitties_amount =  Self::kitties_amount() + 1;
        <KittiesAmountOfAll>::put(all_kitties_amount);
        <KittiesListOfAll<T>>::insert(all_kitties_amount, id_hash);

        Self::deposit_event(RawEvent::CreateKitty(owner, id_hash));

        Ok(())
    }
}
