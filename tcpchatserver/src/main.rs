use std::net::{TcpListener, TcpStream};
use std::io::{Write, Read};
use std::iter::Iterator;
use std::thread;


fn handle_client(mut stream:TcpStream) {
    let mut buffer = [0u8; 1024];
    let bytes = stream.read(&mut buffer).unwrap();
    println!("{}", String::from_utf8_lossy(&buffer[..bytes]));
}

fn main() {
    
    //First get a socket and then bind to it
    let listener = TcpListener::bind("127.0.0.1:3490").unwrap(); //does  socket, bind, and listen (c)
    
    while(true){
        match listener.accept(){
            Ok((_socket, addr))=> {handle_client(_socket)},
            Err(e)=>println!("ruh roh: {e}"),
        }
    }


}
