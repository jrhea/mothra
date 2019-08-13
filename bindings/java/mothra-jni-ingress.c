#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <assert.h>
#include "mothra-jni-ingress.h"

static JavaVM *jvm;

JNIEXPORT void JNICALL Java_mothra_Init(JNIEnv* jenv, jclass jcls)
{
   jint rs = (*jenv)->GetJavaVM(jenv, &jvm);
   assert (rs == JNI_OK);
}

void receive_gossip(char* message) {
    JNIEnv *jenv;
    jint rs = (*jvm)->AttachCurrentThread(jvm, (void**)&jenv, NULL);
    assert (rs == JNI_OK);
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
