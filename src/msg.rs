use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub first_name: String,
    pub last_name: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    UserDetails {
        first_name: String,
        last_name: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
#[allow(non_camel_case_types)]
pub enum QueryMsg {
    #[returns(UserDetails)]
    userdetails {},
}

#[cw_serde]
pub struct UserDetails {
    pub first_name: String,
    pub last_name: String,
}

// #[cw_serde]
// pub struct InstantiateMsg {}

// #[cw_serde]
// pub enum ExecuteMsg {}

// #[cw_serde]
// #[derive(QueryResponses)]
// pub enum QueryMsg {}

// #[cw_serde]
// pub struct UserDetails {
//     pub first_name: String,
//     pub last_name: String,
// }
