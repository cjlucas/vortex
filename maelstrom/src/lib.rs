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
