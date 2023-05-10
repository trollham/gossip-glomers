use std::io::{self, Write};

use gossip_glomers::proto::Message;

fn main() {
    let input = io::stdin().lock();
    let mut output = io::stdout().lock();

    let mut input = serde_json::Deserializer::from_reader(input).into_iter::<Message>();

    while let Some(msg) = input.next() {
        match msg {
            Ok(mut msg) => {
                let serialized_resp = match msg.body.r#type.as_str() {
                    "echo" => {
                        msg.body.r#type = "echo_ok".to_string();

                        msg.body.in_reply_to = msg.body.msg_id;
                        msg.body.msg_id = msg.body.msg_id.map(|i| i + 1);

                        std::mem::swap(&mut msg.src, &mut msg.dest);

                        serde_json::to_string(&msg).unwrap()
                    }
                    "init" => {
                        msg.body.r#type = "init_ok".to_string();

                        msg.body.in_reply_to = msg.body.msg_id;
                        msg.body.msg_id = msg.body.msg_id.map(|i| i + 1);
                        std::mem::swap(&mut msg.src, &mut msg.dest);

                        serde_json::to_string(&msg).unwrap()
                    }
                    _ => continue,
                };
                let resp_buf = serialized_resp.as_bytes();
                output
                    .write_all(resp_buf)
                    .expect("failed to write to stdout");
                output
                    .write(b"\n")
                    .expect("Come on, how can you fail here? It's just a newline!");
            }
            Err(e) => eprintln!("unrecognized message: {e:?}"),
        }
    }
}
