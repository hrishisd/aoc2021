import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.Arrays;
import java.util.List;
import java.util.Map;
import java.util.function.BiFunction;
import java.util.stream.IntStream;

import static java.util.stream.Collectors.partitioningBy;;

class Main {
    public static void main(String[] args) throws IOException {
        var bitstrings = Files.readAllLines(Path.of("../input.txt"));
        System.out.println(part1(bitstrings));
        System.out.println(part2(bitstrings));
    }

    static int part1(List<String> bitstrings) {
        var n = bitstrings.get(0).length();
        var gammaBits = new char[n];
        Arrays.fill(gammaBits, '0');
        IntStream.range(0, n).filter(i -> numOnesSet(bitstrings, i) > bitstrings.size() / 2)
                .forEach(i -> gammaBits[i] = '1');
        var gamma = new String(gammaBits);
        var epsilon = complement(gamma);
        return toInt(gamma) * toInt(epsilon);
    }

    static int part2(List<String> bitstrings) {
        var o2 = filterBySelector(bitstrings, 0, (zeros, ones) -> ones.size() >= zeros.size() ? ones : zeros);
        var co2 = filterBySelector(bitstrings, 0, (zeros, ones) -> zeros.size() <= ones.size() ? zeros : ones);
        return toInt(o2) * toInt(co2);
    }

    static String filterBySelector(List<String> candidates, int i,
            BiFunction<List<String>, List<String>, List<String>> selector) {
        if (candidates.size() == 1) {
            return candidates.get(0);
        }
        Map<Boolean, List<String>> partitions = candidates.stream()
                .collect(partitioningBy(bits -> bits.charAt(i) == '1'));
        return filterBySelector(selector.apply(partitions.get(false), partitions.get(true)), i + 1, selector);
    }

    static String complement(String bits) {
        var result = new StringBuilder(bits.length());
        for (char c : bits.toCharArray()) {
            result.append(c == '1' ? '0' : '1');
        }
        return result.toString();
    }

    static long numOnesSet(List<String> bitstrings, int idx) {
        return bitstrings.stream().filter(bits -> bits.charAt(idx) == '1').count();
    }

    static int toInt(String bitstring) {
        var n = bitstring.length();
        return IntStream.range(0, n).filter(i -> bitstring.charAt(i) == '1').map(i -> 1 << (n - i - 1)).sum();
    }
}