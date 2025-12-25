#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
mod contexts;
mod state;

use state::*;
use contexts::*;
declare_id!("FqzkXZdwYjurnUKetJCAvaUw5WAqbwzU6gZEwydeEfqS");

#[program]
pub mod votee {
    use super::*;
    
    pub fn create_poll(
        ctx:Context<CreatePoll>,
        poll_id : u64,
        title : String,
        description : String,
        poll_start : u64,
        poll_end : u64,
        poll_status : bool
    ) -> Result<()> {
        let poll = &mut ctx.accounts.poll;
        poll.poll_id = poll_id;
        poll.creator = ctx.accounts.signer.key();
        poll.title = title ;
        poll.description = description;
        poll.poll_end = poll_end;
        poll.poll_start = poll_start;
        poll.poll_status = poll_status;


        Ok(())
    }

    pub fn create_candidate(
        ctx:Context<CreateCandidate>,
        candidate_id :u64,
        candidate_name : String,
    ) -> Result<()>{
        let candidate =&mut ctx.accounts.candidate;
        candidate.candidate_id = candidate_id;
        candidate.candidate_name = candidate_name;
        candidate.candidate_vote = 0;


        Ok(())
    }



    pub fn delete_poll(_ctx:Context<StopPoll>) -> Result<()> {
        

        Ok(())
    }
    
   
}
