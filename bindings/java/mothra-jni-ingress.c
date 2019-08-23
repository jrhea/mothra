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

void receive_gossip(unsigned char* topic, int topic_length, unsigned char* data, int data_length) {
    JNIEnv *jenv;
    jint rs = (*jvm)->AttachCurrentThread(jvm, (void**)&jenv, NULL);
    assert (rs == JNI_OK);
    if(jenv != NULL) {
        jclass mothra_class;
        jmethodID receivegossip_method;
        jbyteArray jtopic;
        jbyteArray jdata;
        mothra_class = (*jenv)->FindClass(jenv, "net/p2p/mothra");
        if(!mothra_class){
            detach(jenv);
        }
        //Put the native unsigned chars in the java byte array
        jtopic = (*jenv)->NewByteArray(jenv, topic_length);
        jdata = (*jenv)->NewByteArray(jenv, data_length);
        (*jenv)->SetByteArrayRegion(jenv, jdata, 0, data_length, (jbyte *)data);
        (*jenv)->SetByteArrayRegion(jenv, jtopic, 0, topic_length, (jbyte *)topic);
        if(!jdata || !jtopic){
            detach(jenv);
        }
        receivegossip_method = (*jenv)->GetStaticMethodID(jenv, mothra_class, "ReceiveGossip", "([B[B)V");
        if(!receivegossip_method){
            detach(jenv);
        }
        (*jenv)->CallStaticVoidMethod(jenv, mothra_class, receivegossip_method, jtopic, jdata);
    }
}

void detach(JNIEnv* jenv){
    if((*jenv)->ExceptionOccurred(jenv)) {
        (*jenv)->ExceptionDescribe(jenv);
    }
    (*jvm)->DetachCurrentThread(jvm);
}
