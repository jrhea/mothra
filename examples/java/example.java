import java.util.List;
import java.util.ArrayList;
import java.util.Arrays;

public class example {
    public static void main(String[] args) throws InterruptedException {

        List<String> argList = new ArrayList<String>(Arrays.asList(args));
        argList.add(0,"./example");
        final String[] processed_args = argList.toArray(new String[0]);
        mothra.Init();
        mothra.Start(processed_args);
        while(true){
            mothra.SendGossip("Hello from Java");
            Thread.sleep(1000);
        }

    }
}