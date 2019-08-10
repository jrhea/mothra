#include <unistd.h>
#include <stdio.h>
#include "mothra.h"

int main (int argc, char** argv) {
    libp2p_start(argv,argc);
    while(1){
        libp2p_send_gossip("The dude abides",15);
        sleep(1);
    }
    
}