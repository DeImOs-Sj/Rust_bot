use slack::api::{Message, MessageStandard};
use slack::RtmClient;

fn main() {
    // Replace "YOUR_BOT_TOKEN" with your actual Slack bot token.
    let bot_token = "xoxb-5895096432432-5895113839584-KoyNljkVfTtXZ3Y2SpbOusxr";

    let mut client = RtmClient::new(bot_token);

    match client {
        Ok(mut rtm_client) => {
            rtm_client.connect().unwrap();

            loop {
                match rtm_client.recv() {
                    Ok(event) => {
                        match event {
                            slack::Event::Message(message) => {
                                if let Some(text) = message.text {
                                    if text.contains("/age") {
                                        if let Some(channel_id) = message.channel {
                                            // Parse the year of birth from the message content.
                                            if let Some(year_str) = text.split_whitespace().nth(1) {
                                                if let Ok(year_of_birth) = year_str.parse::<i32>() {
                                                    // Calculate age and send a response.
                                                    let current_year = 2023;
                                                    let age = current_year - year_of_birth;
                                                    let response = format!("Your age is {}", age);

                                                    // Send the response back to the same channel.
                                                    rtm_client
                                                        .sender()
                                                        .send_message(&channel_id, &response)
                                                        .unwrap();
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    Err(_) => {}
                }
            }
        }
        Err(err) => {
            eprintln!("Error: {:?}", err);
        }
    }
}
