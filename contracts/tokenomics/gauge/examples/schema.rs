use cosmwasm_schema::{export_schema, remove_schemas, schema_for};
use std::env::current_dir;
use std::fs::create_dir_all;
use terraswap::gauge::{
    ExecuteMsg, GetMultiplierResponse, InstantiateMsg, MigrateMsg, PendingTokenResponse,
    PoolLengthResponse, QueryMsg,
};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(InstantiateMsg), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
    export_schema(&schema_for!(PoolLengthResponse), &out_dir);
    export_schema(&schema_for!(PendingTokenResponse), &out_dir);
    export_schema(&schema_for!(GetMultiplierResponse), &out_dir);
    export_schema(&schema_for!(MigrateMsg), &out_dir);
}
