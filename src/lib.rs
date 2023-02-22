use std::{any::Any, collections::HashSet};

use blockchain::{
    blockchain::{Record, SignedRecord},
    errs::CustomErrs,
    utils::Entity,
};
use serde::{Deserialize, Serialize};

pub mod vms;

trait Detail<T> {
    fn get_title(&self) -> String;
    fn get_body(&self) -> T;
}

#[derive(PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct Id {
    id: String,
}

impl Id {
    pub fn random() -> Self {
        Self { id: "".to_owned() }
    }
}

pub struct Election {
    id: Id,
    candidates: Vec<Id>,
    allowed_voters: HashSet<Id>,
}

impl Election {
    pub fn add_voter(&mut self, voter: &Voter) -> bool {
        self.allowed_voters.insert(voter.id.clone());
        true
    }

    pub fn add_voter_by_id(&mut self, voter_id: Id) -> bool {
        self.allowed_voters.insert(voter_id);
        true
    }
}

pub struct Voter {
    id: Id,
    public_key: Vec<u8>,
    pending_votes: Vec<Id>,

    unique_details: Vec<Box<dyn Detail<dyn Any>>>,

    details: Vec<Box<dyn Detail<dyn Any>>>,
}

impl Voter {
    pub fn vote(&self) -> Vote {
        
    }
}

impl Entity<Vote> for Voter {
    fn create() -> Self {
        todo!()
    }

    fn public_key(&self) -> &[u8] {
        &self.public_key
    }

    fn write_record(&self) -> Vote {
        Vote { voter: self.id.clone(), voted_for: Id::random() }
    }
}
///Candidate represents choices the Voter will vote for
/// 
pub struct Candidate {
    id: Id,
    details: Vec<Box<dyn Detail<dyn Any>>>,
}
///
///
///
///
///

#[derive(Clone, Serialize, Deserialize)]
struct Vote {
    voter: Id,
    voted_for: Id,
}

impl Record for Vote {}
