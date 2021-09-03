use std::convert::TryFrom;

use frame_support::{StorageHasher, Twox128, sp_runtime::traits::BlakeTwo256};
use frame_system::{Account, AccountInfo};
use hex::FromHex;
use hex_literal::hex;
use codec::{Encode, Decode};

fn storage_value_key(pallet_name: &[u8], storage_name: &[u8]) -> [u8; 32] {
    let mut final_key = [0u8; 32];
    final_key[0..16].copy_from_slice(&Twox128::hash(pallet_name));
    final_key[16..32].copy_from_slice(&Twox128::hash(storage_name));
    final_key
}

#[derive(Debug, PartialEq, Eq, Clone, Encode, Decode)]
struct BlockAnnouncesHandshake {
	/// Roles of the node.
	roles: u8,
	/// Best block number.
	best_number: u32,
	/// Best block hash.
	best_hash: sp_core::H256,
	/// Genesis block hash.
	genesis_hash: sp_core::H256,
}

fn main() {
    let keys: Vec<Vec<u8>> = vec![
        b"Block".to_owned().to_vec(),
        b"BlockHash".to_owned().to_vec(),
        b"ExtrinsicData".to_owned().to_vec(),
        b"AllExtrinsicsLen".to_owned().to_vec(),
        b"NodeBlock".to_owned().to_vec(),
        b"UncheckedExtrinsic".to_owned().to_vec(),
        b"System".to_owned().to_vec(),
        b"RandomnessCollectiveFlip".to_owned().to_vec(),
        b"Timestamp".to_owned().to_vec(),
        b"Indices".to_owned().to_vec(),
        b"Balances".to_owned().to_vec(),
        b"TransactionPayment".to_owned().to_vec(),
        b"Sudo".to_owned().to_vec(),
        b"ParachainStaking".to_owned().to_vec(),
        b"Session".to_owned().to_vec(),
        b"Authorship".to_owned().to_vec(),
        b"Aura".to_owned().to_vec(),
        b"AuraExt".to_owned().to_vec(),
        b"ParachainSystem".to_owned().to_vec(),
        b"ParachainInfo".to_owned().to_vec(),
        b"Vesting".to_owned().to_vec(),
        b"KiltLaunch".to_owned().to_vec(),
        b"Utility".to_owned().to_vec(),
        b"Account".to_owned().to_vec(),
        b"Number".to_owned().to_vec(),
        b"CurrentSlot".to_owned().to_vec(),
        b"Authorities".to_owned().to_vec(),
        b"Round".to_owned().to_vec(),
        b"CollatorState".to_owned().to_vec(),
        b"SelectedCandidates".to_owned().to_vec(),
        b"Total".to_owned().to_vec(),
        b"CandidatePool".to_owned().to_vec(),
        b"InflationConfig".to_owned().to_vec(),
        b"LastRewardReduction".to_owned().to_vec(),
        b"TransferAccount".to_owned().to_vec(),
        b"UnlockingAt".to_owned().to_vec(),
        b"BalanceLocks".to_owned().to_vec(),
        b"UnownedAccount".to_owned().to_vec(),
        b"ParentHash".to_owned().to_vec(),
        b"Digest".to_owned().to_vec(),
        b"Events".to_owned().to_vec(),
        b"EventCount".to_owned().to_vec(),
        b"EventTopics".to_owned().to_vec(),
        b"BlockWeight".to_owned().to_vec(),
        b"ExtrinsicCount".to_owned().to_vec(),
        b"Author".to_owned().to_vec(),
    ];
    for key in keys {
        println!(
            "{:?} => {:?}",
            String::from_utf8(key.clone()).unwrap_or(hex::encode(&key[..])),
            hex::encode(Twox128::hash(&key[..]))
        );
    }
    let mut encoded1 = Vec::<u8>::from_hex(b"060000000000000002000000010000000000000014187132b48694cf4ace4f200000000000000000000000000000000000000000000020c65abc8ed70a00000000000000000020c65abc8ed70a00000000000000").unwrap();
    let account1 =
        AccountInfo::<u64, pallet_balances::AccountData<u128>>::decode(&mut &encoded1[..]);
    println!("Collator read: {:?}", account1);

    let mut encoded1 = Vec::<u8>::from_hex(b"06000000000000000200000001000000000000001468288e5ff594cf4ace4f200000000000000000000000000000000000000000000020c65abc8ed70a00000000000000000020c65abc8ed70a00000000000000").unwrap();
    let account1 =
        AccountInfo::<u64, pallet_balances::AccountData<u128>>::decode(&mut &encoded1[..]);
    println!("Collator write: {:?}", account1);

    let mut encoded2 = Vec::<u8>::from_hex(b"06000000000000000200000001000000000000001468288e5ff594cf4ace4f200000000000000000000000000000000000000000000020c65abc8ed70a00000000000000000020c65abc8ed70a00000000000000").unwrap();
    let account2 =
        AccountInfo::<u64, pallet_balances::AccountData<u128>>::decode(&mut &encoded2[..]);
    println!("Account after 172042: {:?}", account2);

    let mut encoded2 = Vec::<u8>::from_hex(b"06000000000000000200000001000000000000001468288e5ff594cf4ace4f200000000000000000000000000000000000000000000020c65abc8ed70a00000000000000000020c65abc8ed70a00000000000000").unwrap();
    let account2 =
        AccountInfo::<u64, pallet_balances::AccountData<u128>>::decode(&mut &encoded2[..]);
    println!("Account at 172052: {:?}", account2);

    let mut encoded2 = Vec::<u8>::from_hex(b"06000000000000000200000001000000000000001468288e5ff594cf4ace4f200000000000000000000000000000000000000000000020c65abc8ed70a00000000000000000020c65abc8ed70a00000000000000").unwrap();
    let account2 =
        AccountInfo::<u64, pallet_balances::AccountData<u128>>::decode(&mut &encoded2[..]);
    println!("Account at 172053: {:?}", account2);

    let mut encoded2 = Vec::<u8>::from_hex(b"060000000000000002000000010000000000000014b8dfe90a6495cf4ace4f200000000000000000000000000000000000000000000020c65abc8ed70a00000000000000000020c65abc8ed70a00000000000000").unwrap();
    let account2 =
        AccountInfo::<u64, pallet_balances::AccountData<u128>>::decode(&mut &encoded2[..]);
    println!("Account at 172054: {:?}", account2);

    let mut encoded3 = Vec::<u8>::from_hex(b"16a0020000000000").unwrap();
    let blocknum = u64::decode(&mut &encoded3[..]);
    println!("blocknumber: {:?}", blocknum);

    let mut encoded4 = Vec::<u8>::from_hex(b"08e29df39b74777495ca00cd7a316ce98c5225d7088ae924b122fe0e2e6a4b556934fe8b3e78e63ca6139669cf9fc4694e5ae7c3a2f8e9fbcacf855b4716147429").unwrap();
    let authorities = u64::decode(&mut &encoded4[..]);
    println!("authorities: {:?}", authorities);

    let ba = BlockAnnouncesHandshake {
        roles: 4,
        best_number: 0,
        best_hash: sp_core::H256::from_low_u64_be(0),
        genesis_hash: sp_core::H256::from_low_u64_be(0),
    };
    let mut encode = ba.encode();
    println!("BlockAnnouncesHandshake: {:?}", encode);

    let mut encoded4 = [4, 3, 181, 42, 0, 127, 14, 119, 47, 121, 138, 119, 120, 20, 141, 4, 199, 123, 120, 239, 84, 106, 12, 7, 167, 46, 58, 246, 238, 222, 6, 117, 88, 2, 223, 93, 161, 255, 246, 253, 148, 37, 31, 87, 13, 76, 156, 223, 37, 160, 71, 93, 160, 215, 173, 53, 22, 2, 144, 218, 25, 218, 216, 249, 202, 248, 191, 49, 181];
    let authorities = BlockAnnouncesHandshake::decode(&mut &encoded4[..]);
    println!("BlockAnnouncesHandshake: {:?}", authorities);
}
