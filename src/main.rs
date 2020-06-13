use futures::TryStreamExt;

#[derive(Debug)]
enum Error {
    BFError(heliometer::Error),
    ParseError(std::string::FromUtf8Error),
    JoinError(tokio::task::JoinError),
}

#[tokio::main]
async fn main() -> Result<(), noob::Error> {
    let token = std::env::var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN");

    let (client, stream) = noob::Client::connect(&token).await?;
    let client = std::sync::Arc::new(client);
    stream
        .try_for_each(|evt| {
            if let noob::Event::MessageCreate(msg) = evt {
                let content = msg.content;
                let author = msg.author;
                let channel_id = msg.channel_id;
                if content.starts_with("%bf ") {
                    let client = client.clone();
                    tokio::spawn(async move {
                        let result = flatten_result(
                            tokio::task::spawn_blocking(move || {
                                let program = &content[4..];

                                let mut output = Vec::new();
                                let mut input: &[u8] = &[];

                                heliometer::execute(program, &mut input, &mut output)
                                    .map_err(|e| Error::BFError(e))
                                    .and(
                                        String::from_utf8(output).map_err(|e| Error::ParseError(e)),
                                    )
                            })
                            .await
                            .map_err(Error::JoinError),
                        );

                        let res = client
                            .send_message(
                                &noob::MessageBuilder::new(&format!(
                                    "<@{}>: {}",
                                    author.id,
                                    match result {
                                        Ok(output) => output,
                                        Err(err) => format!("Error: {:?}", err),
                                    },
                                )),
                                &channel_id,
                            )
                            .await;

                        if let Err(e) = res {
                            eprintln!("Failed something {:?}", e);
                        }
                    });
                }
            }

            futures::future::ready(Ok(()))
        })
        .await
        .unwrap();

    Ok(())
}

fn flatten_result<T, E>(res: Result<Result<T, E>, E>) -> Result<T, E> {
    match res {
        Err(e) => Err(e),
        Ok(Err(e)) => Err(e),
        Ok(v) => v,
    }
}
