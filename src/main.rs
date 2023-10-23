#![allow(dead_code)]
struct Node{
    id: usize,
    
}

//#[derive(Deserialize, Serialize)]
struct Message{
    src: usize,
    dest: usize,
    body: Body,
}

//#[derive(Deserialize, Serialize)]
//#[serde(tag = "type")]
//serde flatten?
enum Body{
    Echo(Echo),
    EchoOk(EchoOk)
}

struct Echo{
//#[derive(Deserialize)]
    r#type: String,
    msg_id: usize,
    echo: String,
}

struct EchoOk{
//#[derive(Serialize)]
    r#type: String,
    msg_id: usize,
    echo: String,
}

fn main() {
    println!("Hello, world!");
}
