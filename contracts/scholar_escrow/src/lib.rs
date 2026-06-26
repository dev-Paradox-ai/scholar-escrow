#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype,
    token, symbol_short,
    Address, Env, Vec,
};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Escrow,
}

#[contracttype]
#[derive(Clone, PartialEq)]
pub enum MilestoneStatus {
    Pending,
    Submitted,
    Approved,
}

#[contracttype]
#[derive(Clone)]
pub struct MilestoneRecord {
    pub status: MilestoneStatus,
    pub amount: i128,
}

#[contracttype]
#[derive(Clone)]
pub struct EscrowState {
    pub ngo: Address,
    pub student: Address,
    pub token: Address,
    pub milestones: Vec<MilestoneRecord>,
    pub total_funded: i128,
    pub total_released: i128,
}

#[contract]
pub struct ScholarEscrow;

#[contractimpl]
impl ScholarEscrow {

    /// Called by the NGO to initialize the escrow and deposit USDC.
    /// milestone_amounts: a Vec of USDC amounts, one per milestone.
    pub fn initialize(
        env: Env,
        ngo: Address,
        student: Address,
        token: Address,
        milestone_amounts: Vec<i128>,
    ) {
        ngo.require_auth();

        if env.storage().instance().has(&DataKey::Escrow) {
            panic!("already initialized");
        }

        let total: i128 = milestone_amounts.iter().sum();

        let token_client = token::Client::new(&env, &token);
        token_client.transfer(&ngo, &env.current_contract_address(), &total);

        let mut milestones: Vec<MilestoneRecord> = Vec::new(&env);
        for amount in milestone_amounts.iter() {
            milestones.push_back(MilestoneRecord {
                status: MilestoneStatus::Pending,
                amount,
            });
        }

        let state = EscrowState {
            ngo,
            student,
            token,
            milestones,
            total_funded: total,
            total_released: 0,
        };

        env.storage().instance().set(&DataKey::Escrow, &state);
        env.events().publish((symbol_short!("init"),), total);
    }

    /// Called by the student to mark a milestone as submitted.
    pub fn submit_milestone(env: Env, student: Address, milestone_index: u32) {
        student.require_auth();

        let mut state: EscrowState = env
            .storage()
            .instance()
            .get(&DataKey::Escrow)
            .expect("not initialized");

        if state.student != student {
            panic!("unauthorized: not the registered student");
        }

        let mut record = state.milestones.get(milestone_index).expect("invalid index");

        if record.status != MilestoneStatus::Pending {
            panic!("milestone already submitted or approved");
        }

        record.status = MilestoneStatus::Submitted;
        state.milestones.set(milestone_index, record);

        env.storage().instance().set(&DataKey::Escrow, &state);
        env.events().publish((symbol_short!("submit"),), milestone_index);
    }

    /// Called by the NGO to approve a submitted milestone and release funds.
    pub fn approve_milestone(env: Env, ngo: Address, milestone_index: u32) {
        ngo.require_auth();

        let mut state: EscrowState = env
            .storage()
            .instance()
            .get(&DataKey::Escrow)
            .expect("not initialized");

        if state.ngo != ngo {
            panic!("unauthorized: not the registered NGO");
        }

        let mut record = state.milestones.get(milestone_index).expect("invalid index");

        if record.status != MilestoneStatus::Submitted {
            panic!("milestone not in Submitted state");
        }

        let token_client = token::Client::new(&env, &state.token);
        token_client.transfer(
            &env.current_contract_address(),
            &state.student,
            &record.amount,
        );

        record.status = MilestoneStatus::Approved;
        state.total_released += record.amount;
        state.milestones.set(milestone_index, record.clone());

        env.storage().instance().set(&DataKey::Escrow, &state);
        env.events().publish(
            (symbol_short!("approve"),),
            (milestone_index, record.amount),
        );
    }

    /// Returns the current escrow state. Read-only.
    pub fn get_state(env: Env) -> EscrowState {
        env.storage()
            .instance()
            .get(&DataKey::Escrow)
            .expect("not initialized")
    }
}
