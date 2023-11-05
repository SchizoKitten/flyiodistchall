use std::{io::{stdin, stdout}, collections::HashMap};
use flyiodistchall::Node;

fn main() {
    let input = stdin();
    let output = stdout();
    let mut n = Node::new(input, output);
    let key: &'static str = "\"echo\"";
    n.handler(key, Box::new(|message|{
        let new_body = message.get_body_mut_ref();
        *new_body.get_mut("type").unwrap() = "\"echo_ok\"".to_string();
        new_body.insert(
            "in_reply_to".to_string(),
            new_body.get("msg_id").unwrap().to_string());
        new_body.remove("msg_id");
    }));
    let key: &'static str = "\"generate\"";
    let mut counter = 0;
    n.handler(key, Box::new(move |message|{
        let mut new_body: HashMap<String, String> = HashMap::new();
        new_body.insert("type".to_string(), "\"generate_ok\"".to_string());
        new_body.insert("id".to_string(),
            format!("\"{}{}\"", message.get_dest(), counter));
        new_body.insert(
            "in_reply_to".to_string(),
            message.get_body_ref().get("msg_id").unwrap().to_string());
        counter += 1;
        *message.get_body_mut_ref() = new_body;
    }));
    n.run();
}
