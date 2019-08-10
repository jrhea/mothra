#include <unistd.h>
#include <stdio.h>
#include "mothra.h"

int main (int argc, char** argv) {
    libp2p_start(argv,argc);
    while(1){
        libp2p_send_gossip("Hello from C");
        sleep(1);
    }
    
}