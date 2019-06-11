use std::io::{BufRead, Write};
use std::net::TcpStream;
use std::collections::HashMap;

use serde_json::json;
use serde_derive::{Serialize, Deserialize};

// for telling the server your input is complete
const EOF: &[u8; 1] = &['\n' as u8];

const TEAM_NAME: &str = "Your Team Name";

const PROD_EXCHANGE_HOSTNAME: &str = "test-exch-production:25000";
const EXCHANGE_HOSTNAME: &str = "test-exch-EDJ:25000";


const CURRENT_HOSTNAME: &str = EXCHANGE_HOSTNAME; 

#[derive(Debug, Serialize, Deserialize)]
struct Security {
    pub symbol: String,
    pub position: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct RawPortFolio {
    #[serde(rename = "type")]
    pub _type: String,
    pub symbols: Vec<Security>,
}

trait PortFolioConvert {
    fn portfolio_convert(p: RawPortFolio) -> Self;
}

type PortFolio = HashMap<String, i32>;

impl PortFolioConvert for PortFolio  {
    fn portfolio_convert(p: RawPortFolio) -> Self {
        let mut portfolio = PortFolio::new();
        for s in p.symbols {
            portfolio.insert(s.symbol, s.position);
        }
        portfolio
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct JaneStJson {
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(default)]
    pub order_id: usize,
    #[serde(default)]
    pub symbol: String,
    #[serde(default)]
    pub dir: String,
    #[serde(default)]
    pub size: usize,
    #[serde(default)]
    pub buy: Vec<[usize; 2]>,
    #[serde(default)]
    pub symbols: Vec<String>,
    #[serde(default)]
    pub error: String,
}

fn connect() -> std::io::Result<TcpStream> {
    let mut exchange = TcpStream::connect(CURRENT_HOSTNAME)?;
    Ok(exchange)
}

fn say_hello(exchange: &mut TcpStream) -> std::io::Result<PortFolio> {
    let json_1 = json!({
        "type": "hello",
        "team": TEAM_NAME,
    });
    exchange.write_all(json_1.to_string().as_bytes())?;
    exchange.write(EOF);

    let mut reader = std::io::BufReader::new(exchange);
    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;

    let portfolio: PortFolio = PortFolio::portfolio_convert(
            serde_json::from_str(&buffer)?);
    Ok(portfolio)
}

fn read_from_exchange(exchange: &mut TcpStream) -> std::io::Result<JaneStJson> {
    let mut reader = std::io::BufReader::new(exchange);
    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;

    let json: JaneStJson = serde_json::from_str(&buffer)?;
    Ok(json)
}

fn write_to_exchange(exchange: &mut TcpStream, json: &JaneStJson) -> std::io::Result<()> {
    let json_string = serde_json::to_string(json)?;
    exchange.write_all(json_string.as_bytes())?;
    exchange.write(EOF)?;
    Ok(())
}

fn run() -> std::io::Result<()> {
    let mut exchange = connect()?;
    let mut portfolio = say_hello(&mut exchange)?;
    loop {
        match read_from_exchange(&mut exchange) {
            Ok(json) => {

            },
            Err(e) => eprintln!("read_from_exchange error: {}", e),
        }
    }
    Ok(())
}

fn main() {
    match run() {
        Err(e) => eprintln!("Error from run: {}", e),
        _ => {},
    }
    println!("Closing bot");
}
