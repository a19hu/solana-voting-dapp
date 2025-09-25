#[warn(dead_code)]

use anchor_lang::prelude::*;


#[account]
#[derive(InitSpace)]
pub struct Poll {
    pub poll_id : u64,
    pub creator : Pubkey,
    #[max_len(20)]
    pub title : String,
    #[max_len(100)]
    pub description : String,
    pub poll_start : u64,
    pub poll_end : u64,
    pub poll_status : bool, 
}

#[account]
#[derive(InitSpace)]
pub struct Candidate {
    pub poll : Pubkey,
    pub candidate_id : u64,
    #[max_len(20)]
    pub candidate_name : String,
    pub candidate_vote : u64
}


#[account]
#[derive(InitSpace)]
pub struct VoteReceipt {
    pub poll : Pubkey,
    pub candidate : Pubkey,
    pub voter : Pubkey
}