use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Metadata {
    compute: Compute,
}

#[derive(Serialize, Deserialize, Debug)]
struct Compute {
    name: String,
    #[serde(rename = "azEnvironment")]
    environment: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct ScheduledEventResponse {
    document_incarnation: u32,
    events: Vec<Event>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Event {
    event_id: String,
    event_type: String,
    resource_type: String,
    resources: Vec<String>,
    event_status: String,
    not_before: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let res = reqwest::Client::new()
        .get("http://169.254.169.254/metadata/scheduledevents?api-version=2017-11-01")
        .header("Metadata", "true")
        .send()
        .await?
        .text()
        .await?;
    
    let data: ScheduledEventResponse = serde_json::from_str(&res[..]).unwrap();

    println!("{:#?}", data);
    Ok(())
}
 