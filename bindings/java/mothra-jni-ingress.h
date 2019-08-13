#include <jni.h>
#ifndef _MOTHRA_JNI_INGRESS_H_
#define _MOTHRA_JNI_INGRESS_H_

#ifdef __cplusplus
extern "C" {
#endif

JNIEXPORT JNIEnv* JNICALL create_vm(JavaVM **);

void receive_gossip(char*);

#ifdef __cplusplus
}
#endif

#endif // _MOTHRA_JNI_H_
