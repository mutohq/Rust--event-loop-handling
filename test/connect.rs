use std::net::TcpStream;
use std::env;

fn main(){
     //to get command line arguments.
     let args:Vec<_> = env::args().collect();    
     //to convert String => &str
     let address :&str= &*args[1];
     TcpStream::connect(address).unwrap();
}
