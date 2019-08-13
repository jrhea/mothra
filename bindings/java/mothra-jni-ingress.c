#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <assert.h>
#include "mothra-jni-ingress.h"

static JavaVM *jvm;

JNIEXPORT void JNICALL Java_mothra_Init(JNIEnv* jenv, jclass jcls)
{
   printf("Java_mothra_Init: start\n");
   jint rs = (*jenv)->GetJavaVM(jenv, &jvm);
   printf("Java_mothra_Init: rs = %i\n",rs);
   assert (rs == JNI_OK);
   printf("Java_mothra_Init: end\n");
}

void receive_gossip(char* message) {

    JNIEnv *jenv;
    printf("receive_gossip: before attach\n");
    jint rs = (*jvm)->AttachCurrentThread(jvm, (void**)&jenv, NULL);
    printf("receive_gossip: rs = %i\n",rs);
    printf("receive_gossip: after attach\n");
    assert (rs == JNI_OK);
    printf("jenv: %p\n",jenv);
    if(jenv != NULL) {
        jclass mothra_class;
        jmethodID receivegossip_method;
        jstring jmessage;
        mothra_class = (*jenv)->FindClass(jenv, "mothra");
        if(!mothra_class){
            detach(jenv);
        }
        jmessage = (*jenv)->NewStringUTF(jenv, message);
        if(!jmessage){
            detach(jenv);
        }
        receivegossip_method = (*jenv)->GetStaticMethodID(jenv, mothra_class, "ReceiveGossip", "(Ljava/lang/String;)V");
        if(!receivegossip_method){
            detach(jenv);
        }
        (*jenv)->CallStaticVoidMethod(jenv, mothra_class, receivegossip_method, jmessage);
    }
}

void detach(JNIEnv* jenv){
    if((*jenv)->ExceptionOccurred(jenv)) {
        (*jenv)->ExceptionDescribe(jenv);
    }
    (*jvm)->DetachCurrentThread(jvm);
}
