#ifndef _MOTHRA_JNI_H_
#define _MOTHRA_JNI_H_
#include <jni.h>

#ifdef __cplusplus
extern "C" {
#endif

JNIEXPORT void JNICALL Java_mothra_Start (JNIEnv *, jclass, jobjectArray);
JNIEXPORT void JNICALL Java_mothra_SendGossip(JNIEnv *, jclass, jstring);

#ifdef __cplusplus
}
#endif

#endif // _MOTHRA_JNI_H_
