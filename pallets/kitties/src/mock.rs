//mock.rs创建一个模拟运行时环境。以便在不需要节点的情况下测试pallet。
use crate as pallet_kitties;

//为当前crate引用一个别名，crate用于引用当前crate。
//as 用来创建别名。
use frame_support::{
	construct_runtime, parameter_types,
	traits::{ConstU128, ConstU16, ConstU32, ConstU64},
	PalletId,
};
//引入ConstU16和ConstU64,这两个是常量，ConstU16是一个u16类型的常量，ConstU64是一个u64类型的常量。
use pallet_balances;
use pallet_insecure_randomness_collective_flip;
// use frame_system;
//pallet支持热插拔，这行代码引入了一个pallet他是一个不安全的随机数生成器。
//他是subtrate内置的。
//引入一个不安全的随机数生成器
use sp_core::H256;
//sp_core提供了一些核心的类型和功能。
//H256是一个256位的哈希值。是ethereum风格的。
//Substrate Primitives

use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};
//sp_runtime是一个运行时库，提供了一些构建区块链运行时的必须的类型和trait。
//testing::Header是一个用于测试区块头的类型，使用这个类型创建模拟的区块头。
//区块头一般包含以下内容
//区块的版本号
// 前一个区块的哈希值
// 区块的时间戳
// 区块的难度目标
// 区块的随机数（Nonce）
// 区块的 Merkle 树根
//traits::BlakeTwo256是一个哈希算法，用于计算区块头的哈希值。
//traits::IdentityLookup是一个用于查找账户的trait。
type Balance = u128;
const EXISTENTIAL_DEPOSIT: u128 = 500;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
//创建了一个类型别名
//是一个用于测试的未经检查的外部交易类型
//未经检查的外部交易是指还没有经过签名验证和交易费支付检查的交易。
type Block = frame_system::mocking::MockBlock<Test>;
//MockBlock 是一个用于测试的模拟区块类型
//它包含了一个区块头和一个交易列表。
construct_runtime!(
	//他定义了一个模拟运行时
	pub enum Test where
	//声明一个Test的枚举类型
	//他是一个模拟运行时
	//包含了三个pallet
		Block = Block,
		//这是定义区块类型的地方。
		//=左边的Block是一个固定的关键字
		NodeBlock = Block,
		//这是定义节点区块类型的地方。
		//=左边是一个固定的关键字
		UncheckedExtrinsic = UncheckedExtrinsic,
		//这是定义未经检查的外部交易类型的地方。
		//Block：用于指定运行时的区块类型。
		// NodeBlock：用于指定节点的区块类型。在大多数情况下，NodeBlock 的类型和 Block 的类型是一样的。
		// UncheckedExtrinsic：用于指定运行时的未经检查的外部交易类型。未经检查的外部交易是指还没有经过签名验证和交易费支付检查的交易。
		//还有其他很多关键字
		//没有指定的关键字类型会被约束为默认的类型。
	{
		//这是在运行时中添加 frame_system pallet 的地方。
		//frame_system pallet 提供了一些基础的系统功能，例如区块和事件的处理。
		KittiesModule: pallet_kitties,
		//这是在运行时中添加 pallet_kitties pallet 的地方。
		Randomness: pallet_insecure_randomness_collective_flip,
		//运行时包含的所有 pallet 都必须在这里声明。
		Balances: pallet_balances,
		System: frame_system,
	}
	//这个运行时包含了 frame_system、pallet_kitties 和
	// pallet_insecure_randomness_collective_flip 这三个 pallet。
);
//

impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	//用于过滤调用的类型。这里设置为 Everything，表示所有的调用都会被接受。
	type BlockWeights = ();
	//用于定义区块的权重和长度。这里设置为 ()，表示使用默认的权重和长度。
	type BlockLength = ();
	//用于定义区块的权重和长度。这里设置为 ()，表示使用默认的权重和长度。
	type DbWeight = ();
	//用于定义数据库操作的权重。这里设置为 ()，表示使用默认的权重。
	type RuntimeOrigin = RuntimeOrigin;
	//todo 为什么=右边是同名的，但是在别的地方还找不到声明？
	type RuntimeCall = RuntimeCall;
	//todo 为什么=右边是同名的，但是在别的地方还找不到声明？
	type Index = u64;
	//用于定义账户的索引类型。这里设置为 u64，表示账户的索引类型是 u64。
	type BlockNumber = u64;
	//用于定义区块号的类型。这里设置为 u64，表示区块号的类型是 u64。
	type Hash = H256;
	//用于定义哈希值的类型。这里设置为 H256，表示哈希值的类型是 H256。
	type Hashing = BlakeTwo256;
	//用于定义哈希算法的类型。这里设置为 BlakeTwo256，表示哈希算法的类型是 BlakeTwo256。
	type AccountId = u32;
	//用于定义账户的类型。这里设置为 u64，表示账户的类型是 u64。
	type Lookup = IdentityLookup<Self::AccountId>;
	//用于定义账户查找的类型。这里设置为 IdentityLookup<Self::AccountId>，表示账户查找的类型是
	// IdentityLookup<Self::AccountId>。
	type Header = Header;
	//用于定义区块头的类型。这里设置为 Header，表示区块头的类型是 Header。
	type RuntimeEvent = RuntimeEvent;
	//用于定义运行时事件的类型。这里设置为 RuntimeEvent，表示运行时事件的类型是 RuntimeEvent。
	//这个runtimeevent是在pallet_kitties中定义的。
	type BlockHashCount = ConstU64<250>;
	//用于定义区块哈希数量的类型。这里设置为 ConstU64<250>，表示区块哈希数量的类型是
	// ConstU64<250>。 它定义了区块链保存的最近区块哈希的数量。
	//ConstU64<250> 是一个常量类型，它表示一个固定的 u64 值，这里值是250。
	//这行代码的意思是，区块链将保存最近的 250
	// 个区块的哈希。这是为了在需要时可以查询这些区块的哈希，例如在验证交易时。但是，
	// 超过这个数量的旧区块哈希将被丢弃，以节省状态数据库的存储空间。 这并不影响区块数据本身，
	// 区块数据通常会被完整地保存在区块链节点的本地存储中。
	// 只是你无法再通过哈希直接从状态数据库中查询到这些旧的区块。
	type Version = ();
	//用于定义运行时版本的类型。这里设置为 ()，表示运行时版本的类型是 ()。
	type PalletInfo = PalletInfo;
	//用于定义 pallet 信息的类型。这里设置为 PalletInfo，表示 pallet 信息的类型是 PalletInfo。
	//PalletInfo 是一个 trait，它定义了一些用于获取 pallet 信息的方法。
	//todo 在lib.rs和mock.rs中都找不到PalletInfo的定义

	type AccountData = pallet_balances::AccountData<u128>;
	//用于定义账户数据的类型。这里设置为 ()，表示使用默认的帐户数据类型。
	type OnNewAccount = ();
	//用于定义新账户创建时的操作。这里设置为 ()，表示不执行任何操作。
	type OnKilledAccount = ();
	//用于定义账户被消除时的操作。这里设置为 ()，表示不执行任何操作。
	type SystemWeightInfo = ();
	//用于定义系统权重信息的类型。这里设置为 ()，表示使用默认的系统权重信息类型。
	type SS58Prefix = ConstU16<42>;
	//用于定义 SS58 编码的前缀。这里设置为 ConstU16<42>，表示 SS58 编码的前缀是 42。
	type OnSetCode = ();
	//用于定义设置代码时的操作。这里设置为 ()，表示不执行任何操作。
	type MaxConsumers = ConstU32<16>;
	//用于定义最大消费者数量的类型。这里设置为 ConstU32<16>，表示最大消费者数量的类型是
	// ConstU32<16>。 使用ConstU32<16>类型，的好处是，在编译时ConstU32 可以作为类型参数，而 16
	// 不能。 在 Rust 中，类型参数必须是类型，而不能是值。
}

parameter_types! {
	pub KittyPrice: Balance = EXISTENTIAL_DEPOSIT *10;
	pub KittyPalletId: PalletId = PalletId(*b"py/kitty");
}

impl pallet_kitties::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	//用于定义运行时事件的类型。这里设置为 RuntimeEvent，表示运行时事件的类型是 RuntimeEvent。
	//这个runtimeevent是在pallet_kitties中定义的。
	type Randomness = Randomness;
	//用于定义随机数生成器的类型。这里设置为 Randomness，表示随机数生成器的类型是 Randomness。
	type Currency = Balances;
	//用于定义货币类型。
	type KittyPrice = KittyPrice;
	type PalletId = KittyPalletId;
}
//这里为pallet_kitties的配置进行具体的实现
//这里的= 左边的Randomness和Currency是在pallet_kitties中定义的。
//在这里进行具体的实现。
impl pallet_insecure_randomness_collective_flip::Config for Test {}
//这个pallet没有具体的关联类型，所以这里是空。
//虽然是关联类型是空，但是也需要把它实现给Test。这个虚拟的runtime。

impl pallet_balances::Config for Test {
	type MaxLocks = ();
	//一个帐户可以拥有的最大锁定数量
	type MaxReserves = ();
	//一个帐户可以拥有的最大储备数量
	type ReserveIdentifier = [u8; 8];
	//用于定义储备标识符的类型。这里设置为 [u8; 8]，表示储备标识符的类型是 [u8; 8]。
	type Balance = u128;
	//余额的类型
	type DustRemoval = ();
	//用于定义尘埃移除的类型。这里设置为 ()，表示不执行尘埃移除。保持帐户活跃的最小余额。
	type RuntimeEvent = RuntimeEvent;
	//用于定义运行时事件的类型。这里设置为 RuntimeEvent，表示运行时事件的类型是 RuntimeEvent。
	type ExistentialDeposit = ConstU128<1>;
	//最小存款数量是1
	type AccountStore = System;
	//定义了账户存储的类型。
	type WeightInfo = ();
	//定义了权重信息的类型。
	type HoldIdentifier = ();
	//定义了持有标识符的类型。
	type MaxHolds = ();
	//定义了一个账户可以拥有的最大持有数量。
	type FreezeIdentifier = ();
	//定义了冻结标识符的类型。
	type MaxFreezes = ();
	//定义了一个账户可以拥有的最大冻结数量。
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	//sp_io::TestExternalities 是 Substrate 中用于模拟区块链环境的一个类型，
	//它可以让你在测试中模拟执行区块链交易
	let mut ext: sp_io::TestExternalities =
		frame_system::GenesisConfig::default().build_storage::<Test>().unwrap().into();
	//这里创建了一个新的TestExternalities
	//这个实例包含了默认的创世区块配置，并且被转换为 TestExternalities 类型
	ext.execute_with(|| System::set_block_number(1));
	//设置初始的区块号为 1。execute_with 方法允许你在 TestExternalities 的环境中执行一个闭包。
	//这个闭包更改了ext
	//所以这里的ext是一个可变的引用
	ext
}
