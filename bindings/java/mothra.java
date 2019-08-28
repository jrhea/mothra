package net.p2p;

import java.util.Objects;
import java.util.function.Function;
import java.util.function.BiFunction;

public class mothra {
    public static final String NAME = System.getProperty("user.dir") + "/libmothra-egress.dylib"; 
    public static BiFunction<String, byte[], Boolean> ReceivedGossipMessage;
    public static TriFunction<String, String, byte[], Boolean> ReceivedRPCMessage;
    public static native void Init();
    public static native void Start(String[] args);
    public static native void SendGossip(byte[] topic, byte[] message);
    public static void ReceiveGossip(byte[] topic, byte[] message) {
        ReceivedGossipMessage.apply(new String(topic), message);
    }
    public static native void SendRPC(byte[] method, byte[] peer, byte[] message);
    public static void ReceiveRPC(byte[] method, byte[] peer, byte[] message) {
        ReceivedRPCMessage.apply(new String(method), new String(peer), message);
    }
    static {
        try {
            System.load ( NAME ) ;
        } catch (UnsatisfiedLinkError e) {
          System.err.println("Native code library failed to load.\n" + e);
          System.exit(1);
        }
    }

    @FunctionalInterface
    public interface TriFunction<A,B,C,R> {
        R apply(A a, B b, C c);
        default <V> TriFunction<A, B, C, V> andThen(
                                    Function<? super R, ? extends V> after) {
            Objects.requireNonNull(after);
            return (A a, B b, C c) -> after.apply(apply(a, b, c));
        }
    }
}