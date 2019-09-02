#include <jni.h>
#ifndef _MOTHRA_JNI_EGRESS_H_
#define _MOTHRA_JNI_EGRESS_H_

#ifdef __cplusplus
extern "C" {
#endif

JNIEXPORT void JNICALL Java_net_p2p_mothra_Start(JNIEnv *, jclass, jobjectArray);
JNIEXPORT void JNICALL Java_net_p2p_mothra_SendGossip(JNIEnv *, jclass, jbyteArray, jbyteArray);

extern void libp2p_start(char**, int);
extern void libp2p_send_gossip(jbyte*, int, jbyte*, int);
extern void libp2p_send_rpc(jbyte*, int, jbyte*, int, jbyte*, int);

#ifdef __cplusplus
}
#endif

#endif // _MOTHRA_JNI_EGRESS_H_
