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
  sprintf( port, "%d", iport ); //to convert int char*
  memset (&hints, 0, sizeof (struct addrinfo));  
  hints.ai_family = AF_UNSPEC;     /* Return IPv4 and IPv6 choices */
  hints.ai_socktype = SOCK_STREAM; /* We want a TCP socket */
  hints.ai_flags = AI_PASSIVE;     /* All interfaces */
  // printf(" port:%s\n",port);
  
  s = getaddrinfo (NULL, port, &hints, &result);
  
  if (s != 0)
    {
    //   fprintf (stderr, "getaddrinfo: %s\n", gai_strerror (s));
      return -1;
    }
 
  for (rp = result; rp != NULL; rp = rp->ai_next)
    { 
      sfd = socket (rp->ai_family, rp->ai_socktype, rp->ai_protocol);
  
      if (sfd == -1)
        continue;
  
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
