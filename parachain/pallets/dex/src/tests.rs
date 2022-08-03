use super::*;
use crate::mock::*;
use crate::LiquidityPool;
use frame_support::assert_ok;

const ADMIN: u128 = 1; // root account
const TOKEN_0: u32 = 1; // The first token AssetId
const TOKEN_1: u32 = 2; // The second token AssetId

// Simple test to make sure I can create liquidity pools
#[test]
fn new_liquidity_pool() {
    new_test_ext().execute_with(|| {
        assert_ok!(<LiquidityPool<Test>>::new_liquidity_pool((TOKEN_0, TOKEN_1)));
    });
}

// Need to fix this test by making sure that the GenesisConfig bootstraps with correct balances for each asset so the ADMIN account
// #[test]
// fn adds_liquidity() {
//     new_test_ext().execute_with(|| {
//         assert_ok!(Assets::force_create(Origin::root(), TOKEN_0, ADMIN, true, 250u128)); 
//         assert_ok!(Assets::force_create(Origin::root(), TOKEN_1, ADMIN, true, 250u128));
//         let pool = <LiquidityPool<Test>>::new_liquidity_pool((TOKEN_0, TOKEN_1)).unwrap();
//         assert_ok!(pool.add_liquidity((100u128, 100u128), &ADMIN));
//     });
// }
