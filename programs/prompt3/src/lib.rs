#![allow(unused)]
use anchor_lang::prelude::*;

declare_id!("F6X97TzFoSyRAsMCVEjRncxAqNNmDqGoaq5ESFKvCzWZ");

pub mod constants;
pub mod instructions;
pub mod state;
pub mod util;

use constants::*;
use instructions::*;
use state::*;
use util::*;

#[program]
pub mod prompt3 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.buyers = 0;
        state.listings = 0;
        state.sales = 0;
        state.sellers = 0;
        state.bump = *ctx.bumps.get("state").unwrap();

        Ok(())
    }

    pub fn init_buyer(ctx: Context<InitBuyer>) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.buyers = state.buyers.checked_add(1).unwrap();

        let buyer = &mut ctx.accounts.buyer;

        buyer.buyer = ctx.accounts.signer.key();
        buyer.purchases = 0;
        buyer.bump = *ctx.bumps.get("buyer").unwrap();

        Ok(())
    }

    pub fn init_seller(ctx: Context<InitSeller>) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.sellers = state.listings.checked_add(1).unwrap();

        let seller = &mut ctx.accounts.seller;

        seller.seller = ctx.accounts.signer.key();
        seller.sales = 0;
        seller.listings = 0;
        seller.balance = 0;
        seller.bump = *ctx.bumps.get("seller").unwrap();

        Ok(())
    }

    pub fn create_listing(
        ctx: Context<CreateListing>,
        engine: String,
        token: Pubkey,
        price: u64,
    ) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.listings = state.listings.checked_add(1).unwrap();

        let seller = &mut ctx.accounts.seller;
        seller.listings = seller.listings.checked_add(1).unwrap();

        let listing = &mut ctx.accounts.listing;
        listing.engine = engine;
        listing.sales = 0;
        listing.approved = false;
        listing.id = seller.listings;
        listing.seller = ctx.accounts.signer.to_account_info().key();
        listing.token = token;
        listing.price = price;
        listing.bump = *ctx.bumps.get("listing").unwrap();

        Ok(())
    }

    pub fn purchase_listing(ctx: Context<PurchaseListing>, listing_id: u64) -> Result<()> {
        let price = ctx.accounts.listing.price;

        let amount_seller = price.checked_mul(4).unwrap().checked_div(5).unwrap();
        let amount_admin = price.checked_sub(amount_seller).unwrap();

        let seller = &mut ctx.accounts.seller;
        seller.sales = seller.sales.checked_add(1).unwrap();
        seller.balance = seller.balance.checked_add(amount_seller).unwrap();

        let listing = &mut ctx.accounts.listing;
        listing.sales = listing.sales.checked_add(1).unwrap();

        let buyer = &mut ctx.accounts.buyer;
        buyer.purchases = buyer.purchases.checked_add(1).unwrap();

        let state = &mut ctx.accounts.state;
        state.sales = state.sales.checked_add(1).unwrap();
        state.balance = state.balance.checked_add(amount_admin).unwrap();

        let receipt = &mut ctx.accounts.receipt;
        receipt.listing = ctx.accounts.listing.key();

        ctx.accounts.transfer_to(
            &ctx.accounts.seller.key(),
            &ctx.accounts.signer.to_account_info().key(),
            ctx.accounts.seller.to_account_info(),
            ctx.accounts.signer.to_account_info(),
            amount_seller,
        )?;

        ctx.accounts.transfer_to(
            &ctx.accounts.state.key(),
            &ctx.accounts.signer.to_account_info().key(),
            ctx.accounts.state.to_account_info(),
            ctx.accounts.signer.to_account_info(),
            amount_admin,
        )?;

        Ok(())
    }

    pub fn withdraw_balance(ctx: Context<WithdrawBalance>) -> Result<()> {
        if ctx.accounts.seller.balance == 0 {
            return err!(Errors::InvalidBalance);
        }

        **ctx
            .accounts
            .seller
            .to_account_info()
            .try_borrow_mut_lamports()? -= ctx.accounts.seller.balance;

        **ctx.accounts.signer.try_borrow_mut_lamports()? += ctx.accounts.seller.balance;

        let seller_account = &mut ctx.accounts.seller;
        seller_account.balance = 0;

        Ok(())
    }

    /* ADMIN */
    pub fn approve_listing(ctx: Context<ApproveListing>, listing_id: u64) -> Result<()> {
        if ctx.accounts.admin.to_account_info().key() != get_pubkey(MOD_1)
            && ctx.accounts.admin.to_account_info().key() != get_pubkey(MOD_2)
        {
            return err!(Errors::InvalidSigner);
        }

        let listing = &mut ctx.accounts.listing;
        listing.approved = true;

        Ok(())
    }
}

#[error_code]
pub enum Errors {
    #[msg("invalid signer")]
    InvalidSigner,
    #[msg("invalid balance")]
    InvalidBalance,

}
