extern crate crypto;
extern crate rand;

extern crate num_cpus;

use rand::Rng;
use crypto::sha2::Sha256;
use crypto::digest::Digest;

const SIZE_OF_HEAP: usize = 4096;

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

fn verify_vec_hash(vec: &[String], hashed_vec: &[String]) -> Result<i32, String> {
    for x in 0 .. SIZE_OF_HEAP+1{
        if hash(&vec[x]) == hashed_vec[x] {
            return Ok(0);
        }
        return Err("Vector mismatch!".to_string());
    }
    return Err("Improper vector size".to_string());
}

fn main() {

    // count logical cores this process could try to use
    // let num = num_cpus::get();
    // println!("There are {} Logical CPU's", num);

    let mut rng = rand::thread_rng();

    let mut val_heap: Vec<String> = Vec::with_capacity(SIZE_OF_HEAP);
    let mut hash_heap: Vec<String> = Vec::with_capacity(SIZE_OF_HEAP);

    let mut random_val: u64;

    for _ in 0 .. SIZE_OF_HEAP+1{
        random_val = rng.gen();
        val_heap.push(random_val.to_string());
    }

    for x in 0 .. SIZE_OF_HEAP+1{
        hash_heap.push(hash(&val_heap[x]));
    }

    let status = verify_vec_hash(&val_heap, &hash_heap);
    match status{
        Ok(_)  => println!("Vectors Match!"),
        Err(e) => panic!("ERROR: {}",e),
    }
}

