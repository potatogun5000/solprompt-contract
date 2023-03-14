use anchor_lang::prelude::*;
use std::str::FromStr;

pub fn get_pubkey(address: &str) -> Pubkey {
    Pubkey::from_str(address).unwrap()
}


