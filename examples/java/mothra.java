public class mothra {
    public static final String NAME = "mothrajni"; 
    public static native void StartLibP2P();
    static {
        try {
            System.loadLibrary ( NAME ) ;
            
        } catch (UnsatisfiedLinkError e) {
          System.err.println("Native code library failed to load.\n" + e);
          System.exit(1);
        }
    }
}