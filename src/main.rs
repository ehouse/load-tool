extern crate crypto;
extern crate rand;

extern crate num_cpus;

use rand::Rng;
use crypto::sha2::Sha256;
use crypto::digest::Digest;

fn hash(val: &str) -> String{
    let mut hasher = Sha256::new();
    hasher.input_str(&val);
    hasher.result_str().to_string()
}

fn verify_hash(val: &str, hashed_val: &str) -> bool{
    if hash(val) == hashed_val {
        return true;
    }
    return false;
}

fn main() {

    // count logical cores this process could try to use
    // let num = num_cpus::get();
    // println!("There are {} Logical CPU's", num);

    let size_of_heap = 64;

    let mut rng = rand::thread_rng();

    let mut val_heap: Vec<String> = Vec::new();
    let mut hash_heap: Vec<String> = Vec::new();

    let mut random_val: u64;

    for _ in 0 .. size_of_heap+1{
        random_val = rng.gen();
        val_heap.push(random_val.to_string());
    }

    for x in 0 .. size_of_heap+1{
        hash_heap.push(hash(&val_heap[x]));
    }

    println!("{} , {}", val_heap[0], val_heap[1]);
    println!("{} , {}", hash_heap[0], hash_heap[1]);
    println!("Hashes match: {}", verify_hash(&val_heap[0], &hash_heap[0]));
}

