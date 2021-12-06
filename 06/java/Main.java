import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.Arrays;

class Main {
    public static void main(String[] args) throws IOException {
        long[] state = new long[9];
        Arrays.stream(Files.readString(Path.of("../input.txt")).trim().split(","))
                .mapToInt(Integer::parseInt)
                .forEach(i -> state[i]++);

        for (int day = 1; day <= 256; day++) {
            long newFish = state[0];
            for (int idx = 0; idx < 8; idx++) {
                state[idx] = state[idx + 1];
            }
            state[8] = newFish;
            state[6] += newFish;

            if (day == 80 || day == 256) {
                System.out.println(Arrays.stream(state).sum());
            }
        }
    }
}