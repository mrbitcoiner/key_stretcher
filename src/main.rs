use key_stretcher;
use std::{io, io::Write, env, process::exit};
#[allow(unused_imports)]
use log::{debug, info};

enum Difficulty {
    LOW,
    MID,
    HIGH,
    EXTREME
}

impl Difficulty {
    fn get_size(&self) -> usize {
        match self {
            Difficulty::LOW => 536870912,       // 512MB
            Difficulty::MID => 1073741824,      // 1GB
            Difficulty::HIGH => 2147483648,     // 2GB
            Difficulty::EXTREME => 4294967296   // 4GB
        }
    }
}

fn set_mode(input: &str) -> &str {
    match input {
        "low" => "low",
        "mid" => "mid",
        "high" => "high",
        "extreme" => "extreme",
        _=>{
            println!("Memory usage modes: [ low: 512MB | mid: 1GB | high: 2GB | extreme: 4GB ]");
            exit(1);
        }
    }
}

fn main() {
    key_stretcher::set_logger();
    let stretch_mode: &str;
    let mut key: String = String::new();
    #[allow(unused_assignments)]
    let mut stretched_key: String = String::new();

    let args: Vec<String> = env::args().collect();
    match env::args().len() {
        2 => {
            stretch_mode = set_mode(args[1].trim());
            print!("Insert the key: "); 
            let _ = io::stdout().flush(); 
            io::stdin().read_line(&mut key).expect("Failed to read input");
        }
        3 => {
            stretch_mode = set_mode(args[1].trim());
            key = args[2].trim().to_owned();
        }
        _ => {
            print!("Expected args:\n\t mandatory: <memory_usage_mode> optional: <key>\n");
            exit(1)
        }
    }

    info!("Mode: {}, key: {}", stretch_mode, key.trim());

    match stretch_mode {
        "low"=> {
            stretched_key = key_stretcher::stretcher(key.trim(), Difficulty::LOW.get_size());
        }
        "mid"=>{
            stretched_key = key_stretcher::stretcher(key.trim(), Difficulty::MID.get_size());
        }
        "high"=>{
           stretched_key = key_stretcher::stretcher(key.trim(), Difficulty::HIGH.get_size());
        }
        "extreme"=>{
            stretched_key = key_stretcher::stretcher(key.trim(), Difficulty::EXTREME.get_size());
        }
        _=>{
            println!("Error");
            exit(1);
        }
    }
    println!("key={}", stretched_key);
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn low_test(){
        let expected_hash = "
        20ce66449435872d8f60b74a94fc8d65c4782b0ee44872694783b835d06370a9
        db589de3dc6531858093a9fd5981526f36d2b96f07913ca17192ab275a209197"
        .replace("\n", "").replace(" ", "").to_owned();
        let key = "123".to_owned();
        let hash = key_stretcher::stretcher(&key, Difficulty::LOW.get_size());
        assert_eq!(expected_hash, hash);
    }
    #[test]
    fn mid_test(){
        let expected_hash = "
        2c52bcda18e6ee22411ae18193a6a46ebd2e9dc22d64df6b1ed0ecd2a0cf911b
        cd6145e80aea8d027fc1f6adf4ca81e2ee7df24d7a9f37588e71b3ade0bc2076"
        .replace("\n", "").replace(" ", "").to_owned();
        let key = "123".to_owned();
        let hash = key_stretcher::stretcher(&key, Difficulty::MID.get_size());
        assert_eq!(expected_hash, hash);
    }
    #[test]
    fn high_test(){
        let expected_hash = "
        f5ff7c2cc4397d5c391250ff89dde115e7e4886b155e6dff5305c3aed565601d
        6d045498cfb284cef19d2329dcc0a4a9df5d393e12ea646c16037c769dbfb20e"
        .replace("\n", "").replace(" ", "").to_owned();
        let key = "123".to_owned();
        let hash = key_stretcher::stretcher(&key, Difficulty::HIGH.get_size());
        assert_eq!(expected_hash, hash);
    }
    #[test]
    fn extreme_test(){
        let expected_hash = "
        a77838ddc448fcef151302c4bda1391fdd47f3b2b8899dbd8c8078385c9f984f
        800dac1af09a381a9b9a1ed073625839a2e7135a120cc6ace95020d143ba7a05"
        .replace("\n", "").replace(" ", "").to_owned();
        let key = "123".to_owned();
        let hash = key_stretcher::stretcher(&key, Difficulty::EXTREME.get_size());
        assert_eq!(expected_hash, hash);
    }
}