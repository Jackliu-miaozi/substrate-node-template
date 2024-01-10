//mock.rs创建一个模拟运行时环境。以便在不需要节点的情况下测试pallet。
use crate as pallet_kitties;
//为当前crate引用一个别名，crate用于引用当前crate。
//as 用来创建别名。
use frame_support::traits::{ConstU16, ConstU64};
//引入ConstU16和ConstU64,这两个是常量，ConstU16是一个u16类型的常量，ConstU64是一个u64类型的常量。
use pallet_insecure_randomness_collective_flip;
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

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
//创建了一个类型别名
//是一个用于测试的未经检查的外部交易类型
//未经检查的外部交易是指还没有经过签名验证和交易费支付检查的交易。
type Block = frame_system::mocking::MockBlock<Test>;
//MockBlock 是一个用于测试的模拟区块类型
//它包含了一个区块头和一个交易列表。
frame_support::construct_runtime!(
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
	{
		System: frame_system,
		//这是在运行时中添加 frame_system pallet 的地方。
		//frame_system pallet 提供了一些基础的系统功能，例如区块和事件的处理。
		KittiesModule: pallet_kitties,
		//这是在运行时中添加 pallet_kitties pallet 的地方。
		Randomness: pallet_insecure_randomness_collective_flip,
		//运行时包含的所有 pallet 都必须在这里声明。
	}
	//这个运行时包含了 frame_system、pallet_kitties 和
	// pallet_insecure_randomness_collective_flip 这三个 pallet。
);
//

impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_kitties::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Randomness = Randomness;
	type Currency: Currency<Self::AccountId>;
}

impl pallet_insecure_randomness_collective_flip::Config for Test {}

pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut ext: sp_io::TestExternalities =
		frame_system::GenesisConfig::default().build_storage::<Test>().unwrap().into();
	ext.execute_with(|| System::set_block_number(1));
	ext
}
