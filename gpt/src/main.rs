use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenAIRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub stream: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenAIResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub usage: Option<Usage>,
    pub choices: Vec<Choice>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Usage {
    #[serde(rename = "prompt_tokens")]
    pub prompt_tokens: i64,
    #[serde(rename = "completion_tokens")]
    pub completion_tokens: i64,
    #[serde(rename = "total_tokens")]
    pub total_tokens: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Choice {
    pub delta: Option<Delta>,
    pub message: Option<Message>,
    #[serde(rename = "finish_reason")]
    pub finish_reason: Option<String>,
    pub index: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Delta {
    pub content: Option<String>,
    pub role: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let uri = "https://api.openai.com/v1/chat/completions";

    let request = OpenAIRequest {
        model: "gpt-3.5-turbo".to_string(),
        messages: vec![Message {
            role: "user".to_string(),
            content: "Hello, how are you?".to_string(),
        }],
        stream: true,
    };

    // Convert the HashMap to a JSON object
    let json_data = json!(request);

    let mut stream = client
        .post(uri)
        .header("Content-Type", "application/json")
        .header(
            "Authorization",
            "Bearer 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",
        )
        .json(&json_data)
        .send()
        .await?
        .bytes_stream();

    let mut total_string = String::new();

    while let Some(item) = stream.next().await {
        match item {
            Ok(chunk) => {
                println!("Chunk: {:?}", chunk.slice(6..));
                if chunk.slice(6..).eq(&"[DONE]\n\n".to_string()) {
                    println!("Done");
                    break;
                }
                let resp = serde_json::from_slice::<OpenAIResponse>(&chunk.slice(6..));
                if resp.is_err() {
                    println!("Error: {:?}", resp.err());
                    continue;
                }
                for c in resp.unwrap().choices {
                    if let Some(delta) = c.delta {
                        if let Some(content) = delta.content {
                            total_string.push_str(content.as_str());
                        }
                    }
                }

                println!("Total string:{}", total_string);
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }

    Ok(())
}
