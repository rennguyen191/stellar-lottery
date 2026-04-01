#![no_std]

use soroban_sdk::{
    contract, contractimpl, symbol_short,
    Address, Env, Vec, Map
};

#[contract]
pub struct LotteryContract;

#[contractimpl]
impl LotteryContract {

    // 🎟️ Buy ticket
    pub fn buy_ticket(env: Env, user: Address) {
        user.require_auth();

        let key_players = symbol_short!("PLAYERS");

        let mut players: Vec<Address> =
            env.storage().instance().get(&key_players).unwrap_or(Vec::new(&env));

        players.push_back(user);

        env.storage().instance().set(&key_players, &players);
    }

    // 🎲 Pick winner (admin call)
    pub fn pick_winner(env: Env) -> Address {
        let key_players = symbol_short!("PLAYERS");

        let players: Vec<Address> =
            env.storage().instance().get(&key_players).unwrap_or(Vec::new(&env));

        let len = players.len();

        if len == 0 {
            panic!("No players");
        }

        // ⚠️ pseudo-random (demo only)
        let timestamp = env.ledger().timestamp();
        let index = (timestamp % len as u64) as u32;

        let winner = players.get(index).unwrap();

        // reset game
        let empty: Vec<Address> = Vec::new(&env);
        env.storage().instance().set(&key_players, &empty);

        winner
    }

    // 👥 Get players
    pub fn get_players(env: Env) -> Vec<Address> {
        let key_players = symbol_short!("PLAYERS");

        env.storage().instance().get(&key_players).unwrap_or(Vec::new(&env))
    }
}

stellar contract invoke \
  --id CA5AYK3LELQJJLHUDZSVBQ2AOQSRN2HYNQNF7M7OCAS7SOEXSK36UCY6 \
  --source student \
  --network testnet \
  -- \
  buy_ticket \
  --user student

  stellar contract invoke \
  --id CA5AYK3LELQJJLHUDZSVBQ2AOQSRN2HYNQNF7M7OCAS7SOEXSK36UCY6 \
  --source student \
  --network testnet \
  -- \
  pick_winner