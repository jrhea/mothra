#include <jni.h>
#ifndef _MOTHRA_JNI_INGRESS_H_
#define _MOTHRA_JNI_INGRESS_H_

#ifdef __cplusplus
extern "C" {
#endif

JNIEXPORT void JNICALL Java_net_p2p_mothra_Init(JNIEnv*,jclass);

void discovered_peer(const unsigned char*, int);
void receive_gossip(const unsigned char*, int, unsigned char*, int);
void receive_rpc(const unsigned char*, int, int, const unsigned char*, int, unsigned char*, int);

void detach(JNIEnv* );

#ifdef __cplusplus
}
#endif

#endif // _MOTHRA_JNI_INGRESS_H_
