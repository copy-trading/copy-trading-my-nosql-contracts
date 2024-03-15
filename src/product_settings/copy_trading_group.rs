use serde::*;
service_sdk::macros::use_my_no_sql_entity!();

#[enum_model(partition_key:"trading-group")]
#[derive(Serialize, Deserialize, Clone)]
pub struct CopyTradingGroup {
    pub name: String,
    pub subscriber_commission: f64,
    pub provider_commission: f64,
}
