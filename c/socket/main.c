#include <arpa/inet.h>
#include <netinet/in.h>
#include <stdio.h>
#include <sys/socket.h>
#include <unistd.h>

// Create a Socket implementation
int main() {
  int sockfd;
  // Socket Address Internet Style
  struct sockaddr_in serv_addr; //-> sys/netinet/in.h/struct sockaddr_in
  int nserver, flags;

  // Create socket file descriptor -> sys/socket.h/socket
  // This allow us to communicate with other processes through the network
  // [sys/socket.h/AF_INET] -> Address From Internet
  // [sys/socket.h/SOCK_STREAM] -> Stream Socket
  // IPPROTO_TCP -> IP Protocol Control TCP
  sockfd = socket(AF_INET, SOCK_STREAM, IPPROTO_TCP);
  if (sockfd < 0) {
    perror("socket creation failed"); //-> stdio.h/perror
    return 1;
  }

  // Set up the server address structure
  // family == Specifies the type of network protocol to be used
  // port == Specifies the port number to be used
  // address == Specifies the IP address to be used
  // s_addr == Specifies the server's IP address
  // netinet/in.h/INADDR_ANY -> 0.0.0.0
  serv_addr.sin_family = AF_INET;
  serv_addr.sin_port = htons(80);
  serv_addr.sin_addr.s_addr = INADDR_ANY;

  // Bind socket to a specific port and address -> sys/socket.h/bind
  if (bind(sockfd, (struct sockaddr *)&serv_addr, sizeof(serv_addr)) < 0) {
    perror("binding failed");
    return 1;
  }

  // Listen for incoming connections -> sys/socket.h/listen
  if (listen(sockfd, 5) < 0) {
    perror("listening failed");
    return 1;
  }

  printf("Waiting for incoming connections... \n");

  // Accept incoming connections and handle communication events
  while (1) {
    // Wait for incoming connection request -> sys/socket.h/accept
    if (accept(sockfd, NULL, NULL) < 0) {
      perror("accept failed");
      return 1;
    }

    // Accept incoming connection
    struct sockaddr_in cli_addr;
    int cli_sd =
        accept(sockfd, (struct sockaddr *)&cli_addr, (socklen_t *)&cli_sd);
    if (cli_sd < 0) {
      perror("accept failed");
      return 1;
    }

    // Handle incoming data and close the connection
    // ...
  }

  // Close socket file descriptor
  close(sockfd);
  return 0;
}
