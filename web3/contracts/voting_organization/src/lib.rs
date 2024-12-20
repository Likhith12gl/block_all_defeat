#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, vec, Address, Env, String, Symbol, Vec,
    U256,
};

const PENDING: Symbol = symbol_short!("Pending");
const APPROVED: Symbol = symbol_short!("Approved");
const REJECTED: Symbol = symbol_short!("Rejected");

#[contracttype]
pub struct Voter {
    voter_address: Address,
    name: String,
    ipfs: String,
    register_id: U256,
    status: Symbol,
    has_voted: bool,
    message: String,
}

#[contracttype]
pub struct Candidate {
    candidate_address: Address,
    name: String,
    ipfs: String,
    register_id: U256,
    status: Symbol,
    vote_count: U256,
    message: String,
}

#[contracttype]
pub enum Voters {
    Voter(Address),
}

#[contracttype]
pub enum Candidates {
    Candidate(Address),
}

const OWNER: Symbol = symbol_short!("Owner");
const REGISTERED_VOTERS: Symbol = symbol_short!("RegVot");
const REGISTERED_CANDIDATES: Symbol = symbol_short!("RegCan");
const APPROVED_VOTERS: Symbol = symbol_short!("ApproVot");
const APPROVED_CANDIDATES: Symbol = symbol_short!("ApproCan");
const VOTED_VOTERS: Symbol = symbol_short!("VotVoted");
const VOTER_ID_COUNTER: Symbol = symbol_short!("votIdCntr");
const CANDIDATE_ID_COUNTER: Symbol = symbol_short!("canIdCntr");
const START_TIME: Symbol = symbol_short!("StartTime");
const END_TIME: Symbol = symbol_short!("EndTime");

#[contract]
pub struct VotingOrganization;

#[contractimpl]
impl VotingOrganization {
    fn owner_only(env: &Env, address: Address) {
        let addr: Address = Address::from_string(&String::from_str(env, ""));
        let stored_addr = env.storage().persistent().get(&OWNER).unwrap_or(addr);

        if stored_addr != address {
            panic!("can only be called by owner");
        }
    }

    fn only_during_voting_period(env: &Env) {
        let start_time = env.storage().persistent().get(&START_TIME).unwrap_or(0);
        let end_time = env.storage().persistent().get(&END_TIME).unwrap_or(0);

        if !(env.ledger().timestamp() >= start_time && env.ledger().timestamp() <= end_time) {
            panic!("Voting is not active")
        }
    }

    pub fn init(env: Env, owner_address: Address) {
        env.storage().persistent().set(&OWNER, &owner_address);
        env.storage().persistent().set(&VOTER_ID_COUNTER, &1);
        env.storage().persistent().set(&CANDIDATE_ID_COUNTER, &1);
    }

    pub fn registerVoter(env: Env, name: String, ipfs: String, address: Address) {
        const PENDING_MESSAGE: &str = "Currently your registration is pending";
        let voter_id_key = Voters::Voter(address.clone());
        let id_counter_key = VOTER_ID_COUNTER;

        let id_counter = env
            .storage()
            .persistent()
            .get(&id_counter_key)
            .unwrap_or_default();

        let new_voter = Voter {
            voter_address: address.clone(),
            name,
            ipfs,
            has_voted: false,
            message: String::from_str(&env, PENDING_MESSAGE),
            register_id: U256::from_u32(&env, id_counter),
            status: PENDING,
        };

        env.storage().persistent().set(&voter_id_key, &new_voter);
        let mut registered_voters: Vec<Address> = env
            .storage()
            .persistent()
            .get(&REGISTERED_VOTERS)
            .unwrap_or(vec![&env]);

        registered_voters.push_back(address);

        env.storage()
            .persistent()
            .set(&REGISTERED_VOTERS, &registered_voters);

        env.storage()
            .persistent()
            .set(&id_counter_key, &(id_counter + 1));
    }

