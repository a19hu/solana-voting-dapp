use anchor_lang::prelude::*;
use crate::state::*;

#[Accounts]
pub struct CreatePoll<'info>{
    #[account(
        init,
        payer = creator,
        space = 8 + Poll::INIT_SPACE,
        seeds = [b"poll", creator.key().as_ref(), &poll_id.to_le_bytes()],
        bump
    )]
    pub poll: Account<'info, Poll>,

    #[account(mut)]
    pub creator: Signer<'info>,

    pub system_program: Program<'info, System>,
}