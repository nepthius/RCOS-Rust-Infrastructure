use std::net::{TcpListener, TcpStream};
use std::io::{Write, Read};

fn handle_client(stream:TcpStream) {

}

fn main() -> std::io::Result<()>  {
    let mut msg = String::from("HOWDY THERE PARTNER");
    
    let mut stream = TcpStream::connect("127.0.0.1:3490").unwrap();
    stream.write_all(msg.as_bytes());
    Ok(())
    
    

}
