#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_vote_and_pay() {
        let env = Env::default();
        let contract_id = env.register_contract(None, MyDAOContract);
        
        let account = BytesN::from_array(&env, &[0; 32]);

        MyDAOContract::vote(&env, account.clone());
        assert_eq!(MyDAOContract::get_vote_count(&env, account.clone()), 1);

        MyDAOContract::pay_account(&env, account.clone(), 100).unwrap();
        assert!(MyDAOContract::get_fund_pool(&env) < 1000);
        assert!(MyDAOContract::get_donor_pool(&env) > 0);
        assert!(MyDAOContract::get_dev_team_pool(&env) > 0);
    }
}
