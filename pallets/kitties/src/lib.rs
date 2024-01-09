#![cfg_attr(not(feature = "std"), no_std)]
//的作用是告诉编译器，如果没有启用名为"std"的特性（feature），则不使用标准库（no_std）
#[cfg(test)]
mod mock;
//在测试模式下引入的mod
#[cfg(test)]
mod tests;

mod migrations;
//mod migrations; 声明了一个名为 migrations 的模块。这个声明告诉 Rust 在同一目录下查找一个名为 migrations.rs 或 migrations/mod.rs 的文件，并将其内容作为 migrations 模块的内容。这个声明不会将 migrations 模块引入当前作用域。
//只有在当前作用于可见才可以被使用，这是mod存在的原因
pub use pallet::*;
//在这里把pallet所有的公共项暴露给外面使用。
#[frame_support::pallet]
pub mod pallet {
	use super::*;
	//导入了当前模块前一级的所有暴露的模块
	use frame_support::pallet_prelude::*;
	//模块中包含了一些常用的类型和 trait，例如 VecU8、Weight、DispatchResult、Parameter 等。这些类型和 trait 在编写 Substrate pallet 时经常会用到。
	use frame_system::pallet_prelude::*;
	//模块中包含了一些常用的类型和 trait，例如 BlockNumberFor、CheckWeight、Pallet、RawOrigin 等
	use frame_support::{
		traits::{Currency, ExistenceRequirement, Randomness, StorageVersion},
		PalletId,
	};
	//除了pallet_prelude的所有内容还需要导入以上一些内容
	use sp_runtime::traits::AccountIdConversion;
	//AccountIdConversion是一个trait，可以把AccountId转换成其他类型
	use sp_io::hashing::blake2_128;
	//blake2_128是一个函数，可以把参数转换成一个128位的hash值
	//sp_io是Substrate Primitives Input/Output的缩写
	//hashing里面还包含很多其他的函数，比如blake2_256，keccak_256等
	use crate::migrations;
	//必须要注意的是如果使用use必须在模块中可见，否则无法使用use。
	//因为有super::*;上一级也有声明mod migrations;所以可以直接写成use self::migrations;
	// use super::migrations;
	//也可以写成 use crate::migrations;
	//use crate::migrations; 将 migrations 模块引入当前作用域。这意味着你可以在当前文件中直接使用 migrations 模块中的公共项，而不需要写出完整的路径。
	//由于已经暴露在crate一级目录了，所以可以直接使用crate::migrations;
	pub type KittyId = u32;
	//声明了一个类型，类型名为KittyId，类型为u32
	pub type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
	//声明BalanceOf<T>类型，T as Config表示T类型实现了Config这个trait。
	//Config::Currency表示Config这个trait里面的Currency这个类型。
	//Config::Currency 类型实现了 Currency trait，而 Currency trait 的关联类型 AccountId 被指定为 <<T as frame_system::Config>::AccountId>。
	#[derive(
		Encode, Decode, Clone, Copy, RuntimeDebug, PartialEq, Eq, Default, TypeInfo, MaxEncodedLen,
	)]
	//添加了集中trait实现，encode和decode是用来序列化和反序列化的，clone和copy是用来复制的，runtimeDebug是用来调试的，partialEq和Eq是用来比较的，default是用来默认的，typeInfo是用来获取类型信息的，maxEncodedLen是用来获取最大编码长度的。maxencodedlen是限制编码的最大长度，防止长度过大出现问题。
	// pub struct Kitty(pub [u8; 16]);
	pub struct Kitty {
		pub dna: [u8; 16],
		pub name: [u8; 4],
	}
	//创建一个公共项的结构体，这个结构体储存了，kitty的信息。
	const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);
	//声明了一个常量，sub提供了StorageVersion类型，可以为每个pallet的储存指定一个版本。
	//使用StorageVersion::new(1)来创建一个新的storageversion实例，版本号为1。
	#[pallet::pallet]
	//是一个属性宏，它用于标记 pallet 的主要结构体。这个宏会为结构体添加一些 Substrate 运行时所需的方法和类型。在 Substrate 中，每个 pallet 都需要有一个这样的结构体。
	#[pallet::storage_version(STORAGE_VERSION)]
	//版本号也需要定义在pallet里面
	//通过这个属性宏把版本号定义在pallet里面
	//在 Substrate 中，每个 pallet 都有自己的存储空间，用于存储其状态。当你更新 pallet 的代码时，可能会改变存储的结构。为了处理这种情况，Substrate 提供了存储版本，让你可以为每个 pallet 的存储指定一个版本。STORAGE_VERSION 是一个常量，表示存储的版本号。
	pub struct Pallet<T>(_);
	//Pallet<T> 通常用作 pallet（模块）的主要结构体，它包含了 pallet 的状态和行为。
	//[pallet::pallet] 属性宏会为 Pallet<T> 结构体添加一些 Substrate 运行时所需的方法和类型。
	//这个结构体是空，所有的储存都通过storage来实现了。
	#[pallet::config]
	//它用于定义 pallet 的配置接口。
	//使用这个属性宏来定义pallet的配置接口。
	//可以设置一些可配置的参数。
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		//表示pallet的事件类型可以从Event<Self> 类型转换而来，并且是 frame_system::Config::RuntimeEvent 类型的一种。
		//self类似方法里面的self，这里是指实现了frame_system::Config这个trait的类型。

		//在运行时模块的实现方法如下
		//impl frame_system::Config for MyRuntime {
			// type RuntimeEvent = Event;
			// 其他关联类型的实现...
		// }

		type Randomness: Randomness<Self::Hash, Self::BlockNumber>;
		//Self::Hash 和 Self::BlockNumber 作为参数。
		//这两个Self::Hash和Self::BlockNumber的具体类型是在runtime文件中决定的，如下：

		//impl kitties::Config for MyRuntime {
			// type Hash = Blake2Hash;
			// type BlockNumber = u32;
			// 其他关联类型的实现...
		// }

		type Currency: Currency<Self::AccountId>;
		//表示 Currency 类型使用 Self::AccountId 作为参数
		//也是在运行时模块实现
		//impl frame_system::Config for MyRuntime {
			// type AccountId = AccountId32;
			// 其他关联类型的实现...
		// }
		#[pallet::constant]
		//#[pallet::constant] 宏的作用是标记 pallet 中的常量，让你可以在链规范中设置这些常量的值。
		//链规范是 Substrate 框架中用于描述区块链的配置的一种格式
		//链规范通常以 JSON 文件的形式提供，可以在启动 Substrate 节点时指定。如substrate --chain my_chain_spec.json
		//这些常量的值在链运行时不会改变。一旦 Substrate 节点启动，并根据链规范初始化了状态，这些常量的值就固定了，不会再改变。
		//即使在链的运行过程中发生了各种交易和状态变化，这些常量的值也不会改变。
		type KittyPrice: Get<BalanceOf<Self>>;
		//KittyPrice是一个可以获取帐户余额类型值的常量。
		type PalletId: Get<PalletId>;
		//PalletId是一个可以获取PalletId类型值的常量。
		//这两个常量的值在链规范中设置，并且在链运行时不会改变。
	}

	#[pallet::storage]
	//是 Substrate 框架中的一个属性宏，它用于定义 pallet 的存储项。
	#[pallet::getter(fn next_kitty_id)]
	//是 Substrate 框架中的一个属性宏，它用于定义一个 getter 函数，这个函数可以用来读取存储项的值。
	//可以在 pallet 的其他地方使用这个函数来读取 next_kitty_id 存储项的值
	//例如：let id = <NextKittyId<T>>::get();
	//在这个例子中，<NextKittyId<T>>::get() 调用了 next_kitty_id 函数，读取了 NextKittyId 存储项的值
	pub type NextKittyId<T> = StorageValue<_, KittyId, ValueQuery>;
	//这里的<T>是一个泛型，表示NextKittyId是一个泛型类型。
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

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_runtime_upgrade() -> Weight {
			migrations::v1::migrate::<T>()
		}
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn create(origin: OriginFor<T>, name: [u8; 4]) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let kitty_id = Self::get_next_id()?;
			let dna = Self::random_value(&who);
			let kitty = Kitty { dna, name };
			let price = T::KittyPrice::get();
			// T::Currency::reserve(&who, price)?;
			T::Currency::transfer(
				&who,
				&Self::get_account_id(),
				price,
				ExistenceRequirement::KeepAlive,
			)?;

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
			name: [u8; 4],
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(kitty_id_1 != kitty_id_2, Error::<T>::SameKittyId);

			// let kitty_1 = Self::kitties(kitty_id_1).ok_or(Error::<T>::InvalidKittyId)?;
			// let kitty_2 = Self::kitties(kitty_id_2).ok_or(Error::<T>::InvalidKittyId)?;

			let kitty_id = Self::get_next_id()?;

			// let mut kitty = Kitty::default();
			// let selector = Self::random_value(&who);

			let dna = [0u8; 16];
			let kitty = Kitty { dna, name };

			// for i in 0..16 {
			// 	kitty.0[i] = (selector[i] & kitty_1.0[i]) | (!selector[i] & kitty_2.0[i]);
			// }

			let price = T::KittyPrice::get();
			// T::Currency::reserve(&who, price)?;
			T::Currency::transfer(
				&who,
				&Self::get_account_id(),
				price,
				ExistenceRequirement::KeepAlive,
			)?;

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
			//who是带有copy属性的吗？如果 Option 中的值是 Some，那么它就不是 Copy 的，即使 [u8; 32]
			// 类型是 Copy 的。 所以是copy属性。
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
			//转账，把钱转给pallet，ExistenceRequirement::KeepAlive是什么意思？
			// 检查账户是否有最小余额，防止帐户被销户。
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
