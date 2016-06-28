mod mylib;
use std::thread;
use std::sync::mpsc::{Sender,Receiver};
//for channels
use std::sync::mpsc;
use std::os::unix::io::AsRawFd;
use std::net::{TcpListener};
//SIMULATION CODE STARTS ..
 struct Cars {
    num: i32 ,
    name: &'static str
 }
struct Bikes {
    num: i32 ,
    name: &'static str
    
 }
impl mylib::Neccessary for Cars {
     fn initial(&self) {
       println!("working on  {} having number: {}",self.name,self.num);
     }  
}
impl mylib::Neccessary for Bikes {
     fn initial(&self) {
       println!("working on  {} ({})",self.name,self.num);
     }  
}

fn main(){
let (tx,rx)= mylib::make_channel();

mylib::start_eventloop(rx); 

// let instance = Cars{num:6405,name:"SWIFT - DEZIRE"};
let tx1=tx.clone();
//    mylib::eventloop_add(instance,tx1,5);

let tx2=tx.clone();

mylib::eventloop_add(Bikes{num:21,name:"ROYAL - ENFEILD"},tx2,3); 

let listener = TcpListener::bind("127.0.0.1:6564").unwrap(); 
let t_fd= listener.as_raw_fd();
mylib::eventloop_register(t_fd,Bikes{num:0629,name:"ROYAL - ENFEILD"},tx1); 
thread::sleep_ms(200000);
 
}
//....SIMULATION CODE ENDS
