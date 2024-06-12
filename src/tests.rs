use std::alloc::System;

use crate::{mock::*,Error,Event, NameToId};
use frame_support::{assert_noop,assert_ok};
use frame_system::{pallet, Origin};


#[test]
fn setname(){

    new_test_ext().execute_with(||{
        System::set_block_number(1);
        assert_ok!(Ronnie::set_Id(RuntimeOrigin::signed(1), 42));
    });



    // System::set_block_number(1);
    // assert_ok!(Ronnie::set_Id(Origin::Signed(1), 42));
    // assert_eq!(NameToId::<Test>::get(1),Some(42));
    // System::assert_last_event(Event::Ronnie::(pallet::Event::IdSet(1,42)))
}