import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.lang.Thread;

public class app {
    public static void main(String[] args) throws InterruptedException {

        Runnable run = () -> {
            mothra.StartLibP2P();
        };
        Executors.newSingleThreadExecutor().execute(run);
    }
}