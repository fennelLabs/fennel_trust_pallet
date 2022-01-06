use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn issue_trust() {
	new_test_ext().execute_with(|| {
		assert_ok!(TrustModule::issue_trust(Origin::signed(1), Origin::signed(1)));
		assert_eq!(TrustModule::get_current_trust_count(), Some(1));
	});
}

#[test]
fn issue_trust_once() {
	new_test_ext().execute_with(|| {
		assert_ok!(TrustModule::issue_trust(Origin::signed(1), Origin::signed(1)));
		assert_eq!(TrustModule::get_current_trust_count(), Some(1));
		
		assert_ok!(TrustModule::issue_trust(Origin::signed(1), Origin::signed(1)));
		assert_eq!(TrustModule::get_current_trust_count(), Some(1));
	});
}

#[test]
fn remove_trust() {
	new_test_ext().execute_with(|| {
		assert_ok!(TrustModule::issue_trust(Origin::signed(1), Origin::signed(1)));
		assert_eq!(TrustModule::get_current_trust_count(), Some(1));
		
		assert_ok!(TrustModule::remove_trust(Origin::signed(1), Origin::signed(1)));
		assert_eq!(TrustModule::get_current_trust_count(), Some(0));
	});
}

#[test]
fn remove_trust_no_failure() {
	new_test_ext().execute_with(|| {
		assert_ok!(TrustModule::issue_trust(Origin::signed(1), Origin::signed(1)));
		assert_eq!(TrustModule::get_current_trust_count(), Some(1));
		
		assert_ok!(TrustModule::remove_trust(Origin::signed(1), Origin::signed(1)));
		assert_eq!(TrustModule::get_current_trust_count(), Some(0));

		assert_ok!(TrustModule::remove_trust(Origin::signed(1), Origin::signed(1)));
	});
}

#[test]
fn revoke_trust() {
	new_test_ext().execute_with(|| {
		assert_ok!(TrustModule::revoke_trust(Origin::signed(1), Origin::signed(1)));
		assert_eq!(TrustModule::get_current_non_trust_count(), Some(1));
	});
}

#[test]
fn revoke_trust_once() {
	new_test_ext().execute_with(|| {
		assert_ok!(TrustModule::revoke_trust(Origin::signed(1), Origin::signed(1)));
		assert_eq!(TrustModule::get_current_non_trust_count(), Some(1));
		
		assert_ok!(TrustModule::revoke_trust(Origin::signed(1), Origin::signed(1)));
		assert_eq!(TrustModule::get_current_non_trust_count(), Some(1));
	});
}

#[test]
fn remove_revoked_trust() {
	new_test_ext().execute_with(|| {
		assert_ok!(TrustModule::revoke_trust(Origin::signed(1), Origin::signed(1)));
		assert_eq!(TrustModule::get_current_non_trust_count(), Some(1));

		assert_ok!(TrustModule::remove_revoked_trust(Origin::signed(1), Origin::signed(1)));
		assert_eq!(TrustModule::get_current_non_trust_count(), Some(0));
	});
}

#[test]
fn remove_revoked_trust_no_failure() {
	new_test_ext().execute_with(|| {
		assert_ok!(TrustModule::revoke_trust(Origin::signed(1), Origin::signed(1)));
		assert_eq!(TrustModule::get_current_non_trust_count(), Some(1));

		assert_ok!(TrustModule::remove_revoked_trust(Origin::signed(1), Origin::signed(1)));
		assert_eq!(TrustModule::get_current_non_trust_count(), Some(0));

		assert_ok!(TrustModule::remove_revoked_trust(Origin::signed(1), Origin::signed(1)));
	});
}