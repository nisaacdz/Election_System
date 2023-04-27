use std::marker::PhantomData;

use serde::{Serialize, Deserialize};
use blockify::trans::record::Record;

mod axs;
mod vote;
mod parties;

#[derive(Clone, Serialize, Deserialize, Debug
)]
pub struct Encrypted<T> {
    val: Box<[u8]>,
    dat: PhantomData<T>
}

impl<T> Encrypted<T> {
    pub fn encrypt(val: T) -> Encrypted<T> {
        todo!()
    }
}