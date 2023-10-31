#![allow(dead_code)]
//use std::io::{Write, BufRead};
use std::{io::{stdin, stdout, Stdout, Stdin, BufRead, Write}, collections::HashMap};

use itertools::Itertools;

struct Node{
    id: Option<usize>,
    message_count: usize,
    input: Stdin,
    output: Stdout,
    handlers: HashMap<&'static str, Box<dyn Fn(&mut HashMap<String, String>)>>,
}

impl Node{
    pub fn new(input: Stdin, output: Stdout) -> Node{
        Node{
            id: None,
            message_count: 0,
            input,
            output,
            handlers: HashMap::new(),
        }
    }

    pub fn handler(&mut self, key: &'static str, f: Box<dyn Fn(&mut HashMap<String, String>)>){
        self.handlers.insert(key, f);
    }
    
    pub fn run(&mut self){
        loop{
            //parse message into hashmap?
            let message: Vec<String> = self.input.lock()
                .lines()
                .map(|res_line| res_line.unwrap())
                .take_while_inclusive(|line|{
                    line != "}"
                })
                .collect();
            if message.len() < 2{
                continue;
            }
            let mut message = Message::new(message);
            let r#type = message.body.get("type").unwrap();
            if let Some(handle) = self.handlers.get(r#type.as_str()){
                handle(&mut message.body);
                self.send(message);
            }else{
                panic!("No handle");
            }
        }
    }

    fn send(&mut self, mut msg: Message){
        self.message_count += 1;
        msg.add("msg_id", self.message_count.to_string());
        let answer = msg.send();
        let _ = self.output.write_all(answer.as_bytes());
    }
}

//#[derive(Deserialize, Serialize)]
#[derive(Debug)]
struct Message{
    src: String,
    dest: String,
    body: HashMap<String, String>,
}

impl Message{
    pub fn new(input: Vec<String>) -> Message{
        let mut input = input.into_iter();
        input.next();
        let src: String = input.next().unwrap()
            .chars().skip_while(|&char| char != ':')
            .skip_while(|&char| char != 'c')
            .take_while(|&char| char != '"')
            .collect();
        let dest: String = input.next().unwrap()
            .chars().skip_while(|&char| char != ':')
            .skip_while(|&char| char != 'n')
            .take_while(|&char| char != '"')
            .collect();
        let body: HashMap<String, String> = input.skip(1)
            .take_while(|line| line.trim_start() != "}")
            .map(|line| {
                let mut temp = line.chars();
                let key: String = temp.by_ref().skip_while(|&char| char != '"')
                    .skip(1)
                    .take_while(|&char| char != '"')
                    .collect();
                let val: String = temp.skip_while(|&char| char != ' ')
                    .skip(1)
                    .take_while(|&char| char != ',')
                    .collect();
                (key, val)
            })
            .collect();
        Message{
            src,
            dest,
            body,
        }
    }
    
    pub fn send(self) -> String{
        let mut output = String::new();
        output.push_str("{\n");
        output.push_str(&format!("  \"{}\": \"{}\",\n", "scr", self.dest));
        output.push_str(&format!("  \"{}\": \"{}\",\n", "dest", self.src));
        output.push_str("  \"body\": {\n");
        for (key, val) in self.body{
            output.push_str(&format!("    \"{}\": {},\n", key, val));
        }
        output.push_str("  }\n");
        output.push_str("}\n");
        output
    }
    
    pub fn add(&mut self, key: &str, val: String){
        self.body.insert(key.to_string(), val);
    }
}

fn main() {
    let input = stdin();
    let output = stdout();
    let mut n = Node::new(input, output);
    let key: &'static str = "\"echo\"";
    n.handler(key, Box::new(|message_body|{
        eprintln!("{:#?}", message_body);
        let type_ref = message_body.get_mut("type").unwrap();
        *type_ref = "\"echo_ok\"".to_string();
        message_body.insert(
            "in_reply_to".to_string(),
            message_body.get("msg_id").unwrap().to_string());
        message_body.remove("msg_id");
    }));
    n.run();
}
