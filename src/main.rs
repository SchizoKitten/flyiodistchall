use std::io::{stdin, stdout};
use flyiodistchall::Node;

fn main() {
    let input = stdin();
    let output = stdout();
    let mut n = Node::new(input, output);
    let key: &'static str = "echo";
    n.handler(key, Box::new(|message|{
        message.set_type("echo_ok".to_string());
        let body = message.get_body_mut_ref();
        body.insert(
            "in_reply_to".to_string(),
            body.get("msg_id").unwrap().to_string());
        body.remove("msg_id");
    }));
    n.run();
}
