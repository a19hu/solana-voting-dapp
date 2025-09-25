#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
mod contexts;
mod state;
declare_id!("FqzkXZdwYjurnUKetJCAvaUw5WAqbwzU6gZEwydeEfqS");

#[program]
pub mod counter {
    use super::*;
    pub fn initialze_poll(ctx:Context<InitialzePoll>,poll_id : u64,poll_end :u64,poll_start :u64, description:String)-> Result<()> {
        let poll = &mut ctx.accounts.poll;
        poll.poll_id = poll_id;
        poll.poll_end = poll_end;
        poll.poll_start = poll_start;
        poll.description = description;

        Ok(())
    }

    pub fn initialze_candidate(ctx:Context<InitialzeCandidate>,_poll_id : u64,condidate_name:String)-> Result<()> {
        let poll = &mut ctx.accounts.poll;

        let candidate = &mut ctx.accounts.candidate;
        candidate.condidate_name = condidate_name;
        candidate.condidate_votes = 0;

        Ok(())
    }

    pub fn vote(ctx:Context<Vote>,_poll_id : u64,_condidate_name:String)-> Result<()> {
        let candidate = &mut ctx.accounts.candidate;
        candidate.condidate_votes += 1;

        Ok(())
    }
    
    
   
}

#[derive(Accounts)]
#[instruction(poll_id:u64)]
pub struct InitialzePoll<'info>{
    #[account(
        mut
    )]
    pub signer : Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = 8+ Poll::INIT_SPACE,
        seeds = [poll_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub poll: Account<'info,Poll>,

    pub system_program : Program<'info,System>
}

#[derive(Accounts)]
#[instruction(poll_id:u64, condidate_name:String)]
pub struct InitialzeCandidate<'info>{
    #[account(
        mut
    )]
    pub signer : Signer<'info>,
    #[account(
        seeds = [poll_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub poll: Account<'info,Poll>,

    #[account(
        init,
        payer = signer,
        space = 8+ Candidate::INIT_SPACE,
        seeds = [poll_id.to_le_bytes().as_ref(),condidate_name.as_bytes()],
        bump,
    )]
    pub candidate: Account<'info,Candidate>,

    pub system_program : Program<'info,System>
}

#[derive(Accounts)]
#[instruction(poll_id:u64, condidate_name:String)]
pub struct Vote<'info>{
    pub signer : Signer<'info>,
    #[account(
        seeds = [poll_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub poll: Account<'info,Poll>,

    #[account(
        seeds = [poll_id.to_le_bytes().as_ref(),condidate_name.as_bytes()],
        bump,
    )]
    pub candidate: Account<'info,Candidate>,
}

#[account]
#[derive(InitSpace)]
pub struct Poll {
    pub poll_id : u64,
    pub creator : Pubkey,
    #[max_len(100)]
    pub description : String,
    pub poll_start : u64,
    pub poll_end: u64,
    pub poll_status : bool
}

#[account]
#[derive(InitSpace)]
pub struct Candidate {
    #[max_len(100)]
    pub condidate_name : String,
    pub condidate_votes : u64
}

#[account]
#[derive(InitSpace)]
pub struct VoteReceipt {
    pub poll: Pubkey,        // which poll this vote belongs to
    pub voter: Pubkey,       // the wallet that voted
    pub candidate: Pubkey,   // candidate chosen
}

#[account]
#[derive(InitSpace)]
pub struct Voter {
    pub wallet: Pubkey,        // wallet address
    pub votes_cast: u64,       // how many polls this wallet has participated in
    #[max_len(50)]
    pub nickname: String,      // optional
}
