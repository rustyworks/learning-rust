fn main() {
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    let home = IpAddrKind::V4(String::from("127.0.0.1"));

    // route(four);
    // route(six);
    route(home);

    let m = Message::Write(String::from("Kentang goreng"));
    m.call();
}

enum IpAddrKind {
    V4(String),
    V6(String),
}

fn route(ip_kind: IpAddrKind) {
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct QuitMessage;
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String);
struct ChangeColor(i32, i32, i32);

impl Message {
    fn call(&self) {
        println!("Call being called");
    }
}
