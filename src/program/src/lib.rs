use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token::mint_to;
use anchor_spl::token::{MintTo, Token};
use mpl_token_metadata::instruction::create_metadata_accounts_v2;

declare_id!("9FKLho9AUYScrrKgJbG1mExt5nSgEfk1CNEbR8qBwKTZ");

#[program]
pub mod nft_minting_contract {

    use super::*;

    pub fn nft_format(
        ctx: Context<MintNFT>,
        creator_key: Pubkey,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        msg!("Minting the NFT:");
}
