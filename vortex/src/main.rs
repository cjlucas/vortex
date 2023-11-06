fn main() {
    let mut node = maelstrom::Node::start();
    let mut generated_id = 0;

    while let Some(msg) = node.next_message() {
        match &msg.body.payload {
            maelstrom::Payload::Init { .. } => {
                unreachable!("should have already received init");
            }
            maelstrom::Payload::InitOk => {
                unreachable!()
            }
            maelstrom::Payload::Echo { echo } => {
                node.reply(&msg, maelstrom::Payload::EchoOk { echo: echo.clone() })
            }
            maelstrom::Payload::EchoOk { .. } => {}
            maelstrom::Payload::Generate => {
                node.reply(
                    &msg,
                    maelstrom::Payload::GenerateOk {
                        id: format!("{}:{}", node.id.clone(), generated_id),
                    },
                );

                generated_id += 1;
            }
            maelstrom::Payload::GenerateOk { .. } => {}
        }
    }
}
