extern crate libc;
use libc::c_int;
use std::error::Error;
use std::env;  //to get command line arguments

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
      fn createANDbind(req: libc::c_int) -> libc::c_int;
      fn listen(socket: c_int, backlog: c_int) -> c_int;
      fn makeSOCKETnonblocking(sfd: i32) -> i32;
      pub fn epoll_create1(flags: u32) -> libc::c_int;
      pub fn epoll_ctl(epfd: c_int, op: u32, fd: i32, event: *const epoll_event) -> i32;
      pub fn epoll_wait(epfd: libc::c_int, events:*const epoll_event, maxevents: libc::c_int, timeout: libc::c_int) -> libc::c_int;
      fn connection(socket: i32) -> i32;
  }

 //structure to represent epoll_events
  pub struct epoll_event {
    pub events: u32,
    pub fd: i32
   } 

fn main(){  
    let mut args: Vec<_> = env::args().collect(); //to get command line arguments.

    println!("");    
    let mut req: i32 = args[1].trim().parse().expect("Please type a number!");
    
    let mut socket = unsafe {   createANDbind(req)  }; //call to function defined in c (to create and build socket)

    println!("Your listening socket is: {}",socket);
    if socket!=-1 {   //if socket not generated
    }

    let mut s = unsafe{  makeSOCKETnonblocking(socket)  };
    if s==-1 {  panic!("error while non-blocking the socket"); }

    s= unsafe { listen(socket,SOMAXCONN)};   //listen on socket with maximum length SOMAXCONN(120)
    if s==-1 { panic!("error while non-blocking the socket"); }
    //println!("s:{}",s);
  
    let  epfd = unsafe{  epoll_create1(0)   };  //to create epoll instance
    if epfd == -1 { panic!("epoll instance creation error"); }

    let mut event=&epoll_event { events: EPOLLIN |EPOLLET,fd :socket};
              
    s = unsafe {    epoll_ctl(epfd, EPOLL_CTL_ADD, socket,event)  //to add file descriptor to epoll instance
                   };
    if s == -1 { panic!("error while adding fd(socket) to epoll instance "); }

    //to add file descriptor (of console) to epoll instance
    let mut eventc=&epoll_event { events: EPOLLIN |EPOLLET,fd :0};
    // println!("before inserting console to monitor");
    s = unsafe {    epoll_ctl(epfd, EPOLL_CTL_ADD,0,eventc)  
                   };
    // println!("after inserting console to monitor");
    if s == -1 { panic!("error while adding fd(of console) to epoll instance"); }

    let mut events = &epoll_event { events: EPOLLIN | EPOLLET, fd :socket};     

//      >=<      ...Here begins the EVENTLOOP...   >=<
    while true {
      //println!("start loop");
       
      let mut n = unsafe { epoll_wait(epfd,events,MAXEVENTS,6000) };
       
       if n==0 {println!("timeout"); continue;}
       if n==-1 {println!("some error occured"); break;}
       println!("\n Number of fd's accessed:{}, events on fd:{}, and events:{}",n,events.fd,events.events);
     
  //     let mut checkN=0;
  //     while checkN<n {
	// {     
        if events.fd==socket 
        {
         // We have a notification on the listening socket(parent), which  means there may be more incoming connections.    
              println!("\nSOMETHING AT MAIN SOCKET\n"); 
               while true
                {
                 let infd = unsafe {connection(socket) };
                  if infd==-1
                     { 
                       //println!("processed all incoming connections");
                        break;
                      }
                  //make new conection non-blocking    
                  let mut s = unsafe { makeSOCKETnonblocking(infd) };
                  if s==-1 {  panic!("error while non-blocking the socket"); }
                  //initialising struct for new conections
                  let mut event = &epoll_event { events: EPOLLIN |EPOLLET,fd :infd};
                  //adding new connection's fd (infd) to epoll_instance (epfd) 
                  s = unsafe {  epoll_ctl(epfd, EPOLL_CTL_ADD,infd,eventc) } ;
                  if epfd == -1 { panic!("error  adding new fd to epoll instance"); }

                }     
       }else {
       println!("\n Some events:{} on fd:{}  ",events.events,events.fd);
    
       }
           //checkN+=1;
       //   }
      // }
     }  

}  


