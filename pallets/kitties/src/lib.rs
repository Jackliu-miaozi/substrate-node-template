#![cfg_attr(not(feature = "std"), no_std)]
//的作用是告诉编译器，如果没有启用名为"std"的特性（feature），则不使用标准库（no_std）
#[cfg(test)]
mod mock;
//在测试模式下引入的mod
#[cfg(test)]
mod tests;

mod migrations;
//mod migrations; 声明了一个名为 migrations 的模块。这个声明告诉 Rust 在同一目录下查找一个名为
// migrations.rs 或 migrations/mod.rs 的文件，并将其内容作为 migrations 模块的内容。这个声明不会将
// migrations 模块引入当前作用域。 只有在当前作用于可见才可以被使用，这是mod存在的原因
pub use pallet::*;
//在这里把pallet所有的公共项暴露给外面使用。
#[frame_support::pallet]
pub mod pallet {
	use super::*;
	//导入了当前模块前一级的所有暴露的模块
	use frame_support::pallet_prelude::*;
	//模块中包含了一些常用的类型和 trait，例如 VecU8、Weight、DispatchResult、Parameter
	// 等。这些类型和 trait 在编写 Substrate pallet 时经常会用到。
	use frame_system::pallet_prelude::*;
	//模块中包含了一些常用的类型和 trait，例如 BlockNumberFor、CheckWeight、Pallet、RawOrigin 等
	use frame_support::{
		traits::{Currency, ExistenceRequirement, Randomness, StorageVersion},
		PalletId,
		//PalletId 是一个用于标识 pallet 的类型。它是一个 8 字节的数组，通常用于生成一个独特的账户
		// ID，这个账户 ID 可以被 pallet 用于作为其资产的拥有者。
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
	//use crate::migrations; 将 migrations 模块引入当前作用域。这意味着你可以在当前文件中直接使用
	// migrations 模块中的公共项，而不需要写出完整的路径。 由于已经暴露在crate一级目录了，
	// 所以可以直接使用crate::migrations;
	pub type KittyId = u32;
	//声明了一个类型，类型名为KittyId，类型为u32
	pub type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
	//声明BalanceOf<T>类型，T as Config表示T类型实现了Config这个trait。
	//Config::Currency表示Config这个trait里面的Currency这个类型。
	//Config::Currency 类型实现了 Currency trait，而 Currency trait 的关联类型 AccountId 被指定为
	// <<T as frame_system::Config>::AccountId>。
	#[derive(
		Encode, Decode, Clone, Copy, RuntimeDebug, PartialEq, Eq, Default, TypeInfo, MaxEncodedLen,
	)]
	//添加了集中trait实现，encode和decode是用来序列化和反序列化的，clone和copy是用来复制的，
	// runtimeDebug是用来调试的，partialEq和Eq是用来比较的，default是用来默认的，
	// typeInfo是用来获取类型信息的，maxEncodedLen是用来获取最大编码长度的。
	// maxencodedlen是限制编码的最大长度，防止长度过大出现问题。 pub struct Kitty(pub [u8; 16]);
	pub struct Kitty {
		pub dna: [u8; 16],
		pub name: [u8; 4],
	}
	//创建一个公共项的结构体，这个结构体储存了，kitty的信息。
	const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);
	//声明了一个常量，sub提供了StorageVersion类型，可以为每个pallet的储存指定一个版本。
	//使用StorageVersion::new(1)来创建一个新的storageversion实例，版本号为1。
	#[pallet::pallet]
	//是一个属性宏，它用于标记 pallet 的主要结构体。这个宏会为结构体添加一些 Substrate
	// 运行时所需的方法和类型。在 Substrate 中，每个 pallet 都需要有一个这样的结构体。
	#[pallet::storage_version(STORAGE_VERSION)]
	//版本号也需要定义在pallet里面
	//通过这个属性宏把版本号定义在pallet里面
	//在 Substrate 中，每个 pallet 都有自己的存储空间，用于存储其状态。当你更新 pallet
	// 的代码时，可能会改变存储的结构。为了处理这种情况，Substrate 提供了存储版本，让你可以为每个
	// pallet 的存储指定一个版本。STORAGE_VERSION 是一个常量，表示存储的版本号。
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
		//表示pallet的事件类型可以从Event<Self> 类型转换而来，并且是
		// frame_system::Config::RuntimeEvent 类型的一种。 self类似方法里面的self，
		// 这里是指实现了frame_system::Config这个trait的类型。 谁实现了这个trait谁就是self。
		//在运行时模块的实现方法如下
		//impl frame_system::Config for MyRuntime {
		// type RuntimeEvent = Event;
		// 其他关联类型的实现...
		// }
		//todo 这条代码的意思是pallet的IO要符合runtime的io？
		//这两个约束确保了 pallet 的事件可以被正确地转换并包含在 runtime 的事件中，这样，当 pallet
		// 触发一个事件时，这个事件就可以被 runtime 捕获并包含在区块中。

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
		//Self::AccountId也就是runtime的AccountId被定义的类型。
		#[pallet::constant]
		//#[pallet::constant] 宏的作用是标记 pallet 中的常量，让你可以在链规范中设置这些常量的值。
		//链规范是 Substrate 框架中用于描述区块链的配置的一种格式
		//链规范通常以 JSON 文件的形式提供，可以在启动 Substrate 节点时指定。如substrate --chain
		// my_chain_spec.json 这些常量的值在链运行时不会改变。一旦 Substrate
		// 节点启动，并根据链规范初始化了状态，这些常量的值就固定了，不会再改变。
		// 即使在链的运行过程中发生了各种交易和状态变化，这些常量的值也不会改变。
		type KittyPrice: Get<BalanceOf<Self>>;
		//KittyPrice是一个可以获取帐户余额类型值的常量。
		//这里的self是指实现了frame_system::Config这个trait的类型。
		//这个是类型，它实现了一个Get的trait。可以通过get()方法取得一个值，这个值的类型是Balance。
		// ——周俊老师 todo 注意这里仅仅是一个类型声明，规定了kittyprice的类型，赋值在runtime中：
		//todo 在runtime的 parameter_types! 中赋值。
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
	//在这个例子中，<NextKittyId<T>>::get() 调用了 next_kitty_id 函数，读取了 NextKittyId
	// 存储项的值
	pub type NextKittyId<T> = StorageValue<_, KittyId, ValueQuery>;
	//这里的<T>是一个泛型，表示NextKittyId是一个泛型类型。
	//在这里T没有config的类型约束
	//ValueQuery定义储存项的查询行为，这里是一个值查询，表示可以直接读取储存项的值。
	//使用ValueQuery时，如果这个值不存在，那么将返回 u32 类型的默认值（即 0）。
	//OptionQuery 表示当你尝试读取一个不存在的值时，应该返回 None。
	//如果不指定查询行为，会默认使用optionquery查询行为。
	//ResultQuery查询的值不存在时返回err。
	//所有储存类型都有查询行为的参数。
	#[pallet::storage]
	#[pallet::getter(fn kitties)]
	pub type Kitties<T> = StorageMap<_, Blake2_128Concat, KittyId, Kitty>;

	#[pallet::storage]
	#[pallet::getter(fn kitty_owner)]
	pub type KittyOwner<T: Config> = StorageMap<_, Blake2_128Concat, KittyId, T::AccountId>;
	//当储存项使用到config中关联类型时，需要使用T::Config类型约束。
	#[pallet::storage]
	#[pallet::getter(fn kitty_parents)]
	pub type KittyParents<T> = StorageMap<_, Blake2_128Concat, KittyId, (KittyId, KittyId)>;
	//这里没有使用到T::Config实现类型约束，所以不需要使用T::Config类型约束。
	#[pallet::storage]
	#[pallet::getter(fn kitty_on_sale)]
	pub type KittyOnSale<T> = StorageMap<_, Blake2_128Concat, KittyId, ()>;
	//除了Blake2_128Concat外还有其他的哈希函数如：Twox128、Twox64Concat。
	//Twox128它提供了良好的性能和足够的随机性，可以防止哈希碰撞，但是，
	// 它不会将原始值连接到哈希值的末尾 Twox64Concat它提供了良好的性能和足够的随机性，
	// 可以防止哈希碰撞，它会将原始值连接到哈希值的末尾这样可以保证即使在哈希碰撞的情况下，
	// 生成的键也是唯一的。
	#[pallet::event]
	//定义了pallet的事件
	//pallet的事件和runtime的事件是不同的，pallet的事件是在pallet里面定义的，
	// runtime的事件是在runtime里面定义的。 这些是在 pallet 内部定义的事件，通常用于表示 pallet
	// 中的某个操作（例如转账、创建新的实体等）已经成功完成。当这些操作成功完成时，pallet
	// 会触发一个事件，并将其包含在区块中。这些事件可以被链上的其他 pallet
	// 或链下的应用程序监听和处理。 Runtime 事件：这些是在 runtime 层面定义的事件，它们通常用于表示
	// runtime 中的某个重要操作（例如升级
	// runtime、更改治理参数等）已经成功完成。当这些操作成功完成时，runtime
	// 会触发一个事件，并将其包含在区块中。这些事件可以被链上的其他 pallet
	// 或链下的应用程序监听和处理。
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	//是 Substrate 框架中的一个属性宏，它用于生成一个名为 deposit_event
	// 的函数，这个函数可以用来触发 pallet 的事件。 当某个操作成功完成时，
	// 通常会触发一个事件来通知链上的其他 pallet 或链下的应用程序。这个事件是通过调用 deposit_event
	// 函数来触发的。 pub(super) 表示这个函数只能在当前模块或父模块中被访问，这是一种访问控制。
	pub enum Event<T: Config> {
		KittyCreated { who: T::AccountId, kitty_id: KittyId, kitty: Kitty },
		KittyBred { who: T::AccountId, kitty_id: KittyId, kitty: Kitty },
		KittyTransferred { who: T::AccountId, recipient: T::AccountId, kitty_id: KittyId },
		KittyOnSale { who: T::AccountId, kitty_id: KittyId },
		KittyBought { who: T::AccountId, current_owner: T::AccountId, kitty_id: KittyId },
	}
	//#[pallet::call]
	// impl<T: Config> Pallet<T> {
	// #[pallet::weight(10_000)]
	// pub fn create_kitty(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
	// let who = ensure_signed(origin)?;

	// 创建一个 kitty
	// let kitty_id = ...;
	// let kitty = ...;

	// 触发一个 KittyCreated 事件
	// Self::deposit_event(Event::KittyCreated { who: who.clone(), kitty_id, kitty });
	//todo 在这里使用deposit_event触发事件，这个事件是在pallet里面定义的。
	// Ok(().into())
	// }
	// }
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
	//定义pallet的错误
	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		//todo 为什么这里要加上BlockNumberFor<T>？
		//BlockNumberFor<T> 被用作 Hooks trait 的类型参数。
		//Hooks trait 中的一些钩子函数可能需要使用到区块编号。例如，on_initialize 和 on_finalize
		// 钩子函数都会接收一个区块编号作为参数，这个区块编号表示当前正在处理的区块。
		// 虽然 on_runtime_upgrade 钩子函数并没有直接使用到区块编号，但是 Hooks<BlockNumberFor<T>>
		// 的写法是为了满足 Hooks trait 的类型签名
		//
		//Hooks是一个trait，它定义了一些钩子函数，这些钩子函数会在特定的时机被自动调用。
		//BlockNumberFor<T>是一个类型，它表示区块号。
		//并不是Hooks都需要BlockNumberFor<T>的类型约束。
		fn on_runtime_upgrade() -> Weight {
			//每个交易或操作都有一个权重（Weight），
			// 这个权重表示这个交易或操作的复杂性或需要的计算资源。
			// 权重用于限制区块中可以包含的交易数量，以防止区块过大导致网络拥堵
			migrations::v1::migrate::<T>()
			//TODO 为什么要返回一个权重？
		}
	}
	//定义pallet的钩子函数
	//钩子函数是它会在特定的时机被自动调用。
	// on_initialize：在每个区块开始处理之前调用。
	// on_finalize：在每个区块处理完成之后调用。
	// on_runtime_upgrade：在运行时升级时调用。
	// 在 runtime 升级时，执行 migrations::v1::migrate::<T>()
	// 函数进行数据迁移，并返回数据迁移的权重。

	#[pallet::call]
	//宏用于标记一个 impl 块，这个 impl 块中定义了 pallet 的调用接口。
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		//调用接口的序号
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		//定一个调用接口的权重
		pub fn create(origin: OriginFor<T>, name: [u8; 4]) -> DispatchResult {
			//origin标记了调用的来源，OriginFor<T>这个的语义可以理解为对pallet调用的调用者
			//调用这个借口的人是谁，这个人就是调用者。
			//name是携带的一些调用时传递给pallet的信息。
			let who = ensure_signed(origin)?;
			//确保调用者对交易已经签名，返回的是一个AccountId类型的值。
			let kitty_id = Self::get_next_id()?;
			//通过pallet的辅助函数，get_next_id获取下一个id。 ？
			// 返回一个ok包裹的内部的值，不是ok（**）。
			let dna = Self::random_value(&who);
			//通过pallet的辅助函数，random_value获取一个随机值。
			let kitty = Kitty { dna, name };
			//创建一个kitty，由Kitty体组成。
			let price = T::KittyPrice::get();
			// T::Currency::reserve(&who, price)?;
			//获取kitty的价格，这里kitty的价格是一个常量，常量是从parameter_types!
			// 中定义的，因为在类型声明时 是Get<BalanceOf<Self>>，get给了kittyprice一个get方法。
			T::Currency::transfer(
				&who,
				&Self::get_account_id(),
				price,
				ExistenceRequirement::KeepAlive,
				//转账发起时需要确定转账完成后，帐户是存活的，否则转账失败。
			)?;
			//currency是一个trait，里面有transfer方法，可以转账。
			//从签名者转账到pallet的账户，转账的金额是price，转账的要求是KeepAlive。
			//price是一个常量，所有的kitty都是一样的价格。
			Kitties::<T>::insert(kitty_id, &kitty);
			//把kitty的唯一标识和kitty的信息插入到kitties中。
			KittyOwner::<T>::insert(kitty_id, &who);
			//把kitty的唯一标识和kitty的拥有者插入到kittyowner中。
			Self::deposit_event(Event::KittyCreated { who, kitty_id, kitty });
			//触发一个kittycreated事件，这个事件是在pallet里面定义的。
			Ok(())
			//返回调用结果。
		}

		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn breed(
			origin: OriginFor<T>,
			kitty_id_1: KittyId,
			kitty_id_2: KittyId,
			name: [u8; 4],
			//繁殖一个kitty，需要两个kitty作为父母，产生一个kitty作为孩子。
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(kitty_id_1 != kitty_id_2, Error::<T>::SameKittyId);
			//确保两个两个不是同一个kitty。
			// let kitty_1 = Self::kitties(kitty_id_1).ok_or(Error::<T>::InvalidKittyId)?;
			// let kitty_2 = Self::kitties(kitty_id_2).ok_or(Error::<T>::InvalidKittyId)?;

			let kitty_id = Self::get_next_id()?;
			//获取一个新的kitty_id，供孩子使用。

			// let mut kitty = Kitty::default();
			// let selector = Self::random_value(&who);

			let dna = [0u8; 16];
			//初始化dna，值为0，长度为16，类型为u8。
			let kitty = Kitty { dna, name };
			//创建一个kitty，由Kitty结构体组成。

			// for i in 0..16 {
			// 	kitty.0[i] = (selector[i] & kitty_1.0[i]) | (!selector[i] & kitty_2.0[i]);
			// }

			let price = T::KittyPrice::get();
			//得到kitty的价格。

			// T::Currency::reserve(&who, price)?;
			T::Currency::transfer(
				&who,
				&Self::get_account_id(),
				price,
				ExistenceRequirement::KeepAlive,
			)?;
			//转账

			Kitties::<T>::insert(kitty_id, &kitty);
			KittyOwner::<T>::insert(kitty_id, &who);
			KittyParents::<T>::insert(kitty_id, (kitty_id_1, kitty_id_2));
			//除了保存kitty的信息，还要保存kitty的父母信息。

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
			//确保kitty_id是有效的。

			ensure!(Self::kitty_owner(kitty_id) == Some(who.clone()), Error::<T>::NotOwner);
			//确保调用者是kitty的拥有者。

			KittyOwner::<T>::insert(kitty_id, &recipient);
			//把kitty的拥有者改为接收者。

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
			//使用kitty_on_sale函数,对储存库进行查询，看是否有这个kitty_id，如果有，返回错误。
			<KittyOnSale<T>>::insert(kitty_id, ());
			//把kitty_id插入到kitty_on_sale中。
			//对kitty进行一个标记
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
				//try_mutate 一个方法，用于尝试修改存储项的值。这个方法接收一个闭包作为参数，这个闭包接收存储项的当前值，并返回一个 Result。如果闭包返回 Ok，那么存储项的值将被修改为 Ok 中的值；如果闭包返回 Err，那么存储项的值将不会被修改，并且 try_mutate 方法将返回这个错误。
				let current_id = *next_id;
				*next_id = next_id.checked_add(1).ok_or(Error::<T>::InvalidKittyId)?;
				Ok(current_id)
			})
		}

		fn random_value(sender: &T::AccountId) -> [u8; 16] {
			//返回一个16位的随机值[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
			//u8范围是0-255。
			let payload = (
				T::Randomness::random_seed(),
				//返回一个随机种子，这个随机种子是一个128位的值。
				&sender,
				//调用者
				<frame_system::Pallet<T>>::extrinsic_index(),
				//extrinsic_index 函数返回当前区块中的当前交易的索引。这个索引是从 0 开始的，并且每处理一个交易就会增加。没有就返回None。
				//这个是查询了当前交易在区块中的index
				//当前交易排在第几就返回几，顺序是从0开始的的。
			);
			payload.using_encoded(blake2_128)
			//这个返回的是一个128位的hash值。
			//useing_encoded首先将payload编码为字节，然后将字节传递给 blake2_128 函数，最后返回一个 128 位的哈希值。
		}

		fn get_account_id() -> T::AccountId {
			T::PalletId::get().into_account_truncating()
			//可能有长度上的不匹配
			//所以用另一个种方法来匹配长度
			//改.into_account()为
		}
	}
}
