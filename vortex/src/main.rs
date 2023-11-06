use std::io::BufRead;

fn main() {
    eprintln!("oh boy!");
    let stdin = std::io::stdin().lock();
    let mut r = std::io::BufReader::new(stdin);

    let mut line = String::new();

    let mut id = None;
    let mut msg_id = Some(1);

    while r.read_line(&mut line).expect("read_line to work") > 0 {
        eprintln!("{:?}", &line);
        let msg = maelstrom::Message::decode(&line);
        eprintln!("{:?}", &msg);

        match &msg.body.payload {
            maelstrom::Payload::Init { node_id, .. } => {
                id = Some(node_id.clone());

                let resp = maelstrom::Message {
                    src: node_id.clone(),
                    dest: msg.src,
                    body: maelstrom::Body {
                        id: msg_id,
                        in_reply_to: msg.body.id,
                        payload: maelstrom::Payload::InitOk,
                    },
                };

                println!("{}", resp.encode());
            }
            maelstrom::Payload::InitOk => {}
            maelstrom::Payload::Echo { echo } => {
                let resp = maelstrom::Message {
                    src: id.clone().unwrap(),
                    dest: msg.src,
                    body: maelstrom::Body {
                        id: msg_id,
                        in_reply_to: msg.body.id,
                        payload: maelstrom::Payload::EchoOk { echo: echo.clone() },
                    },
                };

                println!("{}", resp.encode());
            }
            maelstrom::Payload::EchoOk { .. } => {}
        }

        line = String::new();
        msg_id = msg_id.map(|i| i + 1);
    }
}
