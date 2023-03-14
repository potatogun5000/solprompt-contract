use anchor_lang::prelude::*;

//use crate::*;
use crate::*; //constants::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut, address = get_pubkey(ADMIN))]
    pub signer: Signer<'info>,
    #[account(
        init,
        space = STATE_PDA_SIZE,
        payer = signer,
        seeds = [STATE_SEED],
        bump
    )]
    pub state: Account<'info, State>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitSeller<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        space = SELLER_PDA_SIZE,
        payer = signer,
        seeds = [SELLER_SEED, signer.key().as_ref()],
        bump
    )]
    pub seller: Account<'info, Seller>,
    #[account(
        mut,
        seeds = [STATE_SEED],
        bump = state.bump
    )]
    pub state: Account<'info, State>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitBuyer<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        space = BUYER_PDA_SIZE,
        payer = signer,
        seeds = [BUYER_SEED, signer.key().as_ref()],
        bump
    )]
    pub buyer: Account<'info, Buyer>,
    #[account(
        mut,
        seeds = [STATE_SEED],
        bump = state.bump
    )]
    pub state: Account<'info, State>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateListing<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [SELLER_SEED, signer.key().as_ref()],
        bump = seller.bump
    )]
    pub seller: Account<'info, Seller>,
    #[account(
        init,
        space = LISTING_PDA_SIZE,
        payer = signer,
        seeds = [LISTING_SEED, signer.key().as_ref(), &seller.listings.to_le_bytes()],
        bump
    )]
    pub listing: Account<'info, Listing>,
    #[account(
        mut,
        seeds = [STATE_SEED],
        bump = state.bump
    )]
    pub state: Account<'info, State>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(
    listing_id: u64,
)]
pub struct ApproveListing<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(mut)]
    /// CHECK: already
    pub seller_account: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [SELLER_SEED, seller_account.key().as_ref()],
        bump = seller.bump,
        constraint = seller_account.key() == seller.seller
    )]
    pub seller: Account<'info, Seller>,
    #[account(
        mut,
        seeds = [LISTING_SEED, seller_account.key().as_ref(), &listing_id.to_le_bytes()],
        bump = listing.bump,
        constraint = listing.approved == false,
        constraint = seller_account.key() == listing.seller
    )]
    pub listing: Account<'info, Listing>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(
    listing_id: u64,
)]
pub struct PurchaseListing<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut, address = seller.seller)]
    /// CHECK: already
    pub seller_account: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [SELLER_SEED, seller_account.key().as_ref()],
        bump = seller.bump,
        constraint = seller_account.key() == seller.seller
    )]
    pub seller: Account<'info, Seller>,
    #[account(
        mut,
        seeds = [LISTING_SEED, seller_account.key().as_ref(), &listing_id.to_le_bytes()],
        bump = listing.bump,
        constraint = listing.seller == seller.seller,
        constraint = listing.approved == true
    )]
    pub listing: Account<'info, Listing>,
    #[account(
        mut,
        seeds = [BUYER_SEED, signer.key().as_ref()],
        bump = buyer.bump,
        constraint = buyer.buyer == signer.key()
    )]
    pub buyer: Account<'info, Buyer>,
    #[account(
        init,
        space = RECEIPT_PDA_SIZE,
        payer = signer,
        seeds = [RECEIPT_SEED, signer.key().as_ref(), &buyer.purchases.to_le_bytes()],
        bump,
    )]
    pub receipt: Account<'info, Receipt>,
    #[account(
        mut,
        seeds = [STATE_SEED],
        bump = state.bump
    )]
    pub state: Account<'info, State>,
    pub system_program: Program<'info, System>,
}

impl<'info> PurchaseListing<'info> {
    pub fn transfer_to(
        &self,
        to_key: &Pubkey,
        from_key: &Pubkey,
        to_account: AccountInfo<'info>,
        from_account: AccountInfo<'info>,
        amount: u64,
    ) -> Result<()> {
        let ix_transfer_to_loot_manager =
            anchor_lang::solana_program::system_instruction::transfer(from_key, to_key, amount);
        anchor_lang::solana_program::program::invoke(
            &ix_transfer_to_loot_manager,
            &[from_account, to_account],
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct WithdrawBalance<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [SELLER_SEED, signer.key().as_ref()],
        bump = seller.bump,
        constraint = signer.key() == seller.seller
    )]
    pub seller: Account<'info, Seller>,
    pub system_program: Program<'info, System>,
}

