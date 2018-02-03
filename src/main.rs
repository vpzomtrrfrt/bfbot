extern crate serenity;
extern crate heliometer;
extern crate memstream;

#[derive(Debug)]
enum Error {
    BFError(heliometer::Error),
    ParseError(std::string::FromUtf8Error)
}

struct Handler;

impl serenity::client::EventHandler for Handler {
    fn message(&self, _: serenity::client::Context, msg: serenity::model::channel::Message) {
        if &msg.content[..4] == "%bf " {
            let program = &msg.content[4..];
            let mut stream = memstream::MemStream::new();
            let mut input: &[u8] = &[];
            let result = heliometer::execute(program, &mut input, &mut stream).map_err(|e|Error::BFError(e));
            let output = String::from_utf8(stream.unwrap()).map_err(|e|Error::ParseError(e));
            match match result.and(output) {
                Ok(ref output) => {
                    msg.reply(&output)
                },
                Err(err) => msg.reply(&format!("Error: {:?}", err))
            } {
                Ok(_) => {},
                Err(e) => {eprintln!("Failed something: {}", e);}
            }
        }
    }
}

fn main() {
    let token = std::env::var("DISCORD_TOKEN")
        .expect("Missing DISCORD_TOKEN");

    let mut client = serenity::Client::new(&token, Handler)
        .expect("Error creating client");

    client.start().unwrap();
}
