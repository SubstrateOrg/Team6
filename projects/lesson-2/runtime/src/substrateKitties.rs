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
        Kitties get(kitty_id): map T::Hash => Kitty<T::Hash, T::Balance>;
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
            let sender_have = <KittyOwned<T>>::get(&sender);

            // set new id of account legal kitty
            let new_kitty_id = sender_have + 1;

            // calc dna of new kitty
            let id_hash = (&sender, new_kitty_id).using_encoded(<T as system::Trait>::Hashing::hash);

            let dna_hash_array = id_hash.as_ref();
			let dna_hash = BigEndian::read_u128(&dna_hash_array[0..16]);

            // require dna not added
            ensure!(!<KittyOwnership<T>>::exists(id_hash), "Kitty Exists.");

            // new kitty
            let new_kitty = Kitty {
                id: id_hash,
                dna: dna_hash,
                // birthTime: BlockTimeStamp, ?? how to get block timestamp
                birthTime: 0,
                price: 0, 
                generation: 0,
            };

            // store kitty
            Kitties::insert(id_hash, new_kitty);
            <KittyOwnership<T>>::insert(id_hash, &sender);
            <KittyOwned<T>>::insert(&sender, new_kitty_id);

            // all kitties
            let all_kitties_amount =  Self::kitties_amount() + 1;
            <KittiesAmountOfAll>::put(all_kitties_amount);
            <KittiesListOfAll<T>>::insert(all_kitties_amount, id_hash);

            Self::deposit_event(RawEvent::CreateKitty(sender, id_hash));

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
