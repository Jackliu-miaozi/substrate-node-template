use crate::*;
use frame_support::{
	migration::storage_key_iter, pallet_prelude::*, storage::StoragePrefixedMap,
	traits::GetStorageVersion, weights::Weight, Blake2_128Concat,
};

#[derive(
	Encode, Decode, Clone, Copy, RuntimeDebug, PartialEq, Eq, Default, TypeInfo, MaxEncodedLen,
)]

pub struct KittyV1{
    pub dna: [u8; 16],
    pub name: [u8; 8],
}

pub fn migrate<T: Config>() -> Weight {
    // let on_chain_version = Pallet::<T>::on_chain_storage_version();
    let current_version = Pallet::<T>::current_storage_version();
    // if on_chain_version != 1 {
    //     return Weight::zero();
    // }
    if current_version != 4 {
        return Weight::zero();
    }
    let moudle = Kitties::<T>::module_prefix();
    let item = Kitties::<T>::storage_prefix();
    //得到两个最前面的前缀
    for (index, kitty) in
        storage_key_iter::<KittyId, KittyV1, Blake2_128Concat>(moudle, item).drain()
    {
        let mut name = [0u8; 8];
        for i in 0..name.len() {
            name[i] = kitty.name[i%4]+1;
        }
        let new_kitty = Kitty { dna: kitty.dna, name};
        Kitties::<T>::insert(index, &new_kitty);
    }
    Weight::zero()
}
