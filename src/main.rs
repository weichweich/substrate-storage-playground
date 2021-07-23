use std::convert::TryFrom;

use frame_support::{dispatch::Decode, StorageHasher, Twox128};
use frame_system::{Account, AccountInfo};
use hex::FromHex;

fn storage_value_key(pallet_name: &[u8], storage_name: &[u8]) -> [u8; 32] {
    let mut final_key = [0u8; 32];
    final_key[0..16].copy_from_slice(&Twox128::hash(pallet_name));
    final_key[16..32].copy_from_slice(&Twox128::hash(storage_name));
    final_key
}

fn main() {
    let keys: Vec<Vec<u8>> = vec![
        b"Block".to_owned().to_vec(),
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
    ];
    for key in keys {
        println!(
            "{:?} => {:?}",
            String::from_utf8(key.clone()),
            hex::encode(Twox128::hash(&key[..]))
        );
    }
    let mut encoded1 = Vec::<u8>::from_hex(b"0100000000000000020000000100000000000000e78d7b42827c332b0200000000000000000000000000000000000000000000000000b89d0d6955a001000000000000000000b89d0d6955a00100000000000000").unwrap();
    let account1 = AccountInfo::<u64, pallet_balances::AccountData<u128>>::decode(&mut &encoded1[..]);
    println!("{:?}", account1);

    let mut encoded2 = Vec::<u8>::from_hex(b"0100000000000000020000000100000000000000e7a085afa880332b0200000000000000000000000000000000000000000000000000b89d0d6955a001000000000000000000b89d0d6955a00100000000000000").unwrap();
    let account2 = AccountInfo::<u64, pallet_balances::AccountData<u128>>::decode(&mut &encoded2[..]);
    println!("{:?}", account2);
}
