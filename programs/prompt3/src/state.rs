use anchor_lang::prelude::*;

#[account]
pub struct State {
    pub listings: u64,
    pub sales: u64,
    pub sellers: u64,
    pub buyers: u64,
    pub balance: u64,
    pub bump: u8,
}

#[account]
pub struct Seller {
    pub seller: Pubkey,
    pub sales: u64,
    pub listings: u64,
    pub balance: u64,
    pub bump: u8,
}

#[account]
pub struct Buyer {
    pub buyer: Pubkey,
    pub purchases: u64,
    pub bump: u8,
}

#[account]
pub struct Listing {
    pub approved: bool,
    pub sales: u64,
    pub seller: Pubkey,
    pub token: Pubkey,
    pub price: u64,
    pub id: u64,
    pub bump: u8,
    pub engine: String,
}

#[account]
pub struct Receipt {
    pub listing: Pubkey,
    pub bump: u8,
}

