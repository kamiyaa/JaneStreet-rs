# JaneStreetBot

This is a repository that tries to provide Rust starter code for
Jane Street's Trading Bot competition.

## Usage

You will mostly be interacting with `read_from_exchange`, `write_to_exchange`
and `JaneStJson`.

 - `connect()` establishes a TCP connection to the server
 - `say_hello()` sends the server some initial information (e.g. team name etc.)
   and returns a `PortFolio` struct that describes how many of each security your team
   currently owns
 - `read_from_exchange()` reads a line from the server and converts it into a generic
   `JaneStJson` struct
 - `write_to_exchange()` takes in a generic `JaneStJson` struct and writes it to the server
 - `JaneStJson` is a generic struct that accommodates for all the data the server may
   potentially provide. This means many fields may be empty (e.g. 0-value, empty string, etc.).
   Please check `_type` field to make sure exactly which fields will be non-empty
 - `PortFolio` is a hashmap that maps symbols to i32 `String -> i32`

## Examples
```rust
fn run() -> std::io::Result<()> {
    let mut conn = connect()?;
    let mut portfolio = say_hello(&mut conn)?;
    match read_from_exchange(&mut conn) {
        Ok(json) => println!("{:#?}", json),
        Err(e) => println!("error reading from connection: {}", e),
    }
    Ok(())
}

fn main() {
    run();
}
```
see [main.rs](https://github.com/kamiyaa/JaneStreet-rs/blob/master/src/main.rs) for more details
