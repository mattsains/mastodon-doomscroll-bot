import { SecretsManagerClient, GetSecretValueCommand } from "@aws-sdk/client-secrets-manager";

export const post = async (event, context) => {
    console.log("Starting post")
    const accessKey = await getSecret();

    const d = new Date();
    const dedupeId = d.getFullYear() + "-" + (d.getMonth()+1) + "-" + d.getDate() + "-" + d.getHours();
    const message = generate_message();

    console.log("Dedupe ID:", dedupeId);
    console.log("Decided to post:", message);

    const data = new URLSearchParams();
    data.append("status", message);

    const response = await fetch("https://masto.ai/api/v1/statuses", {
        method: 'post',
        headers: {
            "Authorization": "Bearer " + accessKey,
            "Idempotency-Key": dedupeId,
        },
        body: data,
    });

    console.log("Response:", await response.text())

    if (response.status !== 200) {
        throw new Error("Mastodon request failed.");
    }
}

const MESSAGES = [
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
    "Make sure you control the apps you use and not the other way around.",
    "The next scroll won't change the world—but taking action might! Go do something that lifts you up, like <ACTIVITY>.",
    "Social media will be here when you get back. Why not try <ACTIVITY> before you return?",
    "If the news feels overwhelming, take a walk, call a friend, or try <ACTIVITY>.",
    "You control where your attention goes - why not put it toward something that nourishes you? Like <ACTIVITY>.",
    "The world is waiting for you. Put down your phone and try <ACTIVITY>.",
    "The world is happening right now. Step outside and be a part of it!",
    "Social media can wait - your happiness can't. Go do something that makes you smile.",
    "Your life isn't measured in likes. Make today about real experiences instead.",
    "Trade doomscrolling for daydreaming - what's something wonderful you could plan for yourself?",
    "The best stories aren't found online - they're made in real life. Go make one today!"
];

const BETTER_ACTIVITIES = [
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
    "getting ice cream",
    "calling a friend and have a real conversation",
    "finding a new recipe and cooking something delicious",
    "writing down five things you're grateful for",
    "picking up a craft project and creating something with your hands",
    "taking a minute to stretch, breathe, and give your body some love",
    "visiting a local park and enjoying nature",
    "journaling your own thoughts instead of scrolling through those of others",
];

const generate_message = () => {
    let message = MESSAGES[Math.floor(Math.random()*MESSAGES.length)];
    if (message.includes("<ACTIVITY>")) {
        let activity = BETTER_ACTIVITIES[Math.floor(Math.random()*BETTER_ACTIVITIES.length)];
        message = message.replaceAll("<ACTIVITY>", activity);
    }
    return message
}

const getSecret = async () => {
    const client = new SecretsManagerClient({ region: "us-east-1" });
    const command = new GetSecretValueCommand({
        SecretId: process.env['SECRET_ID'],
    });
    const result = await client.send(command);

    if (!result.SecretString) {
        throw new Error("Access key not retrieved successfully")
    }
    
    return result.SecretString;
}