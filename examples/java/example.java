import java.util.concurrent.Executors;

public class example {
    public static void main(String[] args) throws InterruptedException {

        Runnable run = () -> {
            mothra.StartLibP2P();
        };
        Executors.newSingleThreadExecutor().execute(run);
    }
}