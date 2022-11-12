/* 
    Rust Key Stretcher
    By Mr. Bitcoiner
*/

use hex::encode;
use sha2::{Sha512, Digest};
use std::{io, env, process::exit};
use log::{debug, info};
use env_logger;

fn do_hash(input: impl AsRef<[u8]>) -> [u8; 64] {
    let mut hasher = Sha512::new();
    hasher.update(&input);
    let hash = hasher.finalize();
    let mut hash_array: [u8; 64] = [0; 64];
    hash_array.copy_from_slice(&hash);
    return hash_array;
}

fn stretcher(input: &str, max_array_bytes: usize) {

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
    print!("stretched_key={}", encode(&final_hash).as_str());
}

fn set_mode(input: String) -> String {
    match input.trim() {
        "low"=> {
            return String::from("low");
        }
        "mid"=>{
            return String::from("mid");
        }
        "high"=>{
            return String::from("high");
        }
        "extreme"=>{
            return String::from("extreme");
        }
        _=>{
            print!("Memory usage modes: [ low: 512MB | mid: 1GB | high: 2GB | extreme: 4GB ]\n");
            exit(1);
        }
    }
}

fn set_logger() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "error");
    }
    env_logger::init();
}

fn main() {
    set_logger();

    let low:usize =  536870912; // 512MB
    let mid:usize = 1073741824; // 1GB
    let high:usize = 2147483648; // 2GB
    let extreme:usize = 4294967296; // 4GB

    let mut mode:String = String::new();
    let mut key:String = String::new();

    let args: Vec<String> = env::args().collect();
    match env::args().len() {
        2 => {
            mode = set_mode(args[1].to_string());
            print!("\nInsert the key: \n");
            io::stdin().read_line(&mut key).expect("Failed to read input");
        }
        3 => {
            mode = set_mode(args[1].to_string());
            key = args[2].to_string();
        }
        _ => {
            print!("Expected args:\n\t mandatory: <memory_usage_mode> optional: <key>\n");
            if mode.is_empty() { return; }
        }
    }

    info!("Mode: {}, key: {}", mode.as_str(), key.trim());

    match mode.as_str() {
        "low"=> {
            stretcher(key.trim(), low);
        }
        "mid"=>{
            stretcher(key.trim(), mid);
        }
        "high"=>{
            stretcher(key.trim(), high);
        }
        "extreme"=>{
            stretcher(key.trim(), extreme);
        }
        _=>{
            exit(1);
        }
    }
}
