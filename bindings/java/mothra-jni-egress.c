#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <assert.h>
#include "mothra-jni-egress.h"
#include "mothra-jni-ingress.h"

JNIEXPORT void JNICALL Java_net_p2p_mothra_Start (JNIEnv *jenv, jclass jcls, jobjectArray jargs){
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

JNIEXPORT void JNICALL Java_net_p2p_mothra_SendGossip (JNIEnv *jenv, jclass jcls, jbyteArray jtopic, jbyteArray jdata){
    int data_length = (*jenv)->GetArrayLength(jenv, jdata);
    jbyte *topic = (jbyte *) 0 ;
    jbyte *data = (jbyte *) 0 ;
    if (jtopic) {
        jboolean isCopy = JNI_TRUE;
        topic = (*jenv)->GetByteArrayElements(jenv,jtopic,&isCopy);
        if (!topic) return ;
    }
    if (jdata) {
        jboolean isCopy = JNI_TRUE;
        data = (*jenv)->GetByteArrayElements(jenv,jdata,&isCopy);
        if (!data) return ;
    }
    libp2p_send_gossip(topic,data,data_length);
    if (topic) (*jenv)->ReleaseByteArrayElements(jenv, jtopic, topic, 0);
    if (data) (*jenv)->ReleaseByteArrayElements(jenv, jdata, data, 0);
}