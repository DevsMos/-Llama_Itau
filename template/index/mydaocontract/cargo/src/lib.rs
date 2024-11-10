use soroban_sdk::{contractimpl, Env, Symbol, Map, BytesN};

pub struct MyDAOContract;

#[derive(Default)]
pub struct Data {
    fund_pool: u64,
    donor_pool: u64,
    dev_team_pool: u64,
    vote_count: Map<BytesN<32>, u32>, // Mapeia contas para o número de votos
}

#[contractimpl]
impl MyDAOContract {
    pub fn vote(env: Env, account: BytesN<32>) {
        let mut data: Data = env.data().get().unwrap_or_default();

        // Incrementar votos para a conta
        let current_votes = data.vote_count.get(&account).unwrap_or(0);
        data.vote_count.set(account.clone(), current_votes + 1);

        // Salvar o estado atualizado
        env.data().set(data);
    }
#[contractimpl]
impl MyDAOContract {
    pub fn pay_account(env: Env, account: BytesN<32>, payment_amount: u64) -> Result<(), String> {
        let mut data: Data = env.data().get().unwrap_or_default();

        // Verificar se há fundos suficientes
        if data.fund_pool < payment_amount {
            return Err("Fundo insuficiente".into());
        }

        // Calcular taxa (5%) e distribuições
        let fee = (payment_amount as f64 * 0.05) as u64;
        let donation_share = (fee as f64 * 0.6) as u64; // 3% para doadores
        let dev_share = fee - donation_share;            // 2% para equipe de desenvolvimento

        // Atualizar pools
        data.donor_pool += donation_share;
        data.dev_team_pool += dev_share;
        data.fund_pool -= payment_amount;

        // Atualizar o estado
        env.data().set(data);

        Ok(())
    }
}

    #[contractimpl]
impl MyDAOContract {
    pub fn get_fund_pool(env: Env) -> u64 {
        let data: Data = env.data().get().unwrap_or_default();
        data.fund_pool
    }

    pub fn get_donor_pool(env: Env) -> u64 {
        let data: Data = env.data().get().unwrap_or_default();
        data.donor_pool
    }

    pub fn get_dev_team_pool(env: Env) -> u64 {
        let data: Data = env.data().get().unwrap_or_default();
        data.dev_team_pool
    }

    pub fn get_vote_count(env: Env, account: BytesN<32>) -> u32 {
        let data: Data = env.data().get().unwrap_or_default();
        data.vote_count.get(&account).unwrap_or(0)
    }
}
    #[contractimpl]
impl MyDAOContract {
    pub fn get_fund_pool(env: Env) -> u64 {
        let data: Data = env.data().get().unwrap_or_default();
        data.fund_pool
    }

    pub fn get_donor_pool(env: Env) -> u64 {
        let data: Data = env.data().get().unwrap_or_default();
        data.donor_pool
    }

    pub fn get_dev_team_pool(env: Env) -> u64 {
        let data: Data = env.data().get().unwrap_or_default();
        data.dev_team_pool
    }

    pub fn get_vote_count(env: Env, account: BytesN<32>) -> u32 {
        let data: Data = env.data().get().unwrap_or_default();
        data.vote_count.get(&account).unwrap_or(0)
    }
}



