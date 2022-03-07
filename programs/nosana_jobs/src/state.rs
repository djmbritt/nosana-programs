use crate::NosanaError;
use anchor_lang::prelude::*;

/// # Jobs
/// Account for holding jobs of a certain Project
/// - __authority__ is the payer and initial projects' creator
/// - __jobs__ is list of Jobs
pub const JOBS_SIZE: usize = 8 + std::mem::size_of::<Jobs>() + 32 * 100 + 16;

#[account]
pub struct Jobs {
    pub authority: Pubkey,
    pub jobs: Vec<Pubkey>,
}

impl Jobs {
    pub fn init(&mut self, authority: Pubkey) -> () {
        self.authority = authority;
        self.jobs = Vec::new();
    }

    pub fn add_job(&mut self, job_key: Pubkey) -> () {
        self.jobs.push(job_key);
    }

    pub fn remove_job(&mut self, job_key: &Pubkey) -> Result<()> {
        // find job in queue
        let index: Option<usize> = self.jobs.iter().position(|key: &Pubkey| key == job_key);

        // check if job is found
        require!(!index.is_none(), NosanaError::JobQueueNotFound);

        // remove job from jobs list
        self.jobs.remove(index.unwrap());
        Ok(())
    }
}

/// # Job
/// Object that holds relevant information for a single Job
/// - __node__ is the ID of the node that claims the Job
/// - __job_status__ is the JobStatus the current Job
/// - __ipfs_result__ is the IPFS hash pointing to the job instructions
/// - __ipfs_result__ is the IPFS hash pointing to the job results
/// - __tokens__ is amount of tokens
pub const JOB_SIZE: usize = 8 + std::mem::size_of::<Job>();

#[account]
pub struct Job {
    pub node: Pubkey,
    pub job_status: u8,
    pub ipfs_job: [u8; 32],
    pub ipfs_result: [u8; 32],
    pub tokens: u64,
}

impl Job {
    pub fn create(&mut self, data: [u8; 32], amount: u64) -> () {
        self.job_status = JobStatus::Created as u8;
        self.ipfs_job = data;
        self.tokens = amount;
    }

    pub fn claim(&mut self, node: Pubkey) -> () {
        self.job_status = JobStatus::Claimed as u8;
        self.node = node;
    }

    pub fn finish(&mut self, data: [u8; 32]) -> () {
        self.job_status = JobStatus::Finished as u8;
        self.ipfs_result = data;
    }

    pub fn cancel(&mut self) -> () {
        self.job_status = JobStatus::Cancelled as u8;
    }
}

/// # JobStatus
/// Enumeration for the different states a Job can have
#[derive(Clone, Debug, PartialEq, AnchorSerialize, AnchorDeserialize)]
#[repr(u8)]
pub enum JobStatus {
    Created = 0,
    Claimed = 1,
    Finished = 2,
    Cancelled = 3,
}

// token

pub mod nos_spl {
    use crate::ids::mint;
    use anchor_lang::prelude::*;
    use anchor_spl::token::{self, Transfer};

    pub fn transfer<'info>(
        program: AccountInfo<'info>,
        from: AccountInfo<'info>,
        to: AccountInfo<'info>,
        authority: AccountInfo<'info>,
        amount: u64,
    ) -> Result<()> {
        let accounts = token::Transfer {
            from,
            to,
            authority,
        };
        let ctx: CpiContext<Transfer> = CpiContext::new(program, accounts);
        return token::transfer(ctx, amount);
    }

    pub fn transfer_sign<'info>(
        program: AccountInfo<'info>,
        from: AccountInfo<'info>,
        to: AccountInfo<'info>,
        authority: AccountInfo<'info>,
        nonce: u8,
        amount: u64,
    ) -> Result<()> {
        let accounts = token::Transfer {
            from,
            to,
            authority,
        };
        return token::transfer(
            CpiContext::new_with_signer(program, accounts, &[&[&mint::ID.as_ref(), &[nonce]]]),
            amount,
        );
    }
}
