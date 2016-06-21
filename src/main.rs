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
//for fd-set
pub const FD_SETSIZE: usize = 1024;

//external functions of library(libc) and other c files
extern {
      fn createANDbind(req: libc::c_int) -> libc::c_int;
      fn listen(socket_fd: c_int, backlog: c_int) -> c_int;
      fn makeSOCKETnonblocking(sfd: i32) -> i32;
      fn select(nfds: c_int, readfs: i32, writefds: i32, 
                errorfds: i32, timeout: * const timeval) -> c_int;
    //   fn connection(socket_fd: i32) -> i32;
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
    
    let mut socket_fd = unsafe {   createANDbind(req)  }; //call to function defined in c (to create and build socket_fd)

    println!("Your listening socket_fd is: {}",socket_fd);
    if socket_fd!=-1 {   //if socket_fd not generated
    }

    let mut s = unsafe{  makeSOCKETnonblocking(socket_fd)  };
    if s==-1 {  panic!("error while non-blocking the socket_fd"); }

    s= unsafe { listen(socket_fd,SOMAXCONN)};   //start listen on socket_fd with maximum length SOMAXCONN(120)
    if s==-1 { panic!("error while non-blocking the socket_fd"); }
    // println!("s:{}",s);
    windows(socket_fd); 

}


struct timeval {
    tv_sec: i32,
    tv_usec: i32
}

struct fd_set {
   pub fd_count: i32,
   pub fd_array:[usize;FD_SETSIZE]
}
impl fd_set{
    fn counter(&mut self){
        self.fd_count+=1;
    }
}


fn windows(socket_fd: i32){
     let mut timeout = &timeval{tv_sec:10,tv_usec:0};
   
     let mut readfds = &fd_set{fd_count:1,fd_array: [0;FD_SETSIZE]};

//      >=<      ...Here begins the EVENTLOOP...   >=<
    while true {
      println!("start loop");
       

       let n= unsafe { select(10,0,0,0,timeout)};

       println!("N:{}",n); 
       if n==0 {println!("timeout"); continue;}
       if n==-1 {println!("some error occured"); }

     
  //     let mut checkN=0;
  //     while checkN<n {
	// // {     
    //     if events.fd==socket_fd 
    //     {
    //      // We have a notification on the listening socket_fd(parent), which  means there may be more incoming connections.    
    //           println!("\nSOMETHING AT MAIN SOCKET\n"); 
    //            while true
    //             {
    //              let infd = unsafe {connection(socket_fd) };
    //               if infd==-1
    //                  { 
    //                    //println!("processed all incoming connections");
    //                     break;
    //                   }
    //               //make new conection non-blocking    
    //               let mut s = unsafe { makeSOCKETnonblocking(infd) };
    //               if s==-1 {  panic!("error while non-blocking the socket_fd"); }
    //               //initialising struct for new conections
    //               let mut event = &epoll_event { events: EPOLLIN |EPOLLET,fd :infd};
    //               //adding new connection's fd (infd) to epoll_instance (epfd) 
    //               s = unsafe {  epoll_ctl(epfd, EPOLL_CTL_ADD,infd,eventc) } ;
    //               if epfd == -1 { panic!("error  adding new fd to epoll instance"); }

    //             }     
    //    }else {
    //    println!("\n Some events:{} on fd:{}  ",events.events,events.fd);
    
    //    }
    //        //checkN+=1;
    //    //   }
    //   // }
     }  

}  


