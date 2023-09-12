extern crate slack;
extern crate chrono;
extern crate dotenv;

use slack::{Event, EventHandler, Message, RtmClient};
use dotenv::dotenv;

fn main() {
    dotenv().ok();

    let api_key = dotenv::var("SLACK_BOT_TOKEN")
        .expect("SLACK_BOT_TOKEN not found in environment variables");
    
    // Read the SLACK_CHANNEL environment variable
    let channel = dotenv::var("SLACK_CHANNEL")
        .expect("SLACK_CHANNEL not found in environment variables");

    let mut handler = Handler { channel };
    let r = RtmClient::login_and_run(&api_key, &mut handler);

    match r {
        Ok(_) => {}
        Err(err) => panic!("Error: {}", err),
    }
}

pub struct Handler {
    channel: String, // Store the channel ID or name
}

impl EventHandler for Handler {
    fn on_event(&mut self, cli: &RtmClient, event: Event) {
        println!("on_event(event: {:?})", event);

        match event.clone() {
            Event::Message(message) => self.handle_message(message, cli),
            _ => return,
        };
    }

    fn on_close(&mut self, _cli: &RtmClient) {
        println!("on_close");
    }

    fn on_connect(&mut self, _cli: &RtmClient) {
        println!("on_connect");
    }
}

impl Handler {
    fn handle_message(&mut self, message: Box<Message>, cli: &RtmClient) {
        if let Message::Standard(message_standard) = *message {
            let user_id = message_standard.user.unwrap();
            let text = message_standard.text.unwrap();

            if !text.contains(&*cli.start_response().slf.as_ref().unwrap().id.as_ref().unwrap()) {
                println!("Is not a mention");
                return;
            }

            if let Some(birth_year) = extract_year_of_birth(&text) {
                let current_year = 2023;
                let age = current_year - birth_year;
                let response = format!("Your age is {} years.", age);
                
                // Use the specified channel when sending the message
                send_message(&user_id, &response, &self.channel, &cli);
            } else {
                println!("Invalid year of birth format");
            }
        }
    }
}

fn extract_year_of_birth(text: &str) -> Option<i32> {
    let words: Vec<&str> = text.split_whitespace().collect();

    for word in words {
        if let Ok(year) = word.parse::<i32>() {
            if year >= 1900 && year <= 2023 {
                return Some(year);
            }
        }
    }

    None
}

fn send_message(_user_id: &str, text: &str, channel: &str, cli: &RtmClient) {
    let message = cli.sender().send_message(channel, text);
    if let Err(err) = message {
        println!("Error sending message: {:?}", err);
    }
}
