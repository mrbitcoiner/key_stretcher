use hex::encode;
use sha2::{Sha512, Digest};
use std::{io, env, process::exit};
use log::{debug, info};
use env_logger;

pub fn set_logger() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "error");
    }
    env_logger::init();
}

fn do_hash(input: impl AsRef<[u8]>) -> [u8; 64] {
    let mut hasher
     = Sha512::new();
    hasher.update(&input);
    let hash = hasher.finalize();
    let mut hash_array: [u8; 64] = [0; 64];
    hash_array.copy_from_slice(&hash);
    return hash_array;
}

pub fn stretcher(input: &str, max_array_bytes: usize) -> String {

    let input_hash: [u8; 64] = do_hash(&input);
    let mut acumulator: Vec<u8> = Vec::with_capacity(max_array_bytes);

    /* Append first key hash to the acumulator */
    for i in 0..input_hash.len(){
        acumulator.push(input_hash[i]);
    }

    let mut acumulator_length:usize = 0;
    let mut acumulator_round:usize = 0;
    while acumulator_length < max_array_bytes {
        /* Append the hash of acumulator to the acumulator */
        let new_round: [u8; 64] = do_hash(&acumulator);
        for i in 0..new_round.len() {
            acumulator.push(new_round[i]);
        }

        /* Increase the acumulator length */
        let actual_acumulator_length:usize = acumulator.len();
        for i in 0..actual_acumulator_length{
            if acumulator.len() < max_array_bytes{
                acumulator.push(acumulator[i]);
            }
            else {
                break;
            }
        }

        acumulator_round += 1;
        acumulator_length = acumulator.len();
        debug!("Key stretch round {}, acumulator size: {}MB.", acumulator_round, acumulator.len()/1024/1024);
    }
    let mut final_hash: [u8; 64] = do_hash(&acumulator);
    for i in 0..100{
        final_hash = do_hash(&final_hash);
        debug!("Hash Round: {}, hash: {}",i+1, encode(&final_hash));
    }
    encode(&final_hash).to_owned()
}