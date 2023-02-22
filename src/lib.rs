use std::{any::Any, collections::HashSet};

use blockchain::{
    blockchain::{Record, SignedRecord},
    errs::CustomErrs,
    gen,
    utils::Entity,
};
use serde::{Deserialize, Serialize};

pub mod vms;

pub trait Detail<T> {
    fn get_title(&self) -> String;
    fn get_body(&self) -> T;
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct Id {
    id: String,
}

impl Id {
    pub fn random() -> Self {
        Self { id: "".to_owned() }
    }
}

pub struct Election {
    pub id: Id,
    pub candidates: Vec<Id>,
    pub allowed_voters: HashSet<Id>,
}

impl Election {
    pub fn random(voter: &Voter) -> Self {
        let mut obj = Self {
            id: Id::random(),
            candidates: vec![Id::random()],
            allowed_voters: HashSet::new(),
        };

        obj.add_voter(voter);

        obj
    }

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
    pub public_key: Vec<u8>,
    pub pending_votes: Vec<Id>,

    pub unique_details: Vec<Box<dyn Detail<dyn Any>>>,

    pub details: Vec<Box<dyn Detail<dyn Any>>>,
}

impl Voter {
    fn new(public_key: Vec<u8>) -> Self {
        Self {
            id: Id::random(),
            public_key,
            pending_votes: vec![],
            unique_details: vec![],
            details: vec![],
        }
    }
    pub fn cast_vote(&self, election_id: Id, candidate_id: Id) -> Vote {
        Vote { voter: self.id.clone(), election_id, voted_for: candidate_id }
    }

    pub fn cast_signed_vote(&self, election_id: Id, candidate_id: Id, pkey: &[u8]) -> Result<SignedRecord<Vote>, CustomErrs> {
        let vote = self.cast_vote(election_id, candidate_id);
        self.sign_record(vote, pkey)
    }

    pub fn generate() -> (Self, Vec<u8>) {
        let (private_key, public_key) = gen::generate_key_pair();
        let obj = Voter::new(public_key);

        (obj, private_key)
    }
}

impl Entity<Vote> for Voter {
    fn public_key(&self) -> &[u8] {
        &self.public_key
    }
}

///Candidate represents choices the Voter will vote for
///
pub struct Candidate {
    pub id: Id,
    pub details: Vec<Box<dyn Detail<dyn Any>>>,
}

impl Candidate {
    pub fn random() -> Self {
        Self {
            id: Id::random(),
            details: vec![],
        }
    }
}

///
///
///
///
///

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vote {
    voter: Id,
    election_id: Id,
    voted_for: Id,
}

impl Record for Vote {}
