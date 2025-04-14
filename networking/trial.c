#include <stdio.h>
#include <string.h>     
#include <arpa/inet.h>  
#include <sys/types.h>
#include <sys/socket.h>
#include <netdb.h>
#include <arpa/inet.h>
#include <netinet/in.h>
#include <unistd.h>



void getaddr_test(){
    int status;
    struct addrinfo hints;
    struct addrinfo *ret, *p;
    char * user_entry = NULL;
    size_t len = 0;
    char ipstr[INET6_ADDRSTRLEN];

    memset(&hints, 0, sizeof hints);

    hints.ai_family = AF_UNSPEC;
    hints.ai_flags = AI_PASSIVE;
    hints.ai_socktype = SOCK_STREAM;


    printf("Enter an address: ");
    ssize_t read = getline(&user_entry, &len, stdin);
    printf("Line entered: %s", user_entry);

    user_entry[read-1] = '\0';
    status = getaddrinfo(user_entry, "3490", &hints, &ret);

    if (status != 0) {
        fprintf(stderr, "getaddrinfo error: %s\n", gai_strerror(status));
    }

    for(p=ret; p!=NULL; p=p->ai_next){
        void *addr;
        char *ipver;

        if (p->ai_family == AF_INET){
            struct sockaddr_in *ipv4 = (struct sockaddr_in *) p->ai_addr;
            addr = &(ipv4->sin_addr);
            ipver = "IPV4";
        }
        else{
            struct sockaddr_in6 *ipv6 = (struct sockaddr_in6 *)p->ai_addr;
            addr = &(ipv6->sin6_addr);
            ipver = "IPV6";
        }
        inet_ntop(p->ai_family, addr, ipstr, sizeof ipstr);
        printf("%s %s\n", ipstr, ipver);
    }
}

int main(){
    int status;
    struct addrinfo hints, *res;
    struct sockaddr flame_on;

    memset(&hints, 0, sizeof hints);
    hints.ai_family = AF_UNSPEC;
    hints.ai_flags = AI_PASSIVE;
    hints.ai_socktype = SOCK_STREAM;

    int s = getaddrinfo(NULL, "3490", &hints, &res);

    if (s!=0){
        printf("ruh roh");
        return 1;
    }

    s = socket(res->ai_family, res->ai_socktype, res->ai_protocol);
    connect(s, res->ai_addr, res->ai_addrlen);


    socklen_t addr_len = sizeof flame_on;
    int new_fd = accept(s, (struct sockaddr *)&flame_on, &addr_len);
    char *msg = "Ooga booga, server made";
    int len = strlen(msg);
    int  bs = send(s, msg, len, 0);
    printf("%d", bs);
    close(new_fd);
}