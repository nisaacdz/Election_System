use electionapp::{Id, Voter, Election};

fn main() {
    let (voter, private_key) = Voter::generate();
    let election = Election::random(&voter);


    match voter.cast_signed_vote(election.id, Id::random(), &private_key) {
        Ok(vote) => println!("Success ! {:?}", vote),
        Err(err) => println!("Failure ! {:?}", err),
    }
}