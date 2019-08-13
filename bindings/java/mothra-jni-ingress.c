#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include "mothra-jni-ingress.h"

JNIEXPORT JNIEnv* JNICALL create_vm(JavaVM **jvm)
{
    printf("create_vm0\n");
    JNIEnv* env;
    JavaVMInitArgs args;
    JavaVMOption options;
    printf("create_vm1\n");
    args.version = JNI_VERSION_10;
    args.nOptions = 1;
    options.optionString = "-Djava.class.path=/Library/Java/JavaVirtualMachines/openjdk-11.0.1.jdk/Contents/Home/lib/server/";
    args.options = &options;
    args.ignoreUnrecognized = 0;
    int rv;
    printf("create_vm4\n");
    rv = JNI_CreateJavaVM(jvm, (void**)&env, &args);
    printf("create_vm5\n");
    if (rv < 0 || !env)
        printf("Unable to Launch JVM %d\n",rv);
    else
        printf("Launched JVM! :)\n");
    return env;
}

void receive_gossip(char* message) {
    JavaVM *jvm;
    JNIEnv *env;
    env = create_vm(&jvm);
    if(env != NULL) {
        jclass mothra_class;
        jmethodID receivegossip_method;
        mothra_class = (*env)->FindClass(env, "mothra");
        jstring jmessage = (*env)->NewStringUTF(env, message);
        receivegossip_method = (*env)->GetStaticMethodID(env, mothra_class, "ReceiveGossip", "([Ljava/lang/String;)V");
        (*env)->CallStaticVoidMethod(env, mothra_class, receivegossip_method, jmessage);
    }
}
