use serde::*;
service_sdk::macros::use_my_no_sql_entity!();

#[enum_model(partition_key:"trading-group")]
#[derive(Serialize, Deserialize, Clone)]
pub struct CopyTradingGroupModel {
    pub name: String,
    pub commission_to_provider: f64,
    pub commission_to_ct_service: f64,
}
