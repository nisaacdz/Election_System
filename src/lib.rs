use blockify::trans::record::Record;
use serde::{Serialize, Deserialize};

pub struct VoterId {}
pub struct CandidateId {}
pub struct ElectionId {}

#[derive(Serialize, Deserialize, Record)]
pub struct EncryptedVote {
    
}

pub struct Vote {
    voter: VoterId,
    election: ElectionId,
    choice: CandidateId,
}

pub stuct Voter {
    id: VoterId,
}
pub struct Candidate {
    id: CandidateId,
}

pub struct Election {
    id: ElectionId,
    candidates: Vec<CandidateId>,
}