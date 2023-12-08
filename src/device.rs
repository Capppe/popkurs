use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Device {
    pub id: u32,
    pub name: String,
    pub power: String,
    pub total_consumption: f64,
    pub consumption_data: Vec<ConsumptionData>,
}

#[derive(Debug, Deserialize)]
pub struct ConsumptionData {
    pub timestamp: String,
    pub power_usage: f64,
    pub energy_consumed: f64,
}
