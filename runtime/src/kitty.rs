use frame_support::{decl_storage, decl_module,decl_event,StorageValue,dispatch,ensure,traits::Randomness};
use frame_system::{self as system, ensure_signed};
use frame_support::codec::{Encode, Decode};
use sp_core::H256;
//use sp_std::vec::Vec;
use sp_runtime::traits::Hash;
//use sp_runtime::traits::Zero;
pub trait Trait: pallet_balances::Trait +system::Trait {
    //type Randomness: Randomness<Self::Hash>;
    type RandomnessSource: Randomness<H256>;
    type Event:From<Event<Self>> + Into<<Self as system::Trait>::Event>;
    //type IdentificationTuple: Parameter + Ord;
 }

//type ReportIdOf<T> = <T as frame_system::Trait>::Hash;
#[derive(Encode, Decode, Default, Clone, PartialEq)]
pub struct MyStruct<Hash, Balance>{
    gen: u64,
    id: Hash,
    dna: Hash,
    price: Balance,
}
decl_storage! {
    trait Store for Module<T: Trait> as KittyStorage {
        // Declare storage and getter functions here
        // MyItem get(fn my_item):map hasher(blake2_128_concat)T::AccountId=>MyStruct<T::Balance, T::Hash>;
        // MyU32 get(fn my_u32):u32;
        //MyBool get(fn my_bool):bool; 
        //SomeValue get(fn some_value):map hasher(blake2_128_concat) u32=>u32;
        // MyValue get(fn my_value): map hasher(blake2_128_concat)T::AccountId => u32;
        //猫的数组,[编号,猫ID]
        AllKittiesArray get(fn kitty_by_index):map hasher(blake2_128_concat)u64=> T::Hash;
        //猫的数量,一般是猫的最后一个编号+1
        AllKittiesCount get(fn all_kitties_count):u64;
        //猫的ID对应猫的编号
        AllKittiesIndex :map hasher(blake2_128_concat)T::Hash =>u64;
        //账户对应猫的ID
        //OwnedKitty get(fn own_kitty):map hasher(blake2_128_concat)T::AccountId => T::Hash;
        Check get(fn check): <T as pallet_balances::Trait>::Balance;
        Check2 get(fn check2): <T as pallet_balances::Trait>::Balance;
        //猫的ID对应账户
        KittyOwner get(fn kitty_owner):map hasher(blake2_128_concat) T::Hash=> Option<T::AccountId>;
        //猫的信息
        Kitties get(fn kitty):map hasher(blake2_128_concat) T::Hash=> MyStruct<T::Hash, T::Balance>;
        //账户拥有多只猫的储存结构 [账户,猫的编号]=>猫的ID
        OwnedKittiesArray get(fn kitty_of_owner_by_index): map hasher(blake2_128_concat)(T::AccountId, u64) => T::Hash;
        //账户拥有的猫的数量
        OwnedKittiesCount get(fn owned_kitty_count): map hasher(blake2_128_concat)T::AccountId => u64;
        //猫的ID对应猫的编号
        OwnedKittiesIndex: map hasher(blake2_128_concat)T::Hash => u64;
        Nonce get(fn nonce):u64;

  }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Declare public functions here
       /* #[weight = 0] 
        fn my_funtion(origin, input_bool:bool) -> dispatch::DispatchResult {
            let _sender = ensure_signed(origin)?;

            MyBool::put(input_bool);

            Ok(())
        }
        #[weight = 0] 
        fn set_value(origin, value:u32) -> dispatch::DispatchResult {
            let _sender = ensure_signed(origin)?;
            MyU32::put(value);
            Ok(())
        }
        #[weight = 0]
        fn set_account_value(origin,num:u32)-> dispatch::DispatchResult{
            let sender = ensure_signed(origin)?;
            <MyValue<T>>::insert(sender,num);
            Ok(())
        }*/
        fn deposit_event() = default;
        #[weight = 0]
        fn create_struct(origin) ->dispatch::DispatchResult{
            //let sender = ensure_signed(origin)?;
            //let new_struct = MyStruct::default();
            //Ok(())
            //<OwnedKitty<T>>::insert(sender,new_struct);
            let sender = ensure_signed(origin)?;

            let nonce = Nonce::get();
            let random_hash = ( T::RandomnessSource::random_seed(), &sender, nonce)
                .using_encoded(T::Hashing::hash);
            
            let new_kitty = MyStruct {
                id: random_hash,
                dna: random_hash,
                price: Into::<T::Balance>::into(0),
                gen: 0,
            };
        Check::<T>::put(new_kitty.price);
            Self::mint(sender, random_hash, new_kitty)?;
            Nonce::mutate(|n| *n += 1);

            Ok(())
        }
        #[weight = 0]  
        fn set_price(origin, kitty_id: T::Hash, new_price: T::Balance) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;

            ensure!(<Kitties<T>>::contains_key(kitty_id), "This cat does not exist");

            let owner = Self::kitty_owner(kitty_id).ok_or("No owner for this kitty")?;
            ensure!(owner == sender, "You do not own this cat");

            let mut kitty = Self::kitty(kitty_id);
            kitty.price = new_price;

            <Kitties<T>>::insert(kitty_id, kitty);

            Self::deposit_event(RawEvent::PriceSet(sender, kitty_id, new_price));

            Ok(())
        }

    }
}

decl_event!(
    pub enum Event<T>
    where
        <T as frame_system::Trait>::AccountId,
        <T as frame_system::Trait>::Hash,
        <T as pallet_balances::Trait>::Balance
    {
        Created(AccountId, Hash),
        PriceSet(AccountId, Hash, Balance),
    }
);

impl<T: Trait> Module<T> {
    fn mint(to: T::AccountId, kitty_id: T::Hash, new_kitty: MyStruct<T::Hash, T::Balance>) -> dispatch::DispatchResult {
        ensure!(!<KittyOwner<T>>::contains_key(&kitty_id), "Kitty already exists");

        let owned_kitty_count = Self::owned_kitty_count(&to);

        let new_owned_kitty_count = owned_kitty_count.checked_add(1)
        .ok_or("Overflow adding a new kitty to account balance")?;

        let all_kitties_count = Self::all_kitties_count();

        let new_all_kitties_count = all_kitties_count.checked_add(1)
        .ok_or("overflow adding a new kitty to total supply")?;
        Check2::<T>::put(new_kitty.price);
        <Kitties<T>>::insert(kitty_id, new_kitty);
        <KittyOwner<T>>::insert(kitty_id, &to);
        //<OwnedKitty<T>>::insert(&to, kitty_id);

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