use cosmwasm_std::Addr;
use cw_storage_plus::Map;

pub const BLOCKED: Map<&Addr, bool> = Map::new("blocked");
