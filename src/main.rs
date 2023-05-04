/*
    PrismaCord: An administrative Discord bot.
    Copyright (C) 2023  Sylv

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::env;
use serenity::{async_trait, Client};
use serenity::prelude::{GatewayIntents, EventHandler};

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
	let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
	let intents = GatewayIntents::non_privileged();
	let mut client = Client::builder(token, intents)
		.event_handler(Handler)
		.intents(intents)
		.await
		.expect("Error creating client");
	
	if let Err(why) = client.start().await {
		println!("Client error: {:?}", why);
	}
}
