// enums2.rs
//
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a
// hint.

#[derive(Debug)]
enum Message {
    Move { x: i32, y: i32 },
    Echo(String),
    ChangeColor(u8, u8, u8),
    Quit,
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let messages: [Message; 4] = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    let messageMove: Message = Message::Move { x: 10, y: 20 };

    if let Message::Move { x, y } = messageMove {
        println!("{} - {}", x, y);
    }
    // messageMove.x; // ERROR! As we don't know for sure that messageMove is of type Move

    for message in &messages {
        message.call();
    }
}
