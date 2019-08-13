#include <jni.h>
#ifndef _MOTHRA_JNI_EGRESS_H_
#define _MOTHRA_JNI_EGRESS_H_

#ifdef __cplusplus
extern "C" {
#endif

JNIEXPORT void JNICALL Java_mothra_Start(JNIEnv *, jclass, jobjectArray);
JNIEXPORT void JNICALL Java_mothra_SendGossip(JNIEnv *, jclass, jstring);

extern void libp2p_start(char**, int);
extern void libp2p_send_gossip(char*);

#ifdef __cplusplus
}
#endif

#endif // _MOTHRA_JNI_EGRESS_H_
