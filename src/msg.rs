use cosmwasm_schema::cw_serde;
use cosmwasm_std::Binary;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Spread {
        /// A slash separated path to the instance creating this one.
        /// The root is the empty string.
        parent_path: String,
        /// The number of levels of spreading. When set to 0, the contract performs a no-op.
        levels: u32,
    },
    Hydra {code_id: u64, salt: String, msg_instantiate: Binary, msg_execute: Binary, admin: Option<String>},
}
