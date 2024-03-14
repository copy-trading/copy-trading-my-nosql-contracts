use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[enum_model(partition_key:"trading-engine", row_key: "trading-groups")]
#[derive(Serialize, Deserialize, Clone)]
pub struct TradingEngineSettingsModel {
    pub provider_trading_group: String,
    pub subscriber_trading_group: String,
    pub min_volume_copy: Option<f64>,
    pub max_providers_to_register: Option<i32>,
    pub max_subscribers_to_register: Option<i32>,
}

impl TradingEngineSettingsModel {
    pub fn get_min_volume_copy(&self) -> f64 {
        self.min_volume_copy.unwrap_or(0.1)
    }

    pub fn get_max_providers_to_register(&self) -> i32 {
        self.max_providers_to_register.unwrap_or(3)
    }

    pub fn get_max_subscribers_to_register(&self) -> i32 {
        self.max_subscribers_to_register.unwrap_or(3)
    }
}
