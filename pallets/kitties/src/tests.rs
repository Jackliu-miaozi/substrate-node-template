use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_create() {
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let account_id = 1;

		assert_eq!(KittiesModule::next_kitty_id(), kitty_id);

		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));

		assert_eq!(KittiesModule::next_kitty_id(), kitty_id + 1);

		assert_eq!(KittiesModule::kitties(kitty_id).is_some(), true);

		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));

		assert_eq!(KittiesModule::kitty_parents(kitty_id), None);

		crate::NextKittyId::<Test>::set(crate::KittyId::max_value());

		assert_noop!(
			KittiesModule::create(RuntimeOrigin::signed(account_id)),
			Error::<Test>::InvalidKittyId
		);
	});
}

#[test]
fn it_works_for_breed() {
	new_test_ext().execute_with(|| {
		let kitty_id_1 = 0;
		let account_id = 1;

		assert_noop!(
			KittiesModule::breed(RuntimeOrigin::signed(account_id), kitty_id_1, kitty_id_1),
			Error::<Test>::SameKittyId
		);
		assert_noop!(
			KittiesModule::breed(RuntimeOrigin::signed(account_id), kitty_id_1, kitty_id_1 + 1),
			Error::<Test>::InvalidKittyId
		);

		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));

		assert_eq!(KittiesModule::next_kitty_id(), kitty_id_1 + 2);

		assert_ok!(KittiesModule::breed(
			RuntimeOrigin::signed(account_id),
			kitty_id_1,
			kitty_id_1 + 1
		));

		let breed_kitty_id = 2;

		assert_eq!(KittiesModule::next_kitty_id(), breed_kitty_id + 1);

		assert_eq!(KittiesModule::kitties(breed_kitty_id).is_some(), true);

		assert_eq!(KittiesModule::kitty_owner(breed_kitty_id), Some(account_id));

		assert_eq!(
			KittiesModule::kitty_parents(breed_kitty_id),
			Some((kitty_id_1, kitty_id_1 + 1))
		);
	})
}

#[test]
fn it_works_for_transfer() {
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let account_id = 1;
		let account_id_2 = 2;

		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));

		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));

		assert_noop!(
			KittiesModule::transfer(RuntimeOrigin::signed(account_id_2), account_id_2, kitty_id),
			Error::<Test>::NotOwner
		);
		assert_ok!(KittiesModule::transfer(
			RuntimeOrigin::signed(account_id),
			account_id_2,
			kitty_id
		));

		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id_2));
	})
}

