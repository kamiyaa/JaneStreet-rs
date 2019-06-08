use std::io::Write;
use std::io;
use std::net::TcpStream;

use serde_json::json;
use serde_derive::{Serialize, Deserialize};

/*
def connect():
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    s.connect((exchange_hostname, port))
    return s.makefile('rw', 1)
*/

const TEAM_NAME: &str = "EDJ";
const TEST_MODE: bool = true;

// production
const PORT: &str = "25000";
// slower
// const PORT: &str = "25001";
// empty
// const PORT: &str = "25002";

const TEST_EXCHANGE_INDEX: i32 = 2;
const PROD_EXCHANGE_HOSTNAME: &str = "production:25000";
const EXCHANGE_HOSTNAME: &str = "test-exch-EDJ:25000";

// {"type": "add", "order_id": N, "symbol": "SYM", "dir": "BUY", "price": N, "size": N}
#[derive(Serialize, Deserialize)]
struct AddOrder {
    #[serde(rename = "type")]
    _type: &'static str,
    order_id: i32,
    symbol: String,
    dir: &'static str
    price: usize,
    size: usize,
}
impl AddOrder {
    pub fn new_buy(order_id: i32, symbol: String, price: usize, sell: usize) {
        AddOrder {
            _type: "add",
            order_id,
            symbol,
            dir: "buy",
            price,
            size,
        }
    }

    pub fn new_sell(order_id: i32, symbol: String, price: usize, sell: usize) {
        AddOrder {
            _type: "add",
            order_id,
            symbol,
            dir: "sell",
            price,
            size,
        }
    }
}

fn connect() -> io::Result<TcpStream> {
    let mut stream = TcpStream::connect(PROD_EXCHANGE_HOSTNAME)?;
    Ok(stream)
}

fn say_hello(stream: &mut TcpStream) -> io::Result<()> {
    let json_1 = json!({
        "type": "hello",
        "team": TEAM_NAME,
    });
    stream.write_all(json_1.to_string().as_bytes())?;
    stream.read()?;
    Ok(())
}

fn main() {
    let mut exchange = connect().unwrap();
    say_hello(&mut exchange).unwrap(); 
    println!("Hello, world!");
}
