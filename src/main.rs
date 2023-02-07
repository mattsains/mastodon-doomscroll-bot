use std::{sync::Arc, env};

use azure_identity::DefaultAzureCredentialBuilder;
use azure_security_keyvault::SecretClient;
use rand::seq::SliceRandom;
use uuid::Uuid;



#[tokio::main]
async fn main() {

    println!("Hello, world!");
    let access_token = get_secret("accesstoken").await;

    let client = reqwest::Client::new();

    let id = Uuid::new_v4().as_simple().to_string();

    let message = generate_message();

    println!("Chosen to post '{}'", message);

    let res = client.post("https://masto.ai/api/v1/statuses")
        .bearer_auth(access_token)
        .header("Idempotency-Key", id)
        .form(&[("status", message)])
        .send()
        .await.unwrap();

    println!("{:#?}", res);

    if !res.status().is_success() {
        panic!("Request failed with {}", res.status())
    }
}

const MESSAGES: &'static [&'static str] = &[
    "Have you spent too much time on social media today? Scrolling can be a way to self-sooth, but try something more uplifting, like <ACTIVITY>.",
    "Our free time is precious. Consider <ACTIVITY> instead of using social media.",
    "Do you devote too much time to scrolling Mastodon? Why don't you try <ACTIVITY> instead?",
    "Hello, I know you are doing your best. Why don't you try <ACTIVITY> instead of scrolling on here?",
    "Is scrolling on Mastodon making your life better? Maybe <ACTIVITY> would be better.",
    "Sometimes I use social media when I'm feeling disconnected from the world. Maybe you could make plans with someone, or try <ACTIVITY>"
];

const BETTER_ACTIVITIES: &'static [&'static str] = &[
    "reading a book",
    "texting a friend",
    "making yourself a healthy snack",
    "drinking some water",
    "reflecting on your day or planning for tomorrow",
    "going on a walk",
    "planning a trip",
    "going to a park"
];

fn generate_message() -> String {
    let mut message = String::from(*MESSAGES.choose(&mut rand::thread_rng()).unwrap());
    if message.contains("<ACTIVITY>") {
        let activity = BETTER_ACTIVITIES.choose(&mut rand::thread_rng()).unwrap();
        message = message.replace("<ACTIVITY>", activity);
    }
    message
}

async fn get_secret(secret_name: &str) -> String {
    let keyvault_url = env::var("KEYVAULT_URL").expect("KEYVAULT_URL env variable not set!");
    let creds = Arc::new(DefaultAzureCredentialBuilder::new().build());
    let client = SecretClient::new(&keyvault_url, creds).unwrap();
    client.get(secret_name).await.unwrap().value
}