
use tiny_keccak::{Keccak, Hasher};
use sha2::{Sha256, Digest};
use primitive_types::H256;

const keccak_trie_root : &str = "0xbc36789e7a1e281436464229828f817d6612f7b477d66591ff96a9e064bcc98a";
const blake2_trie_root : &str = "0x03170a2e7597b7b7e3d84c05391d139a62b157e78786d8c082f29dcf4c111314";

fn main() {
    println!("Hello, world! {}, {}", keccak_trie_root, blake2_trie_root);
}

/// Do a Blake2 256-bit hash and place result in `dest`.
pub fn blake2_256_into(data: &[u8], dest: &mut [u8; 32]) {
    dest.copy_from_slice(blake2_rfc::blake2b::blake2b(32, &[], data).as_bytes());
}

/// Do a Blake2 256-bit hash and return result.
pub fn blake2_256(data: &[u8]) -> [u8; 32] {
    let mut r = [0; 32];
    blake2_256_into(data, &mut r);
    r
}

/// Do a keccak 256-bit hash and return result.
pub fn keccak_256(data: &[u8]) -> [u8; 32] {
    let mut keccak = Keccak::v256();
    keccak.update(data);
    let mut output = [0u8; 32];
    keccak.finalize(&mut output);
    output
}

/// Do a sha2 256-bit hash and return result.
pub fn sha2_256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.input(data);
    let mut output = [0u8; 32];
    output.copy_from_slice(&hasher.result());
    output
}

#[test]
fn test_hasher() {
    let foo = blake2_256(vec![0].as_slice());
    let bar : H256 = foo.into();
    let zee = format!("0x{:x}", bar);
    assert_eq!(zee, blake2_trie_root);
    println!("{:?}", foo);
    println!("{:?}, {:?}", H256::from(foo), H256(foo));
    println!("{:?}, {:?}\n {}, {}", bar, zee, bar, zee);

    let foo = keccak_256(vec![0].as_slice());
    let bar : H256 = foo.into();
    let zee = format!("0x{:x}", bar);
    assert_eq!(zee, keccak_trie_root);
    println!("{:?}, {}", H256::from(foo), zee);
}

