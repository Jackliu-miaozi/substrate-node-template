use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};

#[test]
fn test_create() {
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let account_id = 1;
		assert_eq!(KittiesModule::next_kitty_id(), kitty_id);
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id), [0u8; 4]));
		assert_eq!(KittiesModule::next_kitty_id(), kitty_id + 1);
		assert_eq!(KittiesModule::kitties(kitty_id).is_some(), true);
		//数据库中存在kitty_id
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));
		//数据库中存在kitty_id的主人
		assert_eq!(KittiesModule::kitty_parents(kitty_id), None);
		//数据库中不存在kitty_id的父母
		crate::NextKittyId::<Test>::set(crate::KittyId::max_value());
		//设置kitty_id的最大值，max_value()是设置kittyid类型的最大值，这里是u32类型，所以是2^32-1。
		assert_noop!(
			//如果你预期一个操作在特定条件下会失败，并且在失败后不会改变任何状态，那么你就应该使用
			// assert_noop!。
			KittiesModule::create(RuntimeOrigin::signed(account_id), [1u8; 4]),
			Error::<Test>::InvalidKittyId /* 这个错误在create调用next_kitty_id()的值超过了最大值时会出现 */
		);
		//用于断言一个操作在不改变链上状态的情况下失败。
		//kitty_id的值超过了最大值，无法创建
		let kitty = KittiesModule::kitties(kitty_id).expect("there should be a kitty");
		System::assert_last_event(Event::KittyCreated { who: account_id, kitty_id, kitty }.into());
	})
}

#[test]
fn test_breed() {
	new_test_ext().execute_with(|| {
		let account_id = 1;
		let kitty_id = 0;

		assert_noop!(
			KittiesModule::breed(RuntimeOrigin::signed(account_id), kitty_id, kitty_id, [0u8; 4]),
			Error::<Test>::SameKittyId
		);
		assert_noop!(
			KittiesModule::breed(
				RuntimeOrigin::signed(account_id),
				kitty_id,
				kitty_id + 1,
				[0u8; 4]
			),
			Error::<Test>::InvalidKittyId
		);
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id), [0u8; 4]));
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id), [1u8; 4]));
		let child_id = kitty_id + 2;
		assert_eq!(KittiesModule::next_kitty_id(), child_id);
		assert_ok!(KittiesModule::breed(
			RuntimeOrigin::signed(account_id),
			kitty_id,
			kitty_id + 1,
			[2u8; 4]
		));
		assert_eq!(KittiesModule::next_kitty_id(), child_id + 1);
		assert_eq!(KittiesModule::kitties(child_id).is_some(), true);
		assert_eq!(KittiesModule::kitty_owner(child_id), Some(account_id));
		assert_eq!(KittiesModule::kitty_parents(child_id), Some((kitty_id, kitty_id + 1)));
		let kitty = KittiesModule::kitties(child_id).expect("there should be a kitty");
		System::assert_last_event(
			Event::KittyBred { who: account_id, kitty_id: child_id, kitty }.into(),
		);
	})
}

#[test]
fn test_transfer() {
	new_test_ext().execute_with(|| {
		let account_id = 1;
		let kitty_id = 0;
		let recipient = 2;
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id), [0u8; 4]));
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));
		assert_noop!(
			KittiesModule::transfer(RuntimeOrigin::signed(recipient), recipient, kitty_id),
			Error::<Test>::NotOwner
		);
		assert_ok!(KittiesModule::transfer(RuntimeOrigin::signed(account_id), recipient, kitty_id));
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(recipient));
		System::assert_last_event(
			Event::KittyTransferred { who: account_id, recipient, kitty_id }.into(),
		);
		assert_ok!(KittiesModule::transfer(RuntimeOrigin::signed(recipient), account_id, kitty_id));
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));
		System::assert_last_event(
			Event::KittyTransferred { who: recipient, recipient: account_id, kitty_id }.into(),
		);
	})
}

#[test]
fn test_sale() {
	new_test_ext().execute_with(|| {
		let account_id = 1;
		let kitty_id = 0;
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id), [0u8; 4]));
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));
		//使用数据库调用函数返回的结果是用some包裹的。否则是none。
		assert_noop!(
			KittiesModule::sale(RuntimeOrigin::signed(account_id), kitty_id),
			Error::<Test>::NotOwner
		);
		assert_ok!(KittiesModule::sale(RuntimeOrigin::signed(account_id), kitty_id));

		assert_noop!(
			KittiesModule::sale(RuntimeOrigin::signed(account_id), kitty_id),
			Error::<Test>::AlreadyOnSale
		);

		System::assert_last_event(Event::KittyOnSale { who: account_id, kitty_id }.into());
	})
}

#[test]
fn buy() {
	new_test_ext().execute_with(|| {
		let account_id = 1;
		let kitty_id = 0;
		assert_noop!(
			KittiesModule::buy(RuntimeOrigin::signed(account_id), kitty_id),
			Error::<Test>::InvalidKittyId
		);
		//上面的error是预期的错误
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id), [0u8; 4]));
		assert_noop!(
			KittiesModule::buy(RuntimeOrigin::signed(account_id), kitty_id + 1),
			Error::<Test>::NoOwner
		);
		//这个kitty没有主人，所以无法购买
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id + 1), [1u8; 4]));
		//2号用户创建了kitty_id+1
		assert_noop!(
			KittiesModule::buy(RuntimeOrigin::signed(account_id), kitty_id),
			Error::<Test>::AlreadyOwned
		);
		//不能买自己的kitty
		assert_noop!(
			KittiesModule::buy(RuntimeOrigin::signed(account_id), kitty_id + 1),
			Error::<Test>::NotOnSale
		);
		assert_ok!(KittiesModule::sale(RuntimeOrigin::signed(account_id + 1), kitty_id + 1));
		//2号用户将kitty_id+1上架
		assert_ok!(KittiesModule::buy(RuntimeOrigin::signed(account_id), kitty_id + 1));
		//1号用户购买了kitty_id+1
		assert_eq!(KittiesModule::kitty_owner(kitty_id + 1), Some(account_id));
		//kitty_id+1的主人是1号用户
		System::assert_last_event(
			Event::KittyBought {
				who: account_id,
				current_owner: account_id,
				kitty_id: kitty_id + 1,
			}
			.into(),
		);
	});
}
