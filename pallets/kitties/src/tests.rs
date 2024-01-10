// use crate::{mock::*, Error, Event};
// use frame_support::{assert_noop, assert_ok};

// #[test]
// fn test_create() {
// 	new_test_ext().execute_with(|| {
// 		let kitty_id = 0;
// 		let account_id = 1;
// 		assert_eq!(KittiesModule::next_kitty_id(), kitty_id);
// 		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
// 		assert_eq!(KittiesModule::next_kitty_id(), kitty_id + 1);
// 		assert_eq!(KittiesModule::kitties(kitty_id).is_some(), true);
// 		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));
// 		assert_eq!(KittiesModule::kitty_parents(kitty_id), None);
// 		crate::NextKittyId::<Test>::set(crate::KittyId::max_value());
// 		assert_noop!(
// 			KittiesModule::create(RuntimeOrigin::signed(account_id)),
// 			Error::<Test>::InvalidKittyId
// 		);
// 		let kitty = KittiesModule::kitties(kitty_id).expect("there should be a kitty");
// 		System::assert_last_event(Event::KittyCreated { who: account_id, kitty_id, kitty }.into());
// 	})
// }

// #[test]
// fn test_breed() {
// 	new_test_ext().execute_with(|| {
// 		let account_id = 1;
// 		let kitty_id = 0;

// 		assert_noop!(
// 			KittiesModule::breed(RuntimeOrigin::signed(account_id), kitty_id, kitty_id),
// 			Error::<Test>::SameKittyId
// 		);
// 		assert_noop!(
// 			KittiesModule::breed(RuntimeOrigin::signed(account_id), kitty_id, kitty_id + 1),
// 			Error::<Test>::InvalidKittyId
// 		);
// 		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
// 		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
// 		let child_id = kitty_id + 2;
// 		assert_eq!(KittiesModule::next_kitty_id(), child_id);
// 		assert_ok!(KittiesModule::breed(RuntimeOrigin::signed(account_id), kitty_id, kitty_id + 1));
// 		assert_eq!(KittiesModule::next_kitty_id(), child_id + 1);
// 		assert_eq!(KittiesModule::kitties(child_id).is_some(), true);
// 		assert_eq!(KittiesModule::kitty_owner(child_id), Some(account_id));
// 		assert_eq!(KittiesModule::kitty_parents(child_id), Some((kitty_id, kitty_id + 1)));
// 		let kitty = KittiesModule::kitties(child_id).expect("there should be a kitty");
// 		System::assert_last_event(
// 			Event::KittyBred { who: account_id, kitty_id: child_id, kitty }.into(),
// 		);
// 	})
// }

// #[test]
// fn test_transfer() {
// 	new_test_ext().execute_with(|| {
// 		let account_id = 1;
// 		let kitty_id = 0;
// 		let recipient = 2;
// 		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
// 		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));
// 		assert_noop!(
// 			KittiesModule::transfer(RuntimeOrigin::signed(recipient), recipient, kitty_id),
// 			Error::<Test>::NotOwner
// 		);
// 		assert_ok!(KittiesModule::transfer(RuntimeOrigin::signed(account_id), recipient, kitty_id));
// 		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(recipient));
// 		System::assert_last_event(
// 			Event::KittyTransferred { who: account_id, recipient, kitty_id }.into(),
// 		);
// 		assert_ok!(KittiesModule::transfer(RuntimeOrigin::signed(recipient), account_id, kitty_id));
// 		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));
// 		System::assert_last_event(
// 			Event::KittyTransferred { who: recipient, recipient: account_id, kitty_id }.into(),
// 		);
// 	})
// }
