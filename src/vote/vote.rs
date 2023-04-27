use super::Record;

use super::{Serialize, Deserialize};
use super::{axs::*, Encrypted};

#[derive(Clone, Serialize, Deserialize, Debug, Record)]
pub struct Vote {
    voter: VoterId,
    election: ElectionId,
    choice: Encrypted<CandidateId>,
}

impl Vote {
    pub fn new(voter: VoterId, election: ElectionId, choice: Encrypted<CandidateId>) -> Vote {
        Vote { voter, election, choice }
    }
}



#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Election {
    id: ElectionId,
    candidates: Vec<CandidateId>,
}


pub struct ElectionResult {
    res: Vec<(CandidateId, u64)>,
}
