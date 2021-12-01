import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;
import java.util.stream.IntStream;

import static java.util.stream.Collectors.toList;

class Main {
    public static void main(String[] args) throws IOException {
        var depths = Files.readAllLines(Path.of("../input.txt")).stream().map(Integer::parseInt)
                .collect(toList());
        System.out.println("part 1: " + part1(depths));
        System.out.println("part 2: " + part2(depths));
    }

    static long part1(List<Integer> depths) {
        return IntStream.range(1, depths.size()).filter(i -> depths.get(i - 1) < depths.get(i)).count();
    }

    static long part2(List<Integer> depths) {
        var windowSums = IntStream.range(2, depths.size())
                .map(i -> depths.get(i - 2) + depths.get(i - 1) + depths.get(i)).boxed().collect(toList());
        return part1(windowSums);
    }
}