    pub fn register_candidate(env: Env, name: String, ipfs: String, address: Address) {
        const PENDING_MESSAGE: &str = "Currently your registration is pending";
        let candidate_id_key = Candidates::Candidate(address.clone());
        let id_counter_key = VOTER_ID_COUNTER;

        let id_counter = env
            .storage()
            .persistent()
            .get(&id_counter_key)
            .unwrap_or_default();

        let new_candidate = Candidate {
            candidate_address: address.clone(),
            name,
            ipfs,
            message: String::from_str(&env, PENDING_MESSAGE),
            register_id: U256::from_u32(&env, id_counter),
            status: PENDING,
            vote_count: U256::from_u32(&env, 0),
        };

        env.storage()
            .persistent()
            .set(&candidate_id_key, &new_candidate);

        let mut registered_voters: Vec<Address> = env
            .storage()
            .persistent()
            .get(&REGISTERED_CANDIDATES)
            .unwrap_or(vec![&env]);

        registered_voters.push_back(address);

        env.storage()
            .persistent()
            .set(&REGISTERED_CANDIDATES, &registered_voters);

        env.storage()
            .persistent()
            .set(&id_counter_key, &(id_counter + 1));
    }

    pub fn approve_voter(env: Env, address: Address, message: String) {
        Self::owner_only(&env, address.clone());

        let key = Voters::Voter(address.clone());
        let mut voter = env.storage().persistent().get(&key).unwrap_or(Voter {
            name: String::from_str(&env, ""),
            ipfs: String::from_str(&env, "NotFound"),
            message: String::from_str(&env, ""),
            has_voted: false,
            register_id: U256::from_u32(&env, 0),
            status: REJECTED,
            voter_address: address.clone(),
        });

        assert_ne!(
            voter.ipfs,
            String::from_str(&env, "NotFound"),
            "Voter not found"
        );

        voter.status = APPROVED;
        voter.message = message;

        env.storage().persistent().set(&key, &voter);

        let mut approved_voters: Vec<Address> = env
            .storage()
            .persistent()
            .get(&APPROVED_VOTERS)
            .unwrap_or(vec![&env]);

        approved_voters.push_back(address);
        env.storage()
            .persistent()
            .set(&APPROVED_VOTERS, &approved_voters);
    }

    pub fn approve_candidate(env: Env, address: Address, message: String) {
        Self::owner_only(&env, address.clone());

        let key = Candidates::Candidate(address.clone());
        let mut candidate = env.storage().persistent().get(&key).unwrap_or(Candidate {
            name: String::from_str(&env, ""),
            ipfs: String::from_str(&env, "NotFound"),
            message: String::from_str(&env, ""),
            register_id: U256::from_u32(&env, 0),
            status: REJECTED,
            candidate_address: address.clone(),
            vote_count: U256::from_u32(&env, 0),
        });

        assert_ne!(
            candidate.ipfs,
            String::from_str(&env, "NotFound"),
            "Voter not found"
        );

        candidate.status = APPROVED;
        candidate.message = message;

        env.storage().persistent().set(&key, &candidate);

        let mut approved_candidate: Vec<Address> = env
            .storage()
            .persistent()
            .get(&APPROVED_CANDIDATES)
            .unwrap_or_else(|| vec![&env]);

        approved_candidate.push_back(address);
        env.storage()
            .persistent()
            .set(&APPROVED_CANDIDATES, &approved_candidate);
    }

    pub fn reject_voter(env: Env, address: Address, message: String) {
        Self::owner_only(&env, address.clone());

        let key = Voters::Voter(address.clone());
        let mut voter = env.storage().persistent().get(&key).unwrap_or(Voter {
            name: String::from_str(&env, ""),
            ipfs: String::from_str(&env, "NotFound"),
            message: String::from_str(&env, ""),
            has_voted: false,
            register_id: U256::from_u32(&env, 0),
            status: REJECTED,
            voter_address: address.clone(),
        });

        assert_ne!(
            voter.ipfs,
            String::from_str(&env, "NotFound"),
            "Voter not found"
        );

        voter.status = APPROVED;
        voter.message = message;

        env.storage().persistent().set(&key, &voter);
    }

