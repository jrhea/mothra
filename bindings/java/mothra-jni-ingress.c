#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <assert.h>
#include "mothra-jni-ingress.h"

static JavaVM *jvm;

JNIEXPORT void JNICALL Java_net_p2p_mothra_Init(JNIEnv* jenv, jclass jcls)
{
   jint rs = (*jenv)->GetJavaVM(jenv, &jvm);
   assert (rs == JNI_OK);
}

void receive_gossip(unsigned char* message, int length) {
    JNIEnv *jenv;
    jint rs = (*jvm)->AttachCurrentThread(jvm, (void**)&jenv, NULL);
    assert (rs == JNI_OK);
    if(jenv != NULL) {
        jclass mothra_class;
        jmethodID receivegossip_method;
        jbyteArray jmessage;
        mothra_class = (*jenv)->FindClass(jenv, "net/p2p/mothra");
        if(!mothra_class){
            detach(jenv);
        }
        //Put the native unsigned chars in the java byte array
        jmessage = (*jenv)->NewByteArray(jenv, length);
        (*jenv)->SetByteArrayRegion(jenv, jmessage, 0, length, (jbyte *)message);
        if(!jmessage){
            detach(jenv);
        }
        receivegossip_method = (*jenv)->GetStaticMethodID(jenv, mothra_class, "ReceiveGossip", "([B)V");
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
