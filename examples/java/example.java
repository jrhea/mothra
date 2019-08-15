import java.util.List;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Scanner;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import net.p2p.mothra;

public class example {
    public static void main(String[] args) throws InterruptedException {
        List<String> argList = new ArrayList<String>(Arrays.asList(args));
        argList.add(0,"./example");
        final String[] processed_args = argList.toArray(new String[0]);
        Runnable run = () -> {
            mothra.Init();
            mothra.Start(processed_args);
            mothra.ReceivedMessage = example::printMessage;
        };
        Executors.newSingleThreadExecutor().execute(run);
        Scanner scanner = new Scanner(System.in);
        while(true){
            System.out.print("Enter a message to send: ");
            String message = scanner.next();
            mothra.SendGossip(message.getBytes());
        }

    }

    public static Boolean printMessage(byte[] message){
        System.out.println("Java: received this message from another peer - " + new String(message));
        return true;
    }
}