use crate::*;
use anchor_spl::token::TokenAccount;
use mpl_token_metadata::{
    pda::find_metadata_account,
    state::{Collection, Metadata, TokenMetadataAccount},
};
use nosana_staking::StakeAccount;

#[derive(Accounts)]
pub struct Enter<'info> {
    pub authority: Signer<'info>,
    #[account(mut, has_one = vault @ NosanaError::InvalidVault)]
    pub market: Account<'info, MarketAccount>,
    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,
    #[account(
        address = utils::get_staking_address(authority.key) @ NosanaError::StakeDoesNotMatchReward,
        has_one = authority @ NosanaError::Unauthorized,
        constraint = stake.amount >= market.node_stake_minimum @ NosanaError::NodeNotEnoughStake,
        constraint = stake.time_unstake == 0 @ NosanaError::NodeNoStake,
    )]
    pub stake: Account<'info, StakeAccount>,
    #[account(constraint = nft.owner == authority.key() @ NosanaError::Unauthorized)]
    pub nft: Account<'info, TokenAccount>,
    /// CHECK: we're going to deserialize this account within the instruction
    #[account(address = find_metadata_account(&nft.mint).0 @ NosanaError::NodeNftWrongMetadata)]
    pub metadata: AccountInfo<'info>,
}

pub fn handler(ctx: Context<Enter>) -> Result<()> {
    // get and verify our nft collection in the metadata, if required
    if ctx.accounts.market.node_access_key != id::SYSTEM_PROGRAM {
        let metadata: Metadata = Metadata::from_account_info(&ctx.accounts.metadata).unwrap();
        let collection: Collection = metadata.collection.unwrap();
        require!(
            collection.verified && collection.key == ctx.accounts.market.node_access_key,
            NosanaError::NodeKeyInvalidCollection
        )
    }
    require!(
        ctx.accounts
            .market
            .find_node(ctx.accounts.authority.key)
            .is_none(),
        NosanaError::NodeAlreadyQueued
    );

    // enter the queue
    ctx.accounts
        .market
        .enter_queue(ctx.accounts.authority.key());
    Ok(())
}
