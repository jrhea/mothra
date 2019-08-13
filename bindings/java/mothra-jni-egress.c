#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <assert.h>
#include "mothra-jni-egress.h"
#include "mothra-jni-ingress.h"

JNIEXPORT void JNICALL Java_mothra_Start (JNIEnv *jenv, jclass jcls, jobjectArray jargs){
    int length = (*jenv)->GetArrayLength(jenv, jargs);
    char **args = (char **) malloc(length * sizeof(char *));
    if(args){
        for (int i=0; i<length; i++) {
            jstring jarg = (jstring) ((*jenv)->GetObjectArrayElement(jenv, jargs, i));
            const char *arg = (*jenv)->GetStringUTFChars(jenv, jarg, 0);
            args[i] = (char*) malloc(strlen(arg)*sizeof(char*));
            strcpy(args[i],arg);
            (*jenv)->ReleaseStringUTFChars(jenv, jarg, (const char *)arg);
        }
    }
    else{
        return;
    }
    libp2p_start(args, length);
    for (int i=0; i<length; i++) {
        free(args[i]);
    }
    free(args);
}

JNIEXPORT void JNICALL Java_mothra_SendGossip (JNIEnv *jenv, jclass jcls, jstring jmessage){
  char *message = (char *) 0 ;
  if (jmessage) {
    message = (char *)(*jenv)->GetStringUTFChars(jenv, jmessage, 0);
    if (!message) return ;
  }
  libp2p_send_gossip(message);
  if (message) (*jenv)->ReleaseStringUTFChars(jenv, jmessage, (const char *)message);
}