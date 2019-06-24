extern crate heliometer;
extern crate serenity;

#[derive(Debug)]
enum Error {
    BFError(heliometer::Error),
    ParseError(std::string::FromUtf8Error),
}

struct Handler;

impl serenity::client::EventHandler for Handler {
    fn message(&self, ctx: serenity::client::Context, msg: serenity::model::channel::Message) {
        if msg.content.starts_with("%bf ") {
            let program = &msg.content[4..];
            let mut output = Vec::new();
            let mut input: &[u8] = &[];
            let result = heliometer::execute(program, &mut input, &mut output)
                .map_err(|e| Error::BFError(e));
            let output = String::from_utf8(output).map_err(|e| Error::ParseError(e));
            match match result.and(output) {
                Ok(ref output) => msg.reply(&ctx, &output),
                Err(err) => msg.reply(&ctx, &format!("Error: {:?}", err)),
            } {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Failed something: {}", e);
                }
            }
        }
    }
}

fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN");

    let mut client = serenity::Client::new(&token, Handler).expect("Error creating client");

    client.start().unwrap();
}
