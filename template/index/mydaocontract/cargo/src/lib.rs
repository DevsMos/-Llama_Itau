use soroban_sdk::{contractimpl, Env, Symbol, Map, BytesN};

pub struct MyDAOContract;

#[derive(Default)]
pub struct Data {
    fund_pool: u64,
    donor_pool: u64,
    dev_team_pool: u64,
    vote_count: Map<BytesN<32>, u32>, // Mapeia contas para o número de votos
}
