# mastodon-doomscroll-bot
A bot that posts helpful messages to Mastodon to stop doomscrolling.

It can be followed at [@selfcare@masto.ai](https://masto.ai/@selfcare)

It posts a reminder to get off social media every twelve hours.

Contributions welcome.

## How to run
The bot requires an Azure KeyVault with a secret called "accesstoken" which is a bearer token to post a status update on masto.ai. The KeyVault URL must be set using the environment variable "KEYVAULT_URL".
