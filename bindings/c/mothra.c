#include "mothra.h"
#include <stdio.h>

void libp2p_start_bind (char** args, int length) {
    printf("FOO2\n");
    libp2p_start(args,length);
}