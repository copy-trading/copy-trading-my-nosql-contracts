use crate::{CopyTradingGroupModel, TradingEngineSettingsModel};

service_sdk::macros::use_my_no_sql_entity!();

#[enum_of_my_no_sql_entity(table_name:"ct-product-settings", generate_unwraps)]
pub enum ProductSettings {
    TradingEngineSettings(TradingEngineSettingsModel),
    CopyTradingGroups(CopyTradingGroupModel),
}
