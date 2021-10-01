use std::convert::TryFrom;

use codec::{Decode, Encode};
use frame_support::{sp_runtime::traits::BlakeTwo256, StorageHasher, Twox128};
use frame_system::{Account, AccountInfo};
use hex::FromHex;
use hex_literal::hex;
use sp_runtime::{
    traits::{IdentifyAccount, Verify},
    MultiSignature,
};

/// Alias to 512-bit hash when used in the context of a transaction signature on
/// the chain.
pub type Signature = MultiSignature;

/// Alias to the public key used for this chain, actually a `MultiSigner`. Like
/// the signature, this also isn't a fixed size when encoded, as different
/// cryptos have different size public keys.
pub type AccountPublic = <Signature as Verify>::Signer;

/// Alias to the opaque account ID type for this chain, actually a
/// `AccountId32`. This is always 32 bytes.
pub type AccountId = <AccountPublic as IdentifyAccount>::AccountId;

type SessionKey = [u8; 32];
type QueuedKeys = Vec<(AccountId, SessionKey)>;

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

#[derive(Copy, Clone, Encode, Eq, Decode, Debug, Ord, PartialEq, PartialOrd)]
pub enum StakingStorageVersion {
	V1_0_0,
	V2_0_0, // New Reward calculation, MaxCollatorCandidateStake
	V3_0_0, // Update InflationConfig
	V4,     // Sort TopCandidates and parachain-stakings by amount
	V5,     // Remove SelectedCandidates, Count Candidates
}

fn main() {
    let mut keys: Vec<Vec<u8>> = vec![
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
        b"Did".to_owned().to_vec(),
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
        b"SessionQueuedKeys".to_owned().to_vec(),
        b"StorageVersion".to_owned().to_vec(),
    ];
    keys.sort();

    for key in keys {
        println!(
            "{:?} => {:?}",
            String::from_utf8(key.clone()).unwrap_or(hex::encode(&key[..])),
            hex::encode(Twox128::hash(&key[..]))
        );
    }

    let mut version: StakingStorageVersion = StakingStorageVersion::V4;
    println!("version: {:?}", hex::encode(StakingStorageVersion::encode(&version)));
}
