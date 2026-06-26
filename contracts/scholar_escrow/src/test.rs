#[cfg(test)]
mod tests {
    use soroban_sdk::{
        testutils::Address as _,
        token, Address, Env, Vec,
    };
    use crate::{ScholarEscrow, ScholarEscrowClient, MilestoneStatus};

    fn setup(env: &Env) -> (Address, Address, Address, ScholarEscrowClient) {
        env.mock_all_auths();

        let ngo = Address::generate(env);
        let student = Address::generate(env);

        let token_admin = Address::generate(env);
        let token_id = env.register_stellar_asset_contract(token_admin.clone());
        let token_mint = token::StellarAssetClient::new(env, &token_id);
        token_mint.mint(&ngo, &1000);

        let contract_id = env.register_contract(None, ScholarEscrow);
        let client = ScholarEscrowClient::new(env, &contract_id);

        (ngo, student, token_id, client)
    }

    // Test 1 — Happy path: full MVP flow works end-to-end
    #[test]
    fn test_happy_path_full_flow() {
        let env = Env::default();
        let (ngo, student, token_id, client) = setup(&env);
        let token_client = token::Client::new(&env, &token_id);

        let mut amounts = Vec::new(&env);
        amounts.push_back(300_i128);
        amounts.push_back(700_i128);

        client.initialize(&ngo, &student, &token_id, &amounts);
        client.submit_milestone(&student, &0u32);
        client.approve_milestone(&ngo, &0u32);

        assert_eq!(token_client.balance(&student), 300);
    }

    // Test 2 — Edge case: unauthorized address cannot approve
    #[test]
    #[should_panic(expected = "unauthorized")]
    fn test_unauthorized_approval_fails() {
        let env = Env::default();
        let (ngo, student, token_id, client) = setup(&env);

        let mut amounts = Vec::new(&env);
        amounts.push_back(500_i128);

        client.initialize(&ngo, &student, &token_id, &amounts);
        client.submit_milestone(&student, &0u32);

        let attacker = Address::generate(&env);
        client.approve_milestone(&attacker, &0u32);
    }

    // Test 3 — State verification: storage reflects correct state after approval
    #[test]
    fn test_state_after_approval() {
        let env = Env::default();
        let (ngo, student, token_id, client) = setup(&env);

        let mut amounts = Vec::new(&env);
        amounts.push_back(400_i128);
        amounts.push_back(600_i128);

        client.initialize(&ngo, &student, &token_id, &amounts);
        client.submit_milestone(&student, &0u32);
        client.approve_milestone(&ngo, &0u32);

        let state = client.get_state();

        assert_eq!(state.milestones.get(0).unwrap().status, MilestoneStatus::Approved);
        assert_eq!(state.milestones.get(1).unwrap().status, MilestoneStatus::Pending);
        assert_eq!(state.total_released, 400);
    }

    // Test 4 — Double submission blocked
    #[test]
    #[should_panic(expected = "already submitted")]
    fn test_double_submission_fails() {
        let env = Env::default();
        let (ngo, student, token_id, client) = setup(&env);

        let mut amounts = Vec::new(&env);
        amounts.push_back(500_i128);

        client.initialize(&ngo, &student, &token_id, &amounts);
        client.submit_milestone(&student, &0u32);
        client.submit_milestone(&student, &0u32);
    }

    // Test 5 — Wrong student cannot submit milestones
    #[test]
    #[should_panic(expected = "unauthorized")]
    fn test_wrong_student_cannot_submit() {
        let env = Env::default();
        let (ngo, student, token_id, client) = setup(&env);

        let mut amounts = Vec::new(&env);
        amounts.push_back(500_i128);

        client.initialize(&ngo, &student, &token_id, &amounts);

        let impostor = Address::generate(&env);
        client.submit_milestone(&impostor, &0u32);
    }
}
