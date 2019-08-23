package net.p2p;

import java.util.function.BiFunction;

public class mothra {
    public static final String NAME = System.getProperty("user.dir") + "/libmothra-egress.dylib"; 
    public static BiFunction<String, byte[], Boolean> ReceivedGossipMessage;
    public static native void Init();
    public static native void Start(String[] args);
    public static native void SendGossip(byte[] topic, byte[] message);
    public static void ReceiveGossip(byte[] topic, byte[] message) {
        ReceivedGossipMessage.apply(new String(topic), message);
    }
    static {
        try {
            System.load ( NAME ) ;
        } catch (UnsatisfiedLinkError e) {
          System.err.println("Native code library failed to load.\n" + e);
          System.exit(1);
        }
    }
}