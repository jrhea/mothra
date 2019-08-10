#ifndef _MOTHRA_JNI_H_
#define _MOTHRA_JNI_H_
#include <jni.h>

#ifdef __cplusplus
extern "C" {
#endif

JNIEXPORT void JNICALL Java_mothra_StartLibP2P (JNIEnv *, jclass, jobjectArray);

#ifdef __cplusplus
}
#endif

#endif // _MOTHRA_JNI_H_
