use crate::{mock::*, Config, Error::TooShort, Event, MainStore};
use frame_support::{assert_noop, assert_ok, BoundedVec};
use sp_std::prelude::Vec;
use pallet_bro;

#[test]
fn in_service_test() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

        let bro_name: Vec<u8> = 
            "Skeebo".as_bytes().to_vec().try_into().unwrap();

        assert_ok!(pallet_bro::Pallet::<Test>::create_bro(RuntimeOrigin::signed(BAHA), bro_name.clone()));

        let id = 0x069420;
        let chute_name: Vec<u8> = 
            "8DC069".as_bytes().to_vec().try_into().unwrap();
        
        assert_eq!(MainStore::<Test>::contains_key(id.clone()), false);

        assert_noop!(
            Parachutes::in_service(RuntimeOrigin::signed(BAHA), id.clone(), chute_name.clone()),
            crate::Error::<Test>::NotARigger
            );

        assert_ok!(pallet_bro::Pallet::<Test>::set_rigger(RuntimeOrigin::signed(BAHA), true));
        assert_ok!(Parachutes::in_service(RuntimeOrigin::signed(BAHA), id.clone(), chute_name.clone()));
        
        assert_eq!(MainStore::<Test>::contains_key(id.clone()), true);
        assert_eq!(MainStore::<Test>::get(id.clone()).unwrap(), chute_name.clone());

        System::assert_last_event(
            Event::InService 
                { bro: BAHA , chute_name: chute_name.clone().try_into().expect("Error") }
                .into()
        );

        assert_noop!(
            Parachutes::in_service(RuntimeOrigin::signed(BAHA), id.clone(), chute_name.clone()), 
            crate::Error::<Test>::AlreadyInService
        );
	});
}

#[test]
fn rigged_test() {
	new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let bro_name: Vec<u8> =
            "Skeebo".as_bytes().to_vec().try_into().unwrap();

        assert_ok!(pallet_bro::Pallet::<Test>::create_bro(RuntimeOrigin::signed(BAHA), bro_name.clone()));
        assert_ok!(pallet_bro::Pallet::<Test>::set_rigger(RuntimeOrigin::signed(BAHA), true));

        let id = 0x069420;
        let chute_name: Vec<u8> =
            "8DC069".as_bytes().to_vec().try_into().unwrap();
        assert_ok!(Parachutes::in_service(RuntimeOrigin::signed(BAHA), id.clone(), chute_name.clone()));

        assert_ok!(Parachutes::rig(RuntimeOrigin::signed(BAHA), id.clone()));
        
        System::assert_last_event(
            Event::Rigged
            { bro: BAHA, chute_name: chute_name.clone().try_into().expect("Error") }
            .into()
        );

        let other_id = 0x42069;
        assert_noop!(Parachutes::rig(
            RuntimeOrigin::signed(BAHA), other_id), 
            crate::Error::<Test>::ParachuteDoesNotExist


        );
	});
}

#[test]
fn jumped_est() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let bro_name: Vec<u8> =
            "Skeebo".as_bytes().to_vec().try_into().unwrap();

        assert_ok!(pallet_bro::Pallet::<Test>::create_bro(RuntimeOrigin::signed(BAHA), bro_name.clone()));
        assert_ok!(pallet_bro::Pallet::<Test>::set_rigger(RuntimeOrigin::signed(BAHA), true));

        let id = 0x069420;
        let chute_name: Vec<u8> =
            "8DC069".as_bytes().to_vec().try_into().unwrap();
        assert_ok!(Parachutes::in_service(RuntimeOrigin::signed(BAHA), id.clone(), chute_name.clone()));

        assert_ok!(Parachutes::jump(RuntimeOrigin::signed(BAHA), id.clone()));

        System::assert_last_event(
            Event::Jumped
            { bro: BAHA, chute_name: chute_name.clone().try_into().expect("Error") }
            .into()
        );

        let other_id = 0x42069;

        assert_ok!(Parachutes::rig(RuntimeOrigin::signed(BAHA), id.clone()));
        assert_noop!(Parachutes::rig(
            RuntimeOrigin::signed(BAHA), other_id),
            crate::Error::<Test>::ParachuteDoesNotExist
        );
    });
}

#[test]
fn repaired_test() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let bro_name: Vec<u8> =
            "Skeebo".as_bytes().to_vec().try_into().unwrap();

        assert_ok!(pallet_bro::Pallet::<Test>::create_bro(RuntimeOrigin::signed(BAHA), bro_name.clone()));
        assert_ok!(pallet_bro::Pallet::<Test>::create_bro(RuntimeOrigin::signed(BUDD), bro_name.clone()));
        assert_ok!(pallet_bro::Pallet::<Test>::set_rigger(RuntimeOrigin::signed(BAHA), true));

        let id = 0x069420;
        let chute_name: Vec<u8> =
            "8DC069".as_bytes().to_vec().try_into().unwrap();
        assert_ok!(Parachutes::in_service(RuntimeOrigin::signed(BAHA), id.clone(), chute_name.clone()));

        assert_noop!(
            Parachutes::repair(RuntimeOrigin::signed(BUDD), id.clone()),
            crate::Error::<Test>::NotARigger
        );

        assert_ok!(Parachutes::repair(RuntimeOrigin::signed(BAHA), id.clone()));

        System::assert_last_event(
            Event::Repaired
            { bro: BAHA, chute_name: chute_name.clone().try_into().expect("Error") }
            .into()
        );

        let other_id = 0x42069;
        assert_noop!(Parachutes::rig(
            RuntimeOrigin::signed(BAHA), other_id),
            crate::Error::<Test>::ParachuteDoesNotExist


        );
    });
}

#[test]
fn retired_test() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let bro_name: Vec<u8> =
            "Skeebo".as_bytes().to_vec().try_into().unwrap();

        assert_ok!(pallet_bro::Pallet::<Test>::create_bro(RuntimeOrigin::signed(BAHA), bro_name.clone()));
        assert_ok!(pallet_bro::Pallet::<Test>::create_bro(RuntimeOrigin::signed(BUDD), bro_name.clone()));
        assert_ok!(pallet_bro::Pallet::<Test>::set_rigger(RuntimeOrigin::signed(BAHA), true));

        let id = 0x069420;
        let chute_name: Vec<u8> =
            "8DC069".as_bytes().to_vec().try_into().unwrap();

        assert_ok!(Parachutes::in_service(RuntimeOrigin::signed(BAHA), id.clone(), chute_name.clone()));

        assert_noop!(
            Parachutes::retire(RuntimeOrigin::signed(BUDD), id.clone()),
            crate::Error::<Test>::NotARigger
        );

        let other_id = 0x42069;
        assert_noop!(Parachutes::retire(
            RuntimeOrigin::signed(BAHA), other_id),
            crate::Error::<Test>::ParachuteDoesNotExist
        );

        assert_ok!(Parachutes::retire(RuntimeOrigin::signed(BAHA), id.clone()));

        System::assert_last_event(
            Event::ParachuteRetired
            { bro: BAHA, chute_name: chute_name.clone().try_into().expect("Error") }
            .into()
        );  
        
        assert_noop!(Parachutes::retire(
            RuntimeOrigin::signed(BAHA), id), 
            crate::Error::<Test>::ParachuteDoesNotExist
        
        
        );
    });
}
