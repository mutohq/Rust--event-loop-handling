extern crate libc;
//to get fd from tcpListener
use std::os::unix::io::AsRawFd;
use std::net::{TcpListener,TcpStream};
use std::thread;
use std::sync::mpsc::{Sender,Receiver};
//for channels
use std::sync::mpsc;
//to make immutable content to share among threads safely
use std::sync::{Arc,Mutex};


//flags of libc
pub const EPOLL_CTL_ADD: u32 = 1;
pub const EPOLLIN:   u32 = 0x01;
pub const EPOLLET: u32 = 0x80000000;
pub const MAXEVENTS: i32 = 32 ; 
pub const MAXTHREAD :i32 = 5;

//external functions of library(libc) and other c files
extern {
      pub fn epoll_create1(flags: u32) -> i32;
      pub fn epoll_ctl(epfd: i32, op: u32, fd: i32, event: *const EpollEvent) -> i32;
      pub fn epoll_wait(epfd: i32, events:*const EpollEvent, maxevents: i32, timeout: i32) -> i32;     
  }

 //structure to represent EpollEvents
  pub struct EpollEvent {
     events: u32,
     fd: i32
   } 

/*structure to serve the incoming event:  queue[ToServe[1],ToServe[2],ToServe[3],..]..
  ....( RUNNING_THREADS < MAXTHREAD ) then extract first connection from queue and  ...
  ....serve it  */
pub struct  ToServe<T>{
     fd : i32,
     status: i32,
     register:bool,
     inner: T 
    }

//create channel to interact with event_loop(Neccessary to call by every user)   
pub fn make_channel<T>() ->(Sender<ToServe<T>>,Receiver<ToServe<T>>) {
      let (tx,rx):(Sender<ToServe<T>>,Receiver<ToServe<T>>) = mpsc::channel();
      (tx,rx)
}

//A trait which is Neccessary for every user structure to implement    
 pub trait Neccessary {
    fn initial(&self);  
 }



//function to start event_loop thread.
pub fn start_eventloop<T:Send + Sync +'static+Neccessary>(rx: Receiver<ToServe<T>>){
 thread::spawn(move ||{
    event_loop(rx);
 }
 );
thread::sleep_ms(1000);
}


//***Event_loop (thread) ***
fn event_loop<T:Send + Sync +'static+Neccessary>(rx: Receiver<ToServe<T>>) {
    let listener = TcpListener::bind("127.0.0.1:6565").unwrap();    
    let socket_fd = listener.as_raw_fd();
    println!("socket_fd:{}",socket_fd);
    //to create epoll_instance  
    let epfd = unsafe{  epoll_create1(0)   }; 
    // println!("epfd:{}",epfd);
    if epfd == -1 { panic!("epoll instance creation error"); }
    //initialising EpollEvent for socket_fd
    let  event=&EpollEvent { events: EPOLLIN |EPOLLET,fd :socket_fd};
    //to add file descriptor to epoll instance          
    let  s = unsafe {    epoll_ctl(epfd, EPOLL_CTL_ADD, socket_fd,event)  
                       };
    if s == -1 { panic!("error while adding fd(socket_fd) to epoll instance "); }

    //creating a event_epoll event instance to capture the events from epoll_wait
    let  events = &EpollEvent { events: EPOLLIN | EPOLLET, fd :socket_fd};         
    //creating queue to store data of  fired events
    let  queue   = Arc::new(Mutex::new(Vec::new()));    
    //varibles to store number of running threads  
    let thread_count = Arc::new(Mutex::new(0));
   
//      >=<      ...Here begins the EVENTLOOP...   >=<
    while true {
      let  n = unsafe { epoll_wait(epfd,events,MAXEVENTS,3000) };      
       if n==0 {println!("timeout"); }
       if n==-1 {println!("some error occured"); continue;}
      //  println!("number of events:{}",n);
       let  len ;
       {      let  temp_queue = queue.lock().unwrap();
              len = temp_queue.len();     
       }    
     
        if n>0{
         //enter if there is some event on monitoring fd's 
        if events.fd==socket_fd 
        {     //event on default socket_fd (represents something to add to queue)
              // println!("Main Socket"); 
              let instance = rx.recv().unwrap();
              if instance.register{
                     //some fd to add to monitoring_list of epoll
                     let  event=&EpollEvent { events: EPOLLIN |EPOLLET,fd :instance.fd};
                     //to add file descriptor to epoll instance          
                     let  s = unsafe {    epoll_ctl(epfd, EPOLL_CTL_ADD, instance.fd,event)  
                       };
              } 
                 //adding received instance to processing queue  
                 let mut temp_queue = queue.lock().unwrap();
                 temp_queue.push(instance);  
                //  println!("instance added");   
              
           }
       else {
        //  println!("event on fd:{}",events.fd);
                for i in 0..len  {
                let  ctr;                 
                   {      //accessing mutually-exclusive values..
                      let  mut queue_elem = queue.lock().unwrap();
                      if queue_elem[i].fd==events.fd{
                      //  println!("status updated");
                      queue_elem[i].status=1;
                      }else{continue;}
                      let  thread_count = thread_count.lock().unwrap();
                      ctr =*thread_count;
                      //  println!(" thread_count:{} ",*thread_count);       
                   }
                if ctr < MAXTHREAD  {
                       {   //increasing thread_count by 1 ,before spawing new thread
                           let mut thread_count =  thread_count.lock().unwrap();
                           *thread_count +=1;
                          println!("value of thread_count:{} ",*thread_count);
                       }
                    //cloning varibles that are going to be shared among threads 
                    let thread_count = thread_count.clone();
                    let queue = queue.clone();  
                    //new thread to serve client_request
                    thread::spawn(move || {
                           let mut temp_queue = queue.lock().unwrap();
                           let ref mut client = temp_queue[i];   
                           //function call to serve request
                           serve(client);
                           /*decreasing thread_count by one (as request got processed... 
                             ...in above function call)*/  
                          let mut thread_count = thread_count.lock().unwrap();
                          *thread_count -=1;
                        });
                 }else{
                        break;   }
                 }     
                     /*skip the below queue-processing this time(when some event is...
                       ...observed on monitoring fd's) */
                     continue;                     
           }
        }
        
      
      /*HERE BEGINS the QUEUE-PROCESSING...(it starts only if there is no event on...
        ... the monitoring list.) */
             
         println!("length of queue:{}",len);
     
       for i in 0..len  {
        // println!("INSIDE QUEUE PROCESSING");
    
        let  ctr;
        let  state:i32;                   
        {      //accessing mutually-exclusive values..
               let  queue_elem = queue.lock().unwrap();
               state = queue_elem[i].status;
               let  thread_count = thread_count.lock().unwrap();
               ctr =*thread_count;
              //  println!(" thread_count:{} ",*thread_count);       
        }
       
         if ctr < MAXTHREAD  {
            //  println!("inside ctr<MAXTHREAD");
              if state>0   {
                {   //increasing thread_count by 1 ,before spawing new thread
                    let mut thread_count =  thread_count.lock().unwrap();
                    *thread_count +=1;
                    println!("value of thread_count:{} ",*thread_count);
                }
              //cloning varibles that are going to be shared among threads 
              let thread_count = thread_count.clone();
              let queue = queue.clone();  
              //new thread to serve client_request
                   thread::spawn(move || {
                    let mut temp_queue = queue.lock().unwrap();
                    let ref mut client = temp_queue[i];   
                    //function call to serve request
                    serve(client);
                    /*decreasing thread_count by one (as request got processed in...
                      ... above function call) */  
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

//to add structure functions to event_loop(to be execute -(repeat) number of times)
pub fn eventloop_add<T>(instance: T  , tx:Sender<ToServe<T>>,repeat: i32) {
      //create instance of ToServe(structure) and send it on channel
      let temp_elem = ToServe{ fd:-1 , status:repeat,register:false,inner:instance};
      tx.send(temp_elem).unwrap();
      //to fire an event on of main socket (for adding above instance to PROCESSING)
      TcpStream::connect("127.0.0.1:6565").unwrap();     
}

/*to register some structure function to event-loop such that if any event occur on...
  ...passed fd corresponding structure function should get executed*/
pub fn eventloop_register<T>(fd:i32,instance: T,tx:Sender<ToServe<T>>){
      //create instance of ToServe(structure) and send it on channel
      let temp_elem = ToServe{ fd:fd , status:0,register:true,inner:instance};
      tx.send(temp_elem).unwrap();
      //to fire an event on of main socket (for adding above instance to PROCESSING)
      TcpStream::connect("127.0.0.1:6565").unwrap(); 
}

 // function to serve  request (it serves requests from both internal files and sockets)..
 fn serve<T:Neccessary>(request: &mut ToServe<T>) {
    //make thread to sleep for some msec's (just for simulation)
     
    thread::sleep_ms(2000);
    request.inner.initial();
    /*updating status of queue instance(to refelect that it had processed one more time)...
      ... one can also use flag to reflect processing status of request*/
    request.status -=1;
 }


