#include <stdio.h>
#include "mothra.h"

void receive_gossip(unsigned char* message) {
    printf("C: received this message from another peer - %s\n",message);
}