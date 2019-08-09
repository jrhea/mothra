#include "mothra-jni.h"
#include "../c/mothra.h"

JNIEXPORT void JNICALL Java_mothra_StartLibP2P (JNIEnv *jenv, jclass jcls){
    int length=1;
    char *arg[1]={"example"};
    libp2p_start(arg, length);
}