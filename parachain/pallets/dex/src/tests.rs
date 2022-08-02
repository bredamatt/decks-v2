use crate::mock::*;
use crate::LiquidityPool;
use frame_support::assert_ok;

const ADMIN: u64 = 1;
const NATIVE_TOKEN: u32 = 0;
const TOKEN_0: u32 = 4;
const TOKEN_1: u32 = 6;
const LP: u64 = 123;
const MIN_BALANCE: u128 = 1;

#[test]
fn new_liquidity_pool() {
    new_test_ext().execute_with(|| {
        assert_ok!(<LiquidityPool<Test>>::new_liquidity_pool((TOKEN_0, TOKEN_1)));
    });
}
