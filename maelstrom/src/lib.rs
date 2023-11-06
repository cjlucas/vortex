use std::io::BufRead;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message {
    pub src: String,
    pub dest: String,
    pub body: Body,
}

impl Message {
    pub fn decode(json: &str) -> Self {
        serde_json::from_str(json).unwrap()
    }

    pub fn encode(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Body {
    #[serde(rename = "msg_id")]
    pub id: Option<u64>,
    pub in_reply_to: Option<u64>,

    #[serde(flatten)]
    pub payload: Payload,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Payload {
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk,
    Echo {
        echo: String,
    },
    EchoOk {
        echo: String,
    },
    Generate,
    GenerateOk {
        id: String,
    },
}

pub struct Node<'a> {
    // id of the node
    pub id: String,
    msg_id: u64,
    input: std::io::BufReader<std::io::StdinLock<'a>>,
}

impl<'a> Node<'a> {
    pub fn start() -> Self {
        let stdin = std::io::stdin().lock();
        let mut input = std::io::BufReader::new(stdin);

        let mut line = String::new();
        let _ = input.read_line(&mut line).expect("read line should work");

        let msg = Message::decode(&line);

        if let Payload::Init { node_id, .. } = &msg.body.payload {
            let mut s = Self {
                id: node_id.clone(),
                msg_id: 0,
                input,
            };
            s.reply(&msg, Payload::InitOk);

            s
        } else {
            unreachable!("Expected init, received {:?}", msg.body.payload);
        }
    }

    pub fn next_message(&mut self) -> Option<Message> {
        let mut line = String::new();
        let bytes_read = self
            .input
            .read_line(&mut line)
            .expect("read line should work");
        if bytes_read == 0 {
            return None;
        }

        Some(Message::decode(&line))
    }

    pub fn reply(&mut self, msg: &Message, payload: Payload) {
        let reply = Message {
            src: self.id.clone(),
            dest: msg.src.clone(),
            body: Body {
                id: Some(self.msg_id),
                in_reply_to: msg.body.id,
                payload,
            },
        };

        println!("{}", reply.encode());

        self.msg_id += 1;
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_thing() {
        // let body = Body {
        //     id: None,
        //     in_reply_to: None,
        //     payload: Payload::Init {
        //         node_id: String::from("n1"),
        //         node_ids: vec![String::from("n1"), String::from("n2"), String::from("n3")],
        //     },
        // };
        // assert!(false);
    }
}
