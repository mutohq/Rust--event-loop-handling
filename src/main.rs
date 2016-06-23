extern crate libc;
use libc::c_int;
use std::error::Error;
use std::env;  //to get command line arguments
//to get fd from tcpListener
use std::os::unix::io::AsRawFd;
use std::net::{TcpListener,TcpStream};
use std::os;
use std::thread;
use std::ptr;
//for channels
use std::sync::mpsc;
//to make immutable content to share among threads safely
use std::sync::{Arc,Mutex};


//flags of libc
pub const EPOLL_CTL_ADD: u32 = 1;
pub const EPOLLIN:   u32 = 0x01;
pub const EPOLLET: u32 = 0x80000000;
pub const MAXEVENTS: i32 = 32 ; 
pub const F_GETFL: c_int = 3;
pub const O_NONBLOCK: c_int = 2048;
pub const F_SETFL: c_int = 4;
pub const SOMAXCONN: c_int = 120;
pub const MAXTHREAD :i32 = 5;

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

/*structure to serve the incoming event:  queue[to_serve[1],to_serve[2],to_serve[3],......]....
  ....(RUNNING_THREADS < MAXTHREAD) then extract first connection from queue and serve it  */
struct  to_serve{
    pub fd : i32,
    pub stream : Option<TcpStream>,
    pub status: bool 
    }


//***Event_loop (thread) ***
fn main() {
    let args: Vec<_> = env::args().collect(); //to get command line arguments.
    // println!("{}",args[1]);
    //to convert String to &str (because String does not live for entire lifetime of program)
    let address :&str= &*args[1] ;
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    
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

    //creating a event_epoll event instance to capture the events from epoll_wait
    let mut events = &epoll_event { events: EPOLLIN | EPOLLET, fd :socket_fd};     
    
    //creating queue to store data of  fired events
    let mut queue   = Arc::new(Mutex::new(vec![to_serve{fd:0,stream:None,status:false}]));
    {
        let mut temp_queue = queue.lock().unwrap();
        temp_queue.remove(0);
    }
    //varibles to store number of running threads  
    let thread_count = Arc::new(Mutex::new(0));
   
//      >=<      ...Here begins the EVENTLOOP...   >=<
    while true {
    //   println!("start event_loop");
      
      let mut n = unsafe { epoll_wait(epfd,events,MAXEVENTS,3000) };
       
       if n==0 {println!("timeout"); continue;}
       if n==-1 {println!("some error occured"); continue;}
      // println!("\n Number of fd's accessed:{}, events on fd:{}, and events:{}",n,events.fd,events.events);

        if events.fd==socket_fd 
        {

         // We have a notification on the listening socket_fd(parent), which  means there may be more incoming connections.    
              println!("\nSOMETHING AT MAIN SOCKET\n"); 

                 for stream in listener.incoming() {
                //  println!("to check incoming connection");
                 match stream {
                         Ok(stream) => {
                        //  connection succeeded                        
                        // create instance of incoming connection 
                         let mut connection = to_serve{ fd: stream.as_raw_fd(), stream: Some(stream) ,status:false };
                           {      //inserting new connection from socket to queue(to_serve)
                                  let mut temp_queue = queue.lock().unwrap();
                                  temp_queue.push(connection);     
                           }
                         }
                         Err(e) => {
                         println!("Accept err {}", e); 
                         }
                   }
                    break;     
                 }
           }
       else {
          println!("\n\n EVENTS NOTICED ON FD  :{}  ",events.fd);             
          let mut connection = to_serve{ fd: events.fd,stream : None ,status:false };
          {         
             {  //inserting event from files(other than socket) to queue(to_serve)      
                let mut temp_queue = queue.lock().unwrap();
                temp_queue.push(connection);     
             }           
          }
        }
        
        //HERE BEGINS the QUEUE-PROCESSING
          let mut len ;
           {      let mut temp_queue = queue.lock().unwrap();
                 len = temp_queue.len();     
           }    
         println!("length of queue:{}",len);
     
       for i in 0..len  {
        // println!("INSIDE QUEUE PROCESSING");
    
        let mut ctr;
        let mut state:bool;                   
        {      //accessing mutually-exclusive values..
               let mut queue_elem = queue.lock().unwrap();
               state = queue_elem[i].status;
               let mut thread_count = thread_count.lock().unwrap();
               ctr =*thread_count;       
        }
       
         if ctr < MAXTHREAD  {
            //  println!("inside ctr<MAXTHREAD");
              if !state {
                {   //increasing thread_count by 1 ,before spawing new thread
                    let mut thread_count =  thread_count.lock().unwrap();
                    *thread_count +=1;
                    println!("value of thread_count:{}",*thread_count);
                }
              //cloning varibles that are going to be shared among threads 
              let thread_count = thread_count.clone();
              let queue = queue.clone();  
              //new thread to serve client_request
                   thread::spawn(move || {
                    let mut temp_queue = queue.lock().unwrap();
                    let ref mut client = temp_queue[i];   
                    //function call to serve request
                    serve_client(client);
                    //decreasing thread_count by one (as request got processed in above function call)  
                    let mut thread_count = thread_count.lock().unwrap();
                    *thread_count -=1;
                });
                 
              }
          }else{
              break;
          }
        }

       
  }  
}

 // function to serve  request (it serves requests from both internal files and sockets)..
 fn serve_client(request: &mut to_serve) {
    //check stream (if stream is present then request on socket otherwise on internal file) 
    match request.stream {
         None => {    println!("serving internal file request from:{}",request.fd);
         }
         //takes reference to prevent "move out of borrowed content (request stream)"
         Some(ref stream) =>{
              println!("serving network request{}, fd:{}",stream.peer_addr().unwrap(),request.fd);
         }
     }
    //make thread to sleep for some msec's (just for simulation)
    thread::sleep_ms(2000);
    /*updating status of queue instance(to refelect that it had processed one time)...
      ... one can also use counter to reflect number of times particular request(to_serve instance) got served*/
    request.status = true;
    println!("closing wrorking on fd:{ }",request.fd);
 }