    pub fn reject_candidate(env: Env, address: Address, message: String) {
        Self::owner_only(&env, address.clone());

        let key = Candidates::Candidate(address.clone());
        let mut candidate = env.storage().persistent().get(&key).unwrap_or(Voter {
            name: String::from_str(&env, ""),
            ipfs: String::from_str(&env, "NotFound"),
            message: String::from_str(&env, ""),
            has_voted: false,
            register_id: U256::from_u32(&env, 0),
            status: REJECTED,
            voter_address: address.clone(),
        });

        assert_ne!(
            candidate.ipfs,
            String::from_str(&env, "NotFound"),
            "Voter not found"
        );

        candidate.status = APPROVED;
        candidate.message = message;

        env.storage().persistent().set(&key, &candidate);
    }

    pub fn set_voting_period(env: Env, start_time: u64, end_time: u64, address: Address) {
        Self::owner_only(&env, address.clone());

        assert!(start_time < end_time, "Start time must be before end time.");

        env.storage().persistent().set(&START_TIME, &start_time);
        env.storage().persistent().set(&END_TIME, &end_time);
    }

    pub fn get_all_registered_voters(env: Env) -> Vec<Voter> {
        let mut voters: Vec<Voter> = vec![&env];

        let registered_voters: Vec<Address> = env
            .storage()
            .persistent()
            .get(&REGISTERED_VOTERS)
            .unwrap_or(vec![&env]);

        for v in registered_voters.iter() {
            let key = Voters::Voter(v.clone());
            let voter = env.storage().persistent().get(&key).unwrap_or(Voter {
                has_voted: false,
                ipfs: String::from_str(&env, "NotFound"),
                message: String::from_str(&env, ""),
                name: String::from_str(&env, ""),
                register_id: U256::from_u32(&env, 0),
                status: REJECTED,
                voter_address: v,
            });

            if voter.ipfs != String::from_str(&env, "NotFound") {
                voters.push_back(voter);
            }
        }

        return voters;
    }

    pub fn get_all_registered_candidates(env: Env) -> Vec<Candidate> {
        let mut candidates: Vec<Candidate> = vec![&env];

        let registered_candidates: Vec<Address> = env
            .storage()
            .persistent()
            .get(&REGISTERED_CANDIDATES)
            .unwrap_or(vec![&env]);

        for c in registered_candidates.iter() {
            let key = Candidates::Candidate(c.clone());
            let candidate = env.storage().persistent().get(&key).unwrap_or(Candidate {
                ipfs: String::from_str(&env, "NotFound"),
                message: String::from_str(&env, ""),
                name: String::from_str(&env, ""),
                register_id: U256::from_u32(&env, 0),
                status: REJECTED,
                candidate_address: c,
                vote_count: U256::from_u32(&env, 0),
            });

            if candidate.ipfs != String::from_str(&env, "NotFound") {
                candidates.push_back(candidate);
            }
        }

        return candidates;
    }

    pub fn get_all_approved_candidates(env: Env) -> Vec<Candidate> {
        let mut approved_candidates: Vec<Candidate> = vec![&env];

        let approved_address: Vec<Address> = env
            .storage()
            .persistent()
            .get(&APPROVED_CANDIDATES)
            .unwrap_or(vec![&env]);

        for a in approved_address.iter() {
            let key = Candidates::Candidate(a.clone());
            let candidate = env.storage().persistent().get(&key).unwrap_or(Candidate {
                ipfs: String::from_str(&env, "NotFound"),
                message: String::from_str(&env, ""),
                name: String::from_str(&env, ""),
                register_id: U256::from_u32(&env, 0),
                status: REJECTED,
                candidate_address: a,
                vote_count: U256::from_u32(&env, 0),
            });

            if candidate.ipfs != String::from_str(&env, "NotFound") {
                approved_candidates.push_back(candidate);
            }
        }

        return approved_candidates;
    }

    pub fn get_all_approved_voters(env: Env) -> Vec<Voter> {
        let mut approved_voters: Vec<Voter> = vec![&env];

        let approved_address: Vec<Address> = env
            .storage()
            .persistent()
            .get(&APPROVED_VOTERS)
            .unwrap_or(vec![&env]);

        for a in approved_address.iter() {
            let key = Voters::Voter(a.clone());
            let voter = env.storage().persistent().get(&key).unwrap_or(Voter {
                ipfs: String::from_str(&env, "NotFound"),
                message: String::from_str(&env, ""),
                name: String::from_str(&env, ""),
                register_id: U256::from_u32(&env, 0),
                status: REJECTED,
                voter_address: a,
                has_voted: false,
            });

            if voter.ipfs != String::from_str(&env, "NotFound") {
                approved_voters.push_back(voter);
            }
        }

        return approved_voters;
    }

