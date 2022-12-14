use crate as pallet_dex;
use frame_support::{parameter_types, PalletId};

use frame_support::traits::{ConstU16, ConstU32, ConstU64, ConstU128};
use frame_system as system;
use frame_system::EnsureRoot;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
	BuildStorage,
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		Assets: pallet_assets::{Pallet, Call, Storage, Config<T>, Event<T>},
		Dex: pallet_dex::{Pallet, Call, Storage, Event<T>},
	}
);

pub type AssetId = u32;
pub type AccountId = u128;
pub type Balance = u128;

impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u128;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_assets::Config for Test {
	type Event = Event;
	type Balance = Balance;
	type AssetId = AssetId;
	type Currency = Balances;
	type ForceOrigin = EnsureRoot<AccountId>;
	type AssetDeposit = ();
	type AssetAccountDeposit = ();
	type MetadataDepositBase = ();
	type MetadataDepositPerByte = ();
	type ApprovalDeposit = ();
	type StringLimit = ConstU32<15>;
	type Freezer = ();
	type Extra = ();
	type WeightInfo = ();
}

impl pallet_balances::Config for Test {
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type Balance = Balance;
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = ConstU128<1>;
	type AccountStore = System;
	type WeightInfo = ();
}

parameter_types! {
	pub const Decks: PalletId = PalletId(*b"dotdecks");
	pub const TokenMinimumBalance: u32 = 1; // Must be greater than 0 (existential deposit)
	pub const TokenDecimals: u8 = 12;
}

impl pallet_dex::Config for Test {
	type Event = Event;
	type AssetId = AssetId;
	type Tokens = Assets;
	type PalletId = Decks;
	type LpTokenMinimumBalance = TokenMinimumBalance;
	type LpTokenDecimals = TokenDecimals;
	type NativeCurrency = Balances;
	type NativeTokenId = ();
	
	fn exists(id: Self::AssetId) -> bool {
		Assets::maybe_total_supply(id).is_some()
	}
}

// Build genesis storage according to the mock runtime.
pub(crate) fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
	GenesisConfig {
		balances: BalancesConfig { 
			balances: vec![
				(1, 10_000_000)
			],
		},
		..Default::default()

		// Want to ideally map the AssetConfig here to the one in node/chain_spec.rs
		// and thereafter simulate state transitions with my tests, but didn't manage

		// assets: AssetConfig {
		// 	assets: vec![
		// 		(TOKEN_0,),
		// 		(TOKEN_1,),
		// 		()
		// 	],
		// 	balances: vec! [

		// 	],
		// 	metadata: vec![],
		// }

	}
	.assimilate_storage(&mut t)
	.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}
