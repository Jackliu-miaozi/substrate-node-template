use crate::*;
//一个包是cargo的一个基本单元，每个包包含一个cargo.toml文件。
// 每个crate包含一个由模块组成的树形结构，其中一个模块是根模块，它与包同名。
// 每个模块可以包含任意数量的项，包括其他模块。模块和项的名称可以是公共的或私有的。
// 私有项只能被同一个模块中的其他项访问。公共项可以被任何模块访问。默认情况下，项是私有的，
// 但可以通过pub关键字使其公开。 一个包可以理解为一个项目
//这行代码的作用是从 crate 的根（也就是 lib.rs 或 main.rs 文件）引入所有公共（public）项。
//一个包可以有多个二进制 crate，因此可以有多个 main.rs 文件。但是，一个包只能有一个库
// crate，因此只能有一个 lib.rs 文件。

//这些 main.rs 和 lib.rs 文件通常位于不同的目录中，以避免混淆。例如，你可能会在 src/bin
// 目录下有多个二进制 crate，每个 crate 都有自己的 main.rs 文件。
// 模块是命名空间系统，用于创建库内部的结构。模块内部的项（函数、方法、结构体、枚举、
// 模块和常量）默认是私有的，但可以通过 pub
// 关键字使其公开。使其变成公共项后，其他代码就可以使用它了。

//当你引入一个模块时，你会引入所有的pub标记的内容，包括pub use 的内容。
use frame_support::{
	migration::storage_key_iter, pallet_prelude::*, storage::StoragePrefixedMap,
	traits::GetStorageVersion, weights::Weight, Blake2_128Concat,
};


#[derive(
	Encode, Decode, Clone, Copy, RuntimeDebug, PartialEq, Eq, Default, TypeInfo, MaxEncodedLen,
)]
pub struct OldKitty(pub [u8; 16]);

pub fn migrate<T: Config>() -> Weight {
	let on_chain_version = Pallet::<T>::on_chain_storage_version();
	let current_version = Pallet::<T>::current_storage_version();
	if on_chain_version != 0 {
		return Weight::zero();
	}
	if current_version != 1 {
		return Weight::zero();
	}
	let moudle = Kitties::<T>::module_prefix();
	let item = Kitties::<T>::storage_prefix();
	//得到两个最前面的前缀
	for (index, kitty) in
		storage_key_iter::<KittyId, OldKitty, Blake2_128Concat>(moudle, item).drain()
	{
		let new_kitty = Kitty { dna: kitty.0, name: *b"abcd", };
		Kitties::<T>::insert(index, &new_kitty);
	}
	Weight::zero()
}
