use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[enum_model(partition_key:"trading-engine", row_key: "trading-groups")]
#[derive(Serialize, Deserialize, Clone)]
pub struct TradingEngineSettingsModel {
    pub provider_trading_group: String,
    pub subscriber_trading_group: String,
    pub min_volume_copy: Option<f64>,
}

impl TradingEngineSettingsModel {
    pub fn get_min_volume_copy(&self) -> f64 {
        self.min_volume_copy.unwrap_or(0.1)
    }
}
