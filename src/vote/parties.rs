
use blockify::sec::rscs::AuthKeyPair;
use blockify::trans::record::SignedRecord;

use super::vote::Vote;
use super::Record;
use super::{Serialize, Deserialize, Encrypted};
use super::axs::*;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Voter {
    id: VoterId,
}

impl Voter {
    pub fn vote(&self, election: ElectionId, choice: CandidateId) -> Vote {
        let choice = self.encrypt_choice(choice);
        Vote::new(self.id.clone(), election, choice)
    }

    pub fn encrypt_choice(&self, choice: CandidateId) -> Encrypted<CandidateId> {
        todo!()
    }

    pub fn record(&self, election: ElectionId, choice: CandidateId, keypair: AuthKeyPair) -> SignedRecord<Vote> {
        let vote = self.vote(election, choice);
        vote.record(keypair).unwrap()
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Candidate {
    id: CandidateId,
}