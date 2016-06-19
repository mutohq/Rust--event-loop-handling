extern crate libc;
use libc::c_int;
use std::error::Error;
use std::env;  //to get command line arguments

  extern {
      fn createANDbind(req: libc::c_int) -> libc::c_int;
      fn listen(socket: c_int, backlog: c_int) -> c_int;
      fn makeSOCKETnonblocking(sfd: i32) -> i32;
      pub fn epoll_create1(flags: u32) -> libc::c_int;
      pub fn epoll_ctl(epfd: c_int, op: u32, fd: i32, event: *const epoll_event) -> i32;
      pub fn epoll_wait(epfd: libc::c_int, events:*const epoll_event, maxevents: libc::c_int, timeout: libc::c_int) -> libc::c_int;

  }

//flags of libc
pub const EPOLL_CTL_ADD: u32 = 1;
pub const EPOLLIN:   u32 = 0x01;
pub const EPOLLET: u32 = 0x80000000;
pub const MAXEVENTS: i32 = 32 ; 
pub const F_GETFL: c_int = 3;
pub const O_NONBLOCK: c_int = 2048;
pub const F_SETFL: c_int = 4;
pub const SOMAXCONN: c_int = 120;
 
// epoll structures to support functioning of FFI
//  pub struct epoll_data {
//     //  ptr: libc::c_void ,
//      fd:   c_int,
//      U32:  c_int,
//      U64:  u64    
//    } 
 pub struct epoll_event {
    pub events: u32,
    pub fd: i32
   } 

fn main(){  
    let mut args: Vec<_> = env::args().collect(); //to get command line arguments.

    println!("");    
    let mut req: i32 = args[1].trim().parse().expect("Please type a number!");
    
    let mut socket = unsafe {   createANDbind(req)  }; //call to function defined in c (to create and build socket)

    println!("new socket is: {}",socket);
    if socket!=-1 {   //if socket not generated
    }

    let mut s = unsafe{  makeSOCKETnonblocking(socket)  };
    if s==-1 {  panic!("error while non-blocking the socket"); }

    s= unsafe { listen(socket,SOMAXCONN)};   //listen on socket with maximum length SOMAXCONN(120)
    if s==-1 { panic!("error while non-blocking the socket"); }
    println!("s:{}",s);
  
    let  epfd = unsafe{  epoll_create1(0)   };  //to create epoll instance
    if epfd == -1 { panic!("epoll instance creation error"); }

    let  event=&epoll_event { events: EPOLLIN |EPOLLET,fd :socket};
              
    s = unsafe {    epoll_ctl(epfd, EPOLL_CTL_ADD, socket,event)  //to add file descriptor to epoll instance
                   };
    if epfd == -1 { panic!("error while adding fd(socket) to epoll instance "); }

    s = unsafe {    epoll_ctl(epfd, EPOLL_CTL_ADD,0,event)  //to add file descriptor to epoll instance
                   };
    if epfd == -1 { panic!("error while adding fd(socket) to epoll instance "); }

    let mut events = &epoll_event { events: EPOLLIN | EPOLLET, fd :socket};     
      // let mut x=0;
    while true {
      println!("start loop");
       
      let mut n = unsafe { epoll_wait(epfd,events,MAXEVENTS,6000) };
       
       if n==0 {println!("timeout"); continue;}
       if n==-1 {println!("some error occured"); break;}
       println!("n:{},fd:{}",n,events.fd);
        if events.fd==socket  // We have a notification on the listening socket, which  means one or more incoming connections. 
       { println!("something happened"); 
        }
    // `file` goes out of scope, and the "hello.txt" file gets closed
}
}    


