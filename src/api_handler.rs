use reqwest;
use crate::device::Device;

pub async fn get_devices() -> Vec<Device> {
    match api_call(None).await {
        Ok(body) => deserialize(body.as_str()),
        Err(err) => Vec::new(),
    }

}

async fn api_call(url: Option<&str>) -> Result<String, reqwest::Error> { 
    let result = reqwest::get(url.unwrap_or("http://localhost:3000/devices"))
    .await?
    .text()
    .await?;

    Ok(result)
}

fn deserialize(data: &str) -> Vec<Device> {
    serde_json::from_str(data).unwrap()
}
