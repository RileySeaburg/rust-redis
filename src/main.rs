// Copyright (c) 2022 Evolving Software Corporation
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

// The `lazy_static` crate is used to create a lazy static variable.
use lazy_static::lazy_static;
// The `resp::Decoder` struct is used to decode RESP messages.
use resp::Decoder;
// The `std::collections::HashMap` struct is used to store key-value pairs.
use std::collections::HashMap;
// The `std::env` module is used to get environment variables.
use std::env;
// The `std::io::{BufReader, Write}` structs are used to read and write to files.
use std::io::{BufReader, Write};
// The `std::net::{Shutdown, TcpListener, TcpStream}` structs are used to create and manage TCP connections.
use std::net::{Shutdown, TcpListener, TcpStream};
// The `std::sync::{Arc, Mutex}` structs are used to share data between threads.
use std::sync::{Arc, Mutex};
// The `std::thread` module is used to create and manage threads.
use std::thread;

// The `commands` module contains the commands that can be executed.
mod commands;
// The `process_client_request` function is used to process a client request.
use crate::commands::process_client_request;

// The `STORE` type is used to store key-value pairs.
// The `Mutex` struct is used to lock the data.
type STORE = Mutex<HashMap<String, String>>;

// Define a static variable to represent the key-value store.
lazy_static! {
    static ref RUDIS_DB: STORE = Mutex::new(HashMap::new());
}

/// # Description
fn main() {
    // Get the address from the command line arguments.
    // If no address is provided, use the default address.
    let addr = env::args()
        .skip(1)
        .next()
        .unwrap_or("127.0.0.1:6378".to_owned());
    let listener = TcpListener::bind(&addr).unwrap();
    // Print a message to the console.
    println!("Listening on {}", addr);
    // Bind the listener to the address.
    for stream in listener.incoming() {
        // For each incoming connection,
        // Wait until the previous thread has finished processing the request.
        for stream in listener.incoming() {
            // Unwrap the stream.
            let stream = stream.unwrap();
            // Print a message to the console.
            println!("Connection from {:?}", stream);
            // Handle the client request.
            thread::spawn(|| handle_client(stream));
        }
    }
}

///  ## Name
/// # `handle_client`
/// ## Description
/// ### The `handle_client` function is used to handle a client request.
fn handle_client(stream: TcpStream) {
    // Create a new stream for the client.
    let mut stream = BufReader::new(stream);
    // Create a new decoder for the client.
    let decoder = Decoder::new(&mut stream).decode();
    // match the decoder.
    match decoder {
        Ok(resp) => {
            // Process the client request.
            let reply = process_client_request(resp);
            // Send the reply to the client.
            stream.get_mut().write_all(&reply).unwrap();
        }
        Err(e) => {
            // Print an error message to the console.
            println!("Invalid command: {:?}", e);
            // Shutdown the client connection.
            let _ = stream.get_mut().shutdown(Shutdown::Both);
        }
    };
}
