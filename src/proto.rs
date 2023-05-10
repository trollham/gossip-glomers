use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Message {
    pub src: String,
    pub dest: String,
    pub body: Body,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Body {
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_reply_to: Option<u64>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<Value>,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{Body, Message};

    #[test]
    fn serialize_test() {
        let msg = Message {
            src: "foo".to_string(),
            dest: "bar".to_string(),
            body: Body {
                r#type: "hello_world".to_string(),
                msg_id: None,
                in_reply_to: None,
                payload: Some(json!({"hello": "world!"})),
            },
        };

        let expected = json!(
        {
            "src": "foo",
            "dest": "bar",
            "body": {
                "type": "hello_world",
                "hello": "world!"
            }
        }
        );

        assert_eq!(serde_json::to_value(&msg).unwrap(), expected)
    }

    #[test]
    fn deserialize_test() {
        let raw_json = json!({
           "src": "foo",
           "dest": "bar",
           "body": {
                "type": "hello_world",
                "hello": "world!",
           }
        });

        let expected = Message {
            src: "foo".to_string(),
            dest: "bar".to_string(),
            body: Body {
                r#type: "hello_world".to_string(),
                msg_id: None,
                in_reply_to: None,
                payload: Some(json!({
                    "hello": "world!"
                })),
            },
        };

        let val = serde_json::from_value::<Message>(raw_json).unwrap();

        assert_eq!(val, expected);
    }
}
