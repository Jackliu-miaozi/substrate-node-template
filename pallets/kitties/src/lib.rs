#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	use frame_support::traits::{Currency, Randomness, ExistenceRequirement};
	use frame_support::PalletId;

	use sp_runtime::traits::AccountIdConversion;

	use sp_io::hashing::blake2_128;

	pub type KittyId = u32;
	pub type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[derive(
		Encode, Decode, Clone, Copy, RuntimeDebug, PartialEq, Eq, Default, TypeInfo, MaxEncodedLen,
	)]
	pub struct Kitty(pub [u8; 16]);

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type Randomness: Randomness<Self::Hash, Self::BlockNumber>;
		type Currency: Currency<Self::AccountId>;
		#[pallet::constant]
		type KittyPrice: Get<BalanceOf<Self>>;
		type PalletId: Get<PalletId>;
	}

	#[pallet::storage]
	#[pallet::getter(fn next_kitty_id)]
	pub type NextKittyId<T> = StorageValue<_, KittyId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn kitties)]
	pub type Kitties<T> = StorageMap<_, Blake2_128Concat, KittyId, Kitty>;

	#[pallet::storage]
	#[pallet::getter(fn kitty_owner)]
	pub type KittyOwner<T: Config> = StorageMap<_, Blake2_128Concat, KittyId, T::AccountId>;

	#[pallet::storage]
	#[pallet::getter(fn kitty_parents)]
	pub type KittyParents<T: Config> = StorageMap<_, Blake2_128Concat, KittyId, (KittyId, KittyId)>;

	#[pallet::storage]
	#[pallet::getter(fn kitty_on_sale)]
	pub type KittyOnSale<T: Config> = StorageMap<_, Blake2_128Concat, KittyId, ()>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		KittyCreated { who: T::AccountId, kitty_id: KittyId, kitty: Kitty },
		KittyBred { who: T::AccountId, kitty_id: KittyId, kitty: Kitty },
		KittyTransferred { who: T::AccountId, recipient: T::AccountId, kitty_id: KittyId },
		KittyOnSale { who: T::AccountId, kitty_id: KittyId },
		KittyBought { who: T::AccountId, current_owner: T::AccountId, kitty_id: KittyId },
	}

	#[pallet::error]
	pub enum Error<T> {
		InvalidKittyId,
		SameKittyId,
		NotOwner,
		AlreadyOnSale,
		NoOwner,
		AlreadyOwned,
		NotOnSale,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn create(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let kitty_id = Self::get_next_id()?;
			let kitty = Kitty(Self::random_value(&who));

			let price = T::KittyPrice::get();
			// T::Currency::reserve(&who, price)?;
			T::Currency::transfer(&who, &Self::get_account_id(), price, ExistenceRequirement::KeepAlive)?;
 
			Kitties::<T>::insert(kitty_id, &kitty);
			KittyOwner::<T>::insert(kitty_id, &who);

			Self::deposit_event(Event::KittyCreated { who, kitty_id, kitty });
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn breed(
			origin: OriginFor<T>,
			kitty_id_1: KittyId,
			kitty_id_2: KittyId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(kitty_id_1 != kitty_id_2, Error::<T>::SameKittyId);

			let kitty_1 = Self::kitties(kitty_id_1).ok_or(Error::<T>::InvalidKittyId)?;
			let kitty_2 = Self::kitties(kitty_id_2).ok_or(Error::<T>::InvalidKittyId)?;

			let kitty_id = Self::get_next_id()?;

			let mut kitty = Kitty::default();
			let selector = Self::random_value(&who);

			for i in 0..16 {
				kitty.0[i] = (selector[i] & kitty_1.0[i]) | (!selector[i] & kitty_2.0[i]);
			}

			let price = T::KittyPrice::get();
			// T::Currency::reserve(&who, price)?;
			T::Currency::transfer(&who, &Self::get_account_id(), price, ExistenceRequirement::KeepAlive)?;


			Kitties::<T>::insert(kitty_id, &kitty);
			KittyOwner::<T>::insert(kitty_id, &who);
			KittyParents::<T>::insert(kitty_id, (kitty_id_1, kitty_id_2));

			Self::deposit_event(Event::KittyBred { who, kitty_id, kitty });
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn transfer(
			origin: OriginFor<T>,
			recipient: T::AccountId,
			kitty_id: KittyId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(KittyOwner::<T>::contains_key(kitty_id), Error::<T>::InvalidKittyId);
			ensure!(Self::kitty_owner(kitty_id) == Some(who.clone()), Error::<T>::NotOwner);

			KittyOwner::<T>::insert(kitty_id, &recipient);

			Self::deposit_event(Event::KittyTransferred { who, recipient, kitty_id });
			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn sale(origin: OriginFor<T>, kitty_id: KittyId) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Self::kitties(kitty_id).ok_or(Error::<T>::InvalidKittyId)?;
			//查看是否储存中有这个kitty_id，如果没有，返回错误。
			ensure!(Self::kitty_owner(kitty_id) == Some(who.clone()), Error::<T>::NotOwner);
			//Self指的就是pallet，kitty_owner是getter函数，所以可以直接调用，并传入参数。
			ensure!(Self::kitty_on_sale(kitty_id).is_some(), Error::<T>::AlreadyOnSale);

			<KittyOnSale<T>>::insert(kitty_id, ());
			Self::deposit_event(Event::KittyOnSale { who, kitty_id });

			Ok(())
		}

		#[pallet::call_index(4)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn buy(origin: OriginFor<T>, kitty_id: KittyId) -> DispatchResult {
			let who = ensure_signed(origin)?;
			//who是带有copy属性的吗？如果 Option 中的值是 Some，那么它就不是 Copy 的，即使 [u8; 32] 类型是 Copy 的。
			// 所以是copy属性。
			Self::kitties(kitty_id).ok_or(Error::<T>::InvalidKittyId)?;
			//查看是否储存中有这个kitty_id，如果没有，返回错误。
			let current_owner = Self::kitty_owner(kitty_id).ok_or(Error::<T>::NoOwner)?;
			//.ok_or()返回的是Some()里面的值，如果是None，就返回后面错误。NoOwner的意思是没有拥有者。
			ensure!(current_owner == who, Error::<T>::AlreadyOwned);
			//判断是否是自己的猫，如果是，返回错误，已经拥有了猫。由于who带有copy属性，
			// 所以不再写clone()。
			ensure!(Self::kitty_on_sale(kitty_id).is_some(), Error::<T>::NotOnSale);
			//判断是否在出售中，如果不是，返回错误，不在出售中。
			let price = T::KittyPrice::get();
			//获取价格
			// T::Currency::reserve(&who, price)?;
			//reserve的作用是什么？质押token，是system_support里面的Currency这个trait的函数。
			KittyOwner::<T>::insert(kitty_id, &who);
			//把猫的拥有者改为买家。
			<KittyOnSale<T>>::remove(kitty_id);
			//移除出售状态
			// T::Currency::unreserve(&current_owner, price);
			//解除质押，因为是买家买了，所以是解除卖家的质押。
			T::Currency::transfer(&who, &current_owner, price, ExistenceRequirement::KeepAlive)?;
			//转账，把钱转给pallet，ExistenceRequirement::KeepAlive是什么意思？检查账户是否有最小余额，防止帐户被销户。
			Self::deposit_event(Event::KittyBought { who, current_owner, kitty_id });
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		fn get_next_id() -> Result<KittyId, DispatchError> {
			NextKittyId::<T>::try_mutate(|next_id| -> Result<KittyId, DispatchError> {
				let current_id = *next_id;
				*next_id = next_id.checked_add(1).ok_or(Error::<T>::InvalidKittyId)?;
				Ok(current_id)
			})
		}

		fn random_value(sender: &T::AccountId) -> [u8; 16] {
			let payload = (
				T::Randomness::random_seed(),
				&sender,
				<frame_system::Pallet<T>>::extrinsic_index(),
			);
			payload.using_encoded(blake2_128)
		}

		fn get_account_id() -> T::AccountId {
			T::PalletId::get().into_account_truncating()
			//可能有长度上的不匹配
			//所以用另一个种方法来匹配长度
			//改.into_account()为
		}
	}
}
 