    pub fn get_candidate(env: Env, addr: Address) -> Candidate {
        let key = Candidates::Candidate(addr.clone());
        let candidate = env.storage().persistent().get(&key).unwrap_or(Candidate {
            ipfs: String::from_str(&env, "NotFound"),
            message: String::from_str(&env, ""),
            name: String::from_str(&env, ""),
            register_id: U256::from_u32(&env, 0),
            status: REJECTED,
            candidate_address: addr,
            vote_count: U256::from_u32(&env, 0),
        });

        return candidate;
    }

    pub fn get_voter(env: Env, addr: Address) -> Voter {
        let key = Voters::Voter(addr.clone());
        let voter = env.storage().persistent().get(&key).unwrap_or(Voter {
            ipfs: String::from_str(&env, "NotFound"),
            message: String::from_str(&env, ""),
            name: String::from_str(&env, ""),
            register_id: U256::from_u32(&env, 0),
            status: REJECTED,
            voter_address: addr,
            has_voted: false,
        });

        return voter;
    }

    pub fn update_voter(env: Env, name: String, ipfs: String, addr: Address) {
        let key = Voters::Voter(addr.clone());
        let mut voter = env.storage().persistent().get(&key).unwrap_or(Voter {
            ipfs: String::from_str(&env, "NotFound"),
            message: String::from_str(&env, ""),
            name: String::from_str(&env, ""),
            register_id: U256::from_u32(&env, 0),
            status: REJECTED,
            voter_address: addr,
            has_voted: false,
        });

        assert_ne!(voter.ipfs, String::from_str(&env, "NotFound"));

        voter.name = name;
        voter.ipfs = ipfs;

        env.storage().persistent().set(&key, &voter)
    }

    pub fn update_candidate(env: Env, name: String, ipfs: String, addr: Address) {
        let key = Candidates::Candidate(addr.clone());
        let mut candidate = env.storage().persistent().get(&key).unwrap_or(Candidate {
            ipfs: String::from_str(&env, "NotFound"),
            message: String::from_str(&env, ""),
            name: String::from_str(&env, ""),
            register_id: U256::from_u32(&env, 0),
            status: REJECTED,
            candidate_address: addr,
            vote_count: U256::from_u32(&env, 0),
        });

        assert_ne!(candidate.ipfs, String::from_str(&env, "NotFound"));

        candidate.name = name;
        candidate.ipfs = ipfs;

        env.storage().persistent().set(&key, &candidate)
    }

    pub fn change_owner(env: Env, new_owner: Address, address: Address) {
        Self::owner_only(&env, address.clone());
        env.storage().persistent().set(&OWNER, &new_owner);
    }

    pub fn reset_contract(env: Env, address: Address) {
        Self::owner_only(&env, address.clone());

        let voters: Vec<Address> = env
            .storage()
            .persistent()
            .get(&REGISTERED_VOTERS)
            .unwrap_or(vec![&env]);

        for v in voters {
            let key = Voters::Voter(v);
            env.storage().persistent().remove(&key);
        }

        let candidates: Vec<Address> = env
            .storage()
            .persistent()
            .get(&REGISTERED_CANDIDATES)
            .unwrap_or(vec![&env]);

        for c in candidates {
            let key = Candidates::Candidate(c);
            env.storage().persistent().remove(&key);
        }

        env.storage().persistent().remove(&REGISTERED_VOTERS);
        env.storage().persistent().remove(&REGISTERED_CANDIDATES);
        env.storage().persistent().remove(&APPROVED_VOTERS);
        env.storage().persistent().remove(&APPROVED_CANDIDATES);
        env.storage().persistent().remove(&VOTED_VOTERS);
        env.storage().persistent().set(&VOTER_ID_COUNTER, &1);
        env.storage().persistent().set(&CANDIDATE_ID_COUNTER, &1);
        env.storage().persistent().set(&START_TIME, &0);
        env.storage().persistent().set(&END_TIME, &0);
    }

