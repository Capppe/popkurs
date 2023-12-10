use reqwest;
use crate::device::Device;

pub async fn get_devices() -> Vec<Device> {
    match api_call(None).await {
        Ok(body) => deserialize(body.as_str()),
        Err(_err) => Vec::new(),
    }

}

pub async fn get_device(id: u32) -> Result<Device, reqwest::Error> {
    let url = "http://localhost:3000/devices/".to_owned() + &id.to_string();
    match api_call(Some(url.as_str())).await {
        Ok(body) => Ok(deserialize_one(body.as_str())),
        Err(err) => Err(err),
    }
}

pub async fn api_call(url: Option<&str>) -> Result<String, reqwest::Error> { 
    let result = reqwest::get(url.unwrap_or("http://localhost:3000/devices"))
    .await?
    .text()
    .await?;

    Ok(result)
}

fn deserialize(data: &str) -> Vec<Device> {
    serde_json::from_str(data).unwrap()
}

fn deserialize_one(data: &str) -> Device {
    serde_json::from_str(data).unwrap()
}
