
public class mothra {
    public static final String NAME = "mothra-egress"; 
    public static native void Init();
    public static native void Start(String[] args);
    public static native void SendGossip(byte[] message);
    public static void ReceiveGossip(byte[] message){
        System.out.println("Java: received this message from another peer - " + new String(message));
    }
    static {
        try {
            System.loadLibrary ( NAME ) ;
            
        } catch (UnsatisfiedLinkError e) {
          System.err.println("Native code library failed to load.\n" + e);
          System.exit(1);
        }
    }
}