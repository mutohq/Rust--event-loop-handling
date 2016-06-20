//for socklen_t,accept() ,getaddrinfo& struct socketaddr 
#include <sys/types.h>        
#include <sys/socket.h>
#include <netdb.h>
#include <unistd.h>
//for printf
#include <stdio.h>
//for epoll flags (EAGAIN,EWOULDBLOCK) and other valuea
#include <sys/epoll.h>
//for errno
#include <errno.h>

int connection(int parent_socket){
            
           struct sockaddr in_addr;
           socklen_t in_len;
           int infd,s;
           char hbuf[NI_MAXHOST], sbuf[NI_MAXSERV];
           in_len = sizeof in_addr;

           //to extract new connection from the QUEUE  ON parent_socket 
           infd = accept (parent_socket, &in_addr, &in_len);  //returns  fd (infd) corresponding to new connection and it's address in "in_addr"
           if (infd == -1)
           {
               if ((errno == EAGAIN) ||
               (errno == EWOULDBLOCK))
                 {
                    // We have processed all incoming connections.                    
                   return -1;
                 }
                
            }

         //get host and service of new connection (it puts host and service in "hbuf" and "sbuf" respectively )
         s = getnameinfo (&in_addr, in_len,
                            hbuf, sizeof hbuf,
                            sbuf, sizeof sbuf,
                            NI_NUMERICHOST | NI_NUMERICSERV);
          if (s == 0)
                {
                printf("New Connection accepted on descriptor %d "
                        "(host=%s, port=%s)\n", infd, hbuf, sbuf);
                return infd;
                }                
                return -1;
}                          
