extern crate slack;
extern crate chrono;

use chrono::Utc;
use chrono::NaiveDate;
use slack::{Event, EventHandler, Message, RtmClient};

pub struct Handler;

#[allow(unused_variables)]
impl EventHandler for Handler {
    fn on_event(&mut self, cli: &RtmClient, event: Event) {
        println!("on_event(event: {:?})", event);

        match event.clone() {
            Event::Message(message) => self.handle_message(*message, cli, &event),
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

#[allow(unused_variables)]
impl Handler {
    fn handle_message(&mut self, message: Message, cli: &RtmClient, _event: &Event) {
        let message_standard = match message {
            Message::Standard(message_standard) => message_standard,
            _ => return,
        };
        let channel: String = message_standard.channel.unwrap();
        let user_id: String = message_standard.user.unwrap();
        let text: String = message_standard.text.unwrap();

        if !text.contains(&cli.start_response().slf.as_ref().unwrap().id.as_ref().unwrap()) {
            println!("Is not a mention");
            return;
        }

        let current_year = Utc::now().year();
        let birth_year = match extract_year_of_birth(&text) {
            Some(year) => year,
            None => {
                println!("Invalid year of birth format");
                return;
            }
        };

        let age = current_year - birth_year;

        let response = format!("Your age is {} years.", age);
        send_message(&user_id, &response, &channel, &cli);
    }
}

fn extract_year_of_birth(text: &str) -> Option<i32> {
    let words: Vec<&str> = text.split_whitespace().collect();

    for word in words {
        if let Ok(year) = word.parse::<i32>() {
            if year >= 1900 && year <= Utc::now().year() {
                return Some(year);
            }
        }
    }

    None
}

fn send_message(user_id: &str, text: &str, channel: &str, cli: &RtmClient) {
    let message = cli.send_message(channel, text);
    if let Err(err) = message {
        println!("Error sending message: {:?}", err);
    }
}
