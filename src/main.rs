use key_stretcher;
#[allow(unused_imports)]
use log::{debug, info};
use std::{env, io, io::Write};

#[derive(Debug)]
#[repr(usize)]
enum Difficulty {
    LOW = 536870912,
    MID = 1073741824,
    HIGH = 2147483648,
    EXTREME = 4294967296,
}

impl TryFrom<&str> for Difficulty {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "low" => Ok(Difficulty::LOW),
            "mid" => Ok(Difficulty::MID),
            "high" => Ok(Difficulty::HIGH),
            "extreme" => Ok(Difficulty::EXTREME),
            _ => Err(Self::Error::InvalidDifficulty(
                "Memory usage modes: [ low: 512MB | mid: 1GB | high: 2GB | extreme: 4GB ]"
                    .to_owned(),
            )),
        }
    }
}

#[derive(Debug)]
enum Error {
    InvalidDifficulty(String),
    InvalidArguments(String),
    IoError(String),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value.to_string())
    }
}

fn parse_input(args: Vec<String>) -> Result<(Difficulty, String), Error> {
    match args.len() {
        2 => {
            let difficulty = Difficulty::try_from(args[1].trim())?;
            let mut key = String::with_capacity(256);
            print!("Insert the key: ");
            io::stdout().flush()?;
            io::stdin().read_line(&mut key)?;
            Ok((difficulty, key.trim().to_owned()))
        }
        3 => {
            let difficulty = Difficulty::try_from(args[1].trim())?;
            let key = args[2].trim().to_owned();
            Ok((difficulty, key.trim().to_owned()))
        }
        _ => Err(Error::InvalidArguments(
            "Mandatory argument: [difficulty] optional argument: [key]".to_owned(),
        )),
    }
}

fn main() -> Result<(), Error> {
    key_stretcher::set_logger();
    let args = env::args().collect();
    let (difficulty, key) = parse_input(args)?;
    info!("Difficulty: {:?}, key: {:?}", difficulty, key);
    let stretched_key = key_stretcher::stretcher(&key, difficulty as usize);
    println!("key={}", stretched_key);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const KEY: &str = "123";

    #[test]
    fn low_test() {
        let expected_hash = "
        20ce66449435872d8f60b74a94fc8d65c4782b0ee44872694783b835d06370a9
        db589de3dc6531858093a9fd5981526f36d2b96f07913ca17192ab275a209197"
            .replace("\n", "")
            .replace(" ", "")
            .to_owned();

        let hash = key_stretcher::stretcher(&KEY, Difficulty::LOW as usize);

        assert_eq!(expected_hash, hash);
    }

    #[test]
    fn mid_test() {
        let expected_hash = "
        2c52bcda18e6ee22411ae18193a6a46ebd2e9dc22d64df6b1ed0ecd2a0cf911b
        cd6145e80aea8d027fc1f6adf4ca81e2ee7df24d7a9f37588e71b3ade0bc2076"
            .replace("\n", "")
            .replace(" ", "")
            .to_owned();

        let hash = key_stretcher::stretcher(&KEY, Difficulty::MID as usize);

        assert_eq!(expected_hash, hash);
    }

    #[test]
    fn high_test() {
        let expected_hash = "
        f5ff7c2cc4397d5c391250ff89dde115e7e4886b155e6dff5305c3aed565601d
        6d045498cfb284cef19d2329dcc0a4a9df5d393e12ea646c16037c769dbfb20e"
            .replace("\n", "")
            .replace(" ", "")
            .to_owned();

        let hash = key_stretcher::stretcher(&KEY, Difficulty::HIGH as usize);

        assert_eq!(expected_hash, hash);
    }

    #[test]
    fn extreme_test() {
        let expected_hash = "
        a77838ddc448fcef151302c4bda1391fdd47f3b2b8899dbd8c8078385c9f984f
        800dac1af09a381a9b9a1ed073625839a2e7135a120cc6ace95020d143ba7a05"
            .replace("\n", "")
            .replace(" ", "")
            .to_owned();

        let hash = key_stretcher::stretcher(&KEY, Difficulty::EXTREME as usize);

        assert_eq!(expected_hash, hash);
    }
}
