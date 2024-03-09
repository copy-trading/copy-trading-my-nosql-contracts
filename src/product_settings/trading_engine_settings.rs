use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[enum_model(partition_key:"emails", row_key: "send-grid")]
#[derive(Serialize, Deserialize, Clone)]
pub struct TradingEngineSettingsModel {
    pub provider_trading_group: String,
    pub subscriber_trading_group: String,
}