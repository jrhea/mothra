#ifndef _MOTHRA_H_
#define _MOTHRA_H_

#ifdef __cplusplus
extern "C" {
#endif

void libp2p_start(char**, int length);
void libp2p_send_gossip(char*, int length);

#ifdef __cplusplus
}
#endif

#endif // _MOTHRA_JNI_H_