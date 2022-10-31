use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct UserDetails {
    pub first_name: String,
    pub last_name: String,
}

pub const USERDETAILS: Item<UserDetails> = Item::new("add_user");
