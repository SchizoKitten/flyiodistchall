use std::{
    io::{Stdout, Stdin, Write},
    collections::HashMap
};

pub struct Node{
    id: String,
    nodes: String,
    message_count: usize,
    input: Stdin,
    output: Stdout,
    handlers: HashMap<&'static str, Box<dyn Fn(&mut Message)>>,
}

impl Node{
    //TODO: finish recieving init msg
    pub fn new(input: Stdin, output: Stdout) -> Node{
        let mut new = Node{
            id: String::new(),
            nodes: String::new(),
            message_count: 0,
            input,
            output,
            handlers: HashMap::new(),
        };
        //let _message = Message::new(new.handle_message());
        new
    }

    pub fn handler(&mut self, key: &'static str, f: Box<dyn Fn(&mut Message)>){
        self.handlers.insert(key, f);
    }

    fn handle_message(&mut self) -> String{
        let mut message = String::new();
        let _ = self.input.read_line(&mut message);
        message
    }
    
    pub fn run(&mut self){
        loop{
            let message = self.handle_message();
            if message.is_empty(){
                continue;
            }
            let mut message = Message::new(message);
            if let Some(handle) = self.handlers.get(dbg!(message.msg_type())){
                handle(&mut message);
                self.send(message);
            }else{
                panic!("No handle");
            }
        }
    }

    //TODO: format send corectly
    fn send(&mut self, mut msg: Message){
        self.message_count += 1;
        msg.add("msg_id", self.message_count.to_string());
        let answer = msg.send();
        let _ = self.output.write_all(answer.as_bytes());
    }
}

#[derive(Debug)]
pub struct Message{
    src: String,
    dest: String,
    r#type: String,
    body: HashMap<String, String>,
}

impl Message{
    //TODO: fix this
    pub fn new(input: String) -> Message{
        let mut input = input.split(',');
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
        let r#type: String = input.next().unwrap()
            .chars().skip_while(|&char| char != ':')
            .skip(1)
            .skip_while(|&char| char != ':')
            .skip(1)
            .collect();
        let body: HashMap<String, String> = input
            .map(|line| {
                let mut temp = line.trim().trim_end_matches('}').chars();
                let key: String = temp.by_ref()
                    .skip(1)
                    .take_while(|&char| char != '"')
                    .collect();
                let val: String = temp
                    .skip(1)
                    .collect();
                (key, val)
            })
            .collect();
        dbg!(Message{
            src,
            dest,
            r#type,
            body,
        })
    }

    pub fn msg_type(&self) -> &str{
        &self.r#type
    }

    pub fn set_type(&mut self, set: String){
       self.r#type = set;
    }
    
    pub fn get_body_mut_ref(&mut self) -> &mut HashMap<String, String>{
        &mut self.body
    }
    
    //TODO:format corectly
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
