extern crate chrono;
extern crate env_logger;
extern crate itertools;
#[macro_use]
extern crate lalrpop_util;
extern crate log;
extern crate matrix_bot_api;
extern crate rand;

use log::info;
use matrix_bot_api::handlers::{HandleResult, StatelessHandler};
use matrix_bot_api::{MatrixBot, MessageType};

mod die;
lalrpop_mod!(parser); // synthesized by LALRPOP

struct Config {
    /// The user you have created for this bot.
    user: &'static str,

    /// The password for that user.
    password: &'static str,

    /// Where the bot is registered.
    homeserver_url: &'static str,
}

fn main() {
    env_logger::init();
    info!(target: "mariabot", "Starting");
    let mut handler = StatelessHandler::new();
    handler.register_handle("echo", |bot, message, tail| {
        info!(target: "mariabot", "Received echo: {}", tail);
        bot.send_message(
            &format!("Echo: {}", tail),
            &message.room,
            MessageType::TextMessage,
        );
        HandleResult::StopHandling
    });

    handler.register_handle("roll", |bot, message, tail| {
        info!(target: "mariabot", "Received roll: {}, took {}ms to receive", tail, (chrono::Local::now() - message.date).num_milliseconds() );
        match parser::OperationParser::new().parse(tail) {
            Err(err) => {
                info!(target: "mariabot", "Roll error: {:?}", err);
                bot.send_message(&format!("Error: {:?}", err), &message.room, MessageType::TextMessage);        
            },
            Ok(expr) => {
                info!(target: "mariabot", "Roll parsed: {:?}", expr);
                let value = expr.roll(&mut rand::thread_rng());
                bot.send_message(&format!("Rolled: {:}", value), &message.room, MessageType::TextMessage);        
            }
        }
        HandleResult::StopHandling
    });

    let bot = MatrixBot::new(handler);

    // If you build this, you'll need to supply this file.
    let config = include!("../.bot.config");
    bot.run(
        config.user,
        config.password,
        config.homeserver_url,
    );
}
