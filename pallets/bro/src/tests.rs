use crate::{mock::*, BroStore, Error, Event};
use frame_support::{assert_noop, assert_ok, BoundedVec,};

#[test]
fn create_bro() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

        let name: Vec<u8> = "Skeebo".as_bytes().to_vec().try_into().unwrap();

        let bounded_name: BoundedVec<u8, <Test as crate::pallet::Config>::MaxLength> =
            BoundedVec::try_from(name.clone()).unwrap();

		assert_eq!(BroStore::<Test>::contains_key(1), false);
		assert_ok!(Bro::create_bro(RuntimeOrigin::signed(1), name.clone()));
		assert_eq!(BroStore::<Test>::contains_key(1), true);
		System::assert_last_event(Event::BroCreated { bro: 1 }.into());

		assert_eq!(BroStore::<Test>::get(1).unwrap(), (bounded_name.clone(), false));


		assert_noop!(
			Bro::create_bro(RuntimeOrigin::signed(1), name),
			Error::<Test>::BroAlreadyCreated
		);
	});
}

#[test]
fn set_rigger() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let bro_name: Vec<u8> =
            "Skeebo".as_bytes().to_vec().try_into().unwrap();

        assert_ok!(Bro::create_bro(RuntimeOrigin::signed(1), bro_name.clone()));
        let ( _bounded_name, rigger ) = BroStore::<Test>::get(1).unwrap();
        assert_eq!(rigger, false);
        
        assert_ok!(Bro::set_rigger(RuntimeOrigin::signed(1), true));
        let ( _bounded_name, rigger ) = BroStore::<Test>::get(1).unwrap();
        assert_eq!(rigger, true);
        
        System::assert_last_event(Event::RiggerSet { bro: 1, rigger: true }.into());

        assert_noop!(Bro::set_rigger(RuntimeOrigin::signed(1), true), Error::<Test>::AlreadyARigger);
    });
}
