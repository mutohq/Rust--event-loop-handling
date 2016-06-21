#include<string.h>  //for memset()
#include <sys/types.h>
#include <sys/socket.h> //for socket(),bind()
#include <netdb.h>  //for getaddrinfo
#include <stdio.h>

int createANDbind(int iport)
  {
  // printf("in c :%d",iport);
  struct addrinfo hints;
  struct addrinfo *result, *rp;
  int s, sfd;    char  *port;  
   //to convert int char* 
  sprintf( port, "%d", iport );
  memset (&hints, 0, sizeof (struct addrinfo));  
  hints.ai_family = AF_UNSPEC;     /* Return IPv4 and IPv6 choices */
  hints.ai_socktype = SOCK_STREAM; /* We want a TCP socket */
  hints.ai_flags = AI_PASSIVE;     /* All interfaces */
  // printf(" port:%s\n",port);
  
  /* The hints argument points to an addrinfo structure that specifies
     criteria for selecting the socket address structures returned in the
      list pointed to by res */
  s = getaddrinfo (NULL, port, &hints, &result);
  
  if (s != 0)
    {
    //   fprintf (stderr, "getaddrinfo: %s\n", gai_strerror (s));
      return -1;
    }
 
  for (rp = result; rp != NULL; rp = rp->ai_next)
    { 
      /*socket() creates an endpoint for communication and returns a file
       descriptor that refers to that endpoint.*/
      sfd = socket (rp->ai_family, rp->ai_socktype, rp->ai_protocol);
  
      if (sfd == -1)
        continue;
    /* bind() assigns
       the address specified by addr to the socket referred to by the file
       descriptor sockfd.*/
       
      s = bind (sfd, rp->ai_addr, rp->ai_addrlen);
      
      if (s == 0)
        { //printf("\nsfd:%d",sfd);
          s=sfd;
          // printf("\n* We managed to bind successfully! *\n");
          freeaddrinfo (result);     //to free ---- structure addrinfo *result
          return sfd;
        }
          // printf("after break");
      close (sfd);
    }

  if (rp == NULL)
    {
      fprintf (stderr, "Could not bind\n");
      return -1;
    }

}
