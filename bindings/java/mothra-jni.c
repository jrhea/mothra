#include "mothra-jni.h"
#include "../c/mothra.h"

JNIEXPORT void JNICALL Java_mothra_StartLibP2P (JNIEnv *jenv, jclass jcls){
    libp2p_init();
}