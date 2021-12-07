import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.Arrays;

class Main {
    public static void main(String[] args) throws IOException {
        var xs = Arrays.stream(Files.readString(Path.of("../input.txt")).trim().split(","))
                .mapToLong(Long::parseLong).sorted().toArray();
        System.out.println(part1(xs));
        System.out.println(part2(xs));
    }

    static long part1(long[] xs) {
        long median = (xs[xs.length / 2] + xs[(xs.length - 1) / 2]) / 2;
        return Arrays.stream(xs).map(x -> Math.abs(x - median)).sum();
    }

    static long part2(long[] xs) {
        long avg = (long) ((double) Arrays.stream(xs).sum() / (double) xs.length);
        return Arrays.stream(xs).map(x -> Math.abs(x - avg)).map(i -> i * (i + 1) / 2).sum();
    }
}