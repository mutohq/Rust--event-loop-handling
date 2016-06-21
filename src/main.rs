extern crate libc;
use libc::c_int;
use std::error::Error;
use std::env;  //to get command line arguments
//to get fd from tcpListener
use std::os::unix::io::AsRawFd;
use std::net::{TcpListener,TcpStream};
use std::os;
use std::thread;
//flags of libc
pub const EPOLL_CTL_ADD: u32 = 1;
pub const EPOLLIN:   u32 = 0x01;
pub const EPOLLET: u32 = 0x80000000;
pub const MAXEVENTS: i32 = 32 ; 
pub const F_GETFL: c_int = 3;
pub const O_NONBLOCK: c_int = 2048;
pub const F_SETFL: c_int = 4;
pub const SOMAXCONN: c_int = 120;

//external functions of library(libc) and other c files
extern {
      pub fn epoll_create1(flags: u32) -> libc::c_int;
      pub fn epoll_ctl(epfd: c_int, op: u32, fd: i32, event: *const epoll_event) -> i32;
      pub fn epoll_wait(epfd: libc::c_int, events:*const epoll_event, maxevents: libc::c_int, timeout: libc::c_int) -> libc::c_int;
     
  }

 //structure to represent epoll_events
  pub struct epoll_event {
    pub events: u32,
    pub fd: i32
   } 

fn main(){  
    let mut args: Vec<_> = env::args().collect(); //to get command line arguments.
    // let host = args[1];
    // println!("");    
    // let mut port: i32 = args[1].trim().parse().expect("Please type a number!");
    // let mut address  = args[1] as String;

    // let mut address =args.to_str();
    println!("{}",args[1]);
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    
    let socket_fd = listener.as_raw_fd();
    println!("socket_fd:{}",socket_fd);

    //to create epoll_instance  
    let  epfd = unsafe{  epoll_create1(0)   }; 
    if epfd == -1 { panic!("epoll instance creation error"); }

    //initialising epoll_event for socket_fd
    let mut event=&epoll_event { events: EPOLLIN |EPOLLET,fd :socket_fd};
    //to add file descriptor to epoll instance          
    let mut s = unsafe {    epoll_ctl(epfd, EPOLL_CTL_ADD, socket_fd,event)  
                   };
    if s == -1 { panic!("error while adding fd(socket_fd) to epoll instance "); }

    //to add file descriptor (of console) to epoll instance
    let mut eventc=&epoll_event { events: EPOLLIN |EPOLLET,fd :0};    
    s = unsafe {    epoll_ctl(epfd, EPOLL_CTL_ADD,0,eventc)  
                   };
    if s == -1 { panic!("error while adding fd(of console) to epoll instance"); }

    let mut events = &epoll_event { events: EPOLLIN | EPOLLET, fd :socket_fd};     

// //      >=<      ...Here begins the EVENTLOOP...   >=<
    while true {
      //println!("start loop");
       
      let mut n = unsafe { epoll_wait(epfd,events,MAXEVENTS,3000) };
       
       if n==0 {println!("timeout"); continue;}
       if n==-1 {println!("some error occured"); break;}
       println!("\n Number of fd's accessed:{}, events on fd:{}, and events:{}",n,events.fd,events.events);
     
  //     let mut checkN=0;
  //     while checkN<n {
	// {     
        if events.fd==socket_fd 
        {
         // We have a notification on the listening socket_fd(parent), which  means there may be more incoming connections.    
              println!("\nSOMETHING AT MAIN SOCKET\n"); 
                 
                 for stream in listener.incoming() {
                 println!("to check incoming connection");
                 match stream {
                         Ok(stream) => {
                         thread::spawn(move|| {
                         // connection succeeded
                         serve_client(stream)
                          });
                         }
                         Err(e) => {
                         println!("Accept err {}", e); 
                         }
                   }
                       
                 }
                 println!("after done checking parent_socket ");

           }
       else {
          println!("\n Some events:{} on fd:{}  ",events.events,events.fd);
    
       }
       
     }  
}  

 fn serve_client(stream: TcpStream) {
     println!("{:?}",stream);
    // stream.write(b"Hello Connection");
    thread::sleep_ms(10000);
    println!("closing:{}",stream.peer_addr().unwrap());
//      println!("{}", stream.peer_addr().unwrap());
 }