    pub fn vote(env: Env, candidate_address: Address, voter_address: Address) {
        Self::only_during_voting_period(&env);
        let mut voter = env
            .storage()
            .persistent()
            .get(&Voters::Voter(voter_address.clone()))
            .unwrap_or(Voter {
                has_voted: false,
                ipfs: String::from_str(&env, "NotFound"),
                message: String::from_str(&env, ""),
                name: String::from_str(&env, ""),
                voter_address: voter_address.clone(),
                register_id: U256::from_u32(&env, 0),
                status: REJECTED,
            });

        assert_ne!(
            voter.ipfs,
            String::from_str(&env, "NotFound"),
            "Account Not Found"
        );
        assert_eq!(voter.status, APPROVED, "You are not an approved voter.");
        assert!(!voter.has_voted, "You have already voted.");

        let mut candidate = env
            .storage()
            .persistent()
            .get(&Candidates::Candidate(candidate_address.clone()))
            .unwrap_or(Candidate {
                ipfs: String::from_str(&env, "NotFound"),
                message: String::from_str(&env, ""),
                name: String::from_str(&env, ""),
                candidate_address: candidate_address.clone(),
                register_id: U256::from_u32(&env, 0),
                status: REJECTED,
                vote_count: U256::from_u32(&env, 0),
            });
        assert_ne!(
            candidate.ipfs,
            String::from_str(&env, "NotFound"),
            "Account Not Found"
        );
        assert_eq!(candidate.status, APPROVED, "Candidate is not approved.");

        voter.has_voted = true;
        candidate.vote_count = candidate.vote_count.add(&U256::from_u32(&env, 1));

        env.storage()
            .persistent()
            .set(&Candidates::Candidate(candidate_address), &candidate);
        env.storage()
            .persistent()
            .set(&Voters::Voter(voter_address.clone()), &voter);

        let mut voters_who_voted: Vec<Address> = env
            .storage()
            .persistent()
            .get(&VOTED_VOTERS)
            .unwrap_or(vec![&env]);

        voters_who_voted.push_back(voter_address);
    }

    pub fn get_all_voters_who_voted(env: Env) -> Vec<Voter> {
        return env
            .storage()
            .persistent()
            .get(&VOTED_VOTERS)
            .unwrap_or(vec![&env]);
    }

    pub fn get_current_voting_status(env: Env) -> Candidate {
        let mut winning_candidate = Candidate {
            name: String::from_str(&env, ""),
            ipfs: String::from_str(&env, "NotFound"),
            message: String::from_str(&env, ""),
            candidate_address: Address::from_string(&String::from_str(&env, "")),
            register_id: U256::from_u32(&env, 0),
            vote_count: U256::from_u32(&env, 0),
            status: REJECTED,
        };

        let candidates: Vec<Address> = env
            .storage()
            .persistent()
            .get(&REGISTERED_CANDIDATES)
            .unwrap_or(vec![&env]);

        for c in candidates {
            let key = Candidates::Candidate(c);
            let cand = env.storage().persistent().get(&key).unwrap_or(Candidate {
                name: String::from_str(&env, ""),
                ipfs: String::from_str(&env, ""),
                message: String::from_str(&env, ""),
                candidate_address: Address::from_string(&String::from_str(&env, "")),
                register_id: U256::from_u32(&env, 0),
                vote_count: U256::from_u32(&env, 0),
                status: REJECTED,
            });

            if winning_candidate.vote_count < cand.vote_count {
                winning_candidate = cand;
            }
        }

        return winning_candidate;
    }

    pub fn get_winning_candidate(env: Env) -> Candidate {
        let end_time: u64 = env.storage().persistent().get(&END_TIME).unwrap_or(0);
        assert!(env.ledger().timestamp() > end_time);
        return Self::get_current_voting_status(env);
    }

    pub fn get_voting_time(env: Env) -> Vec<u64> {
        let start_time: u64 = env.storage().persistent().get(&START_TIME).unwrap_or(0);
        let end_time: u64 = env.storage().persistent().get(&END_TIME).unwrap_or(0);
        return vec![&env, start_time, end_time];
    }
}
