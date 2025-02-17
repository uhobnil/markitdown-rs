use base64::prelude::*;
use rig::{
    agent::Agent,
    completion::{CompletionModel, Prompt},
    message::{ContentFormat, ImageMediaType, Message, UserContent},
    providers::{
        deepseek,
        gemini::{self, completion::gemini_api_types::GenerationConfig},
        openai,
    },
    OneOrMany,
};

pub async fn get_llm_description(
    local_path: &str,
    llm_client: &str,
    llm_model: &str,
) -> Option<String> {
    match llm_client {
        "gemini" => get_llm_description_by_gemini(local_path, llm_model).await,
        "openai" => get_llm_description_by_openai(local_path, llm_model).await,
        "deepseek" => get_llm_description_by_deepseek(local_path, llm_model).await,
        _ => {
            println!("Unsupported llm_client: {}", llm_client);
            return None;
        }
    }
}

async fn get_answer(agent: Agent<impl CompletionModel>, local_path: &str) -> Option<String> {
    let image = std::fs::read(local_path).unwrap();
    let image_base64 = BASE64_STANDARD.encode(image);

    let mut content_items = OneOrMany::one(UserContent::image(
        image_base64,
        Some(ContentFormat::Base64),
        Some(ImageMediaType::JPEG),
        None,
    ));

    content_items.push(UserContent::text(
        "Write a detailed caption for this image.",
    ));

    let message = Message::User {
        content: content_items,
    };

    let response = agent.prompt(message).await;

    match response {
        Ok(response) => {
            let response_str = response.to_string();
            Some(response_str)
        }
        Err(e) => {
            println!("Error: {}", e);
            None
        }
    }
}

async fn get_llm_description_by_gemini(local_path: &str, llm_model: &str) -> Option<String> {
    let params = serde_json::to_value(GenerationConfig {
        top_k: Some(1),
        top_p: Some(0.95),
        candidate_count: Some(1),
        ..Default::default()
    })
    .unwrap();

    let client = gemini::Client::from_env();

    let agent = client
        .agent(llm_model)
        .preamble("You are an image describer.")
        .temperature(0.5)
        .additional_params(params)
        .build();

    get_answer(agent, local_path).await
}

async fn get_llm_description_by_openai(local_path: &str, llm_model: &str) -> Option<String> {
    let params = serde_json::to_value(GenerationConfig {
        top_k: Some(1),
        top_p: Some(0.95),
        candidate_count: Some(1),
        ..Default::default()
    })
    .unwrap();

    let client = openai::Client::from_env();

    let agent = client
        .agent(llm_model)
        .preamble("You are an image describer.")
        .temperature(0.5)
        .additional_params(params)
        .build();

    get_answer(agent, local_path).await
}

async fn get_llm_description_by_deepseek(local_path: &str, llm_model: &str) -> Option<String> {
    let params = serde_json::to_value(GenerationConfig {
        top_k: Some(1),
        top_p: Some(0.95),
        candidate_count: Some(1),
        ..Default::default()
    })
    .unwrap();

    let client = deepseek::Client::from_env();

    let agent = client
        .agent(llm_model)
        .preamble("You are an image describer.")
        .temperature(0.5)
        .additional_params(params)
        .build();

    get_answer(agent, local_path).await
}
