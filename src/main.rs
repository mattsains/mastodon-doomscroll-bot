use std::{sync::Arc, env};

use azure_identity::DefaultAzureCredentialBuilder;
use azure_security_keyvault::SecretClient;
use rand::seq::SliceRandom;



#[tokio::main]
async fn main() {

    println!("Hello, world!");
    let access_token = get_secret("accesstoken").await;

    let client = reqwest::Client::new();

    let date = chrono::offset::Utc::now();
    let dedupe_key = date.format("%Y-%m-%d-%H").to_string(); // this prevents more than one post an hour.

    let message = generate_message();

    println!("Chosen to post '{}'", message);

    let res = client.post("https://masto.ai/api/v1/statuses")
        .bearer_auth(access_token)
        .header("Idempotency-Key", dedupe_key)
        .form(&[("status", message)])
        .send()
        .await.unwrap();

    println!("{:#?}", res);

    if !res.status().is_success() {
        panic!("Request failed with {}", res.status())
    }
    println!("The body was:");
    println!("{}", res.text().await.unwrap_or(String::from("<no body>")));
    println!("Goodbye");
}

const MESSAGES: &'static [&'static str] = &[
    "Have you spent too much time on social media today? Scrolling can be a way to self-soothe, but try something more uplifting, like <ACTIVITY>.",
    "Our free time is precious. Consider <ACTIVITY> instead of using social media.",
    "Do you devote too much time to scrolling Mastodon? Why don't you try <ACTIVITY> instead?",
    "Hello, I know you are doing your best. Why don't you try <ACTIVITY> instead of scrolling on here?",
    "Is scrolling on Mastodon making your life better? Maybe <ACTIVITY> would be better.",
    "Sometimes I use social media when I'm feeling disconnected from the world. Maybe you could make plans with someone, or try <ACTIVITY>",
    "Scrolling social media can give us a false sense of agency in our own lives. Why don't you exercise some real agency by <ACTIVITY>",
    "There are other things to experience in life other than this app. Why don't you try <ACTIVITY> instead",
    "I made this account because it's too easy for me to spend hours scrolling on here. Maybe it's the same for you. I think that our lives are better spent on other things, like <ACTIVITY>.",
    "Very little will come from spending your time on social media. You might get more out of <ACTIVITY>.",
    "Hello, I'm here to remind you to stop using social media excessively. <ACTIVITY> might be a better use of time.",
    "Leave social media for the politicians and the chronically angry. You might have a better time <ACTIVITY>.",
    "There's nothing wrong with using this app, but have you considered <ACTIVITY> instead?",
    "Hey! Maybe you could take a break?",
    "Make sure you control the apps you use and not the other way around."
];

const BETTER_ACTIVITIES: &'static [&'static str] = &[
    "reading a book",
    "texting a friend",
    "making yourself a healthy snack",
    "drinking some water",
    "reflecting on your day or planning for tomorrow",
    "going on a walk",
    "planning a trip",
    "going to a park",
    "doing some tidying around the house",
    "writing down five things that are going well",
    "closing your eyes and focusing on your breathing for a minute",
    "getting some exercise for fun",
    "doing a craft project",
    "going out and buying yourself a small treat",
    "getting ice cream"
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