#include <stdio.h>
#include <string.h>     
#include <arpa/inet.h>  
#include <sys/types.h>
#include <sys/socket.h>
#include <netdb.h>
#include <arpa/inet.h>
#include <netinet/in.h>
#include <unistd.h> 
#define BUFFERSIZE 1024

int main(){
    struct addrinfo hints, *res;
    char buf[BUFFERSIZE];
    memset(buf,0, BUFFERSIZE );


    memset(&hints, 0, sizeof hints);
    hints.ai_family = AF_UNSPEC;
    hints.ai_flags = AI_PASSIVE;
    hints.ai_socktype = SOCK_STREAM;

    int status = getaddrinfo(NULL, "3490", &hints, &res);
    if (status!=0){
        printf("ruh roh");
        return 1;
    }

    int s  = socket(res->ai_family, res->ai_socktype, res->ai_protocol);

    //usually j check for first working ai_addr, doing a naive approach rn
    int bv = bind(s, res->ai_addr, res->ai_addrlen);
    int lv = listen(s, 10);
    struct sockaddr_storage temp;
    socklen_t len = sizeof temp;
    int new_fd = accept(s, (struct sockaddr *)&temp, &len );
    int rv = recv(new_fd, buf, BUFFERSIZE-1, 0);
    close(s);
    close(new_fd);
    freeaddrinfo(res);


    printf("%s", buf);

}