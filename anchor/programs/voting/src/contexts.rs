use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
#[instruction(poll_id:u64)]
pub struct CreatePoll<'info>{

  #[account(
    init,
    payer = signer,
    space = 8 + Poll::INIT_SPACE,
    seeds = [b"poll", signer.key().as_ref(), &poll_id.to_le_bytes()],
    bump
  )]
  pub poll : Account<'info,Poll>,

  #[account(
    mut
  )]
  pub signer : Signer<'info>,


  pub system_program : Program<'info,System>

}


#[derive(Accounts)]
#[instruction(candidate_id:u64)]
pub struct CreateCandidate<'info> {
  #[account(mut)]
  pub poll : Account<'info,Poll>,

  #[account(
    init,
    payer = signer,
    space = 8 + Candidate::INIT_SPACE,
    seeds = [b"candidate", poll.key().as_ref(), &candidate_id.to_le_bytes()],
    bump

  )]
  pub candidate : Account<'info,Candidate>,

  #[account(mut)]
  pub signer : Signer<'info>,

  pub system_program : Program<'info,System>

}

#[derive(Accounts)]
pub struct StopPoll<'info> {
  #[account(
    mut,
    has_one = signer
  )]
  pub poll : Account<'info,Poll>,

  pub signer : Signer<'info>,

}
