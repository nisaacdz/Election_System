use std::any::Any;

use blockify::{dist::Entity, record::Record, GenID};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct VoteSession {}

#[derive(Clone, Serialize, Deserialize)]
struct Vote {
    session: VoteSession,
    choice: GenID,
    public_key: Vec<u8>,
}

impl Record for Vote {
    fn is_valid(&self) -> bool {
        true
    }

    fn get_signer(&self) -> &[u8] {
        &self.public_key
    }
}

pub trait Detail {
    fn title(&self) -> &str;
    fn body(&self) -> &dyn Any;
}

pub struct Voter {
    pub details: Vec<Box<dyn Detail>>,
    pub public_key: Vec<u8>,
}

impl Voter {
    pub fn public_key(&self) -> &[u8] {
        &self.public_key
    }

    pub fn get_detail<T: 'static>(&self, title: &str) -> Option<&T> {
        for detail in self.details.iter() {
            if detail.title() == title {
                if let Some(body) = detail.body().downcast_ref::<T>() {
                    return Some(body);
                }
            }
        }
        None
    }
}
