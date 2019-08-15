package net.p2p;

import java.util.function.Function;

public class mothra {
    public static final String NAME = System.getProperty("user.dir") + "/libmothra-egress.dylib"; 
    public static Function<byte[], Boolean> ReceivedMessage;
    public static native void Init();
    public static native void Start(String[] args);
    public static native void SendGossip(byte[] message);
    public static void ReceiveGossip(byte[] message){
        ReceivedMessage.apply(message);
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