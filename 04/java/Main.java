import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.stream.IntStream;
import java.io.IOException;

import static java.util.stream.Collectors.toList;

class Main {
    public static void main(String[] args) throws IOException {
        var blocks = Files.readString(Path.of("../input.txt")).split("\n\n");
        var draws = Arrays.stream(blocks[0].split(",")).map(Integer::parseInt).collect(toList());
        var boards = Arrays.stream(blocks).skip(1).map(Main::parseBoard).collect(toList());
        var scores = scoreGame(boards, draws);
        System.out.println("part 1: " + scores.get(0));
        System.out.println("part 2: " + scores.get(scores.size() - 1));
    }

    static List<Integer> scoreGame(List<int[][]> boards, List<Integer> draws) {
        var result = new ArrayList<Integer>();
        for (int draw : draws) {
            for (var board : boards) {
                mark(board, draw);
            }
            for (int i = boards.size() - 1; i >= 0; i--) {
                if (hasBingo(boards.get(i))) {
                    result.add(draw * unmarkedSum(boards.get(i)));
                    boards.remove(i);
                }
            }
        }
        return result;
    }

    private static void mark(int[][] board, int draw) {
        for (var row : board) {
            for (int i = 0; i < row.length; i++) {
                if (row[i] == draw) {
                    row[i] = 0;
                    return;
                }
            }
        }
    }

    private static boolean hasBingo(int[][] board) {
        var horizontalBingo = Arrays.stream(board).anyMatch(row -> Arrays.equals(row, new int[] { 0, 0, 0, 0, 0 }));
        var verticalBingo = IntStream.range(0, 5)
                .anyMatch(colIdx -> IntStream.range(0, 5).allMatch(rowIdx -> board[rowIdx][colIdx] == 0));
        return horizontalBingo || verticalBingo;
    }

    private static int unmarkedSum(int[][] board) {
        int result = 0;
        for (var row : board) {
            for (int square : row) {
                result += square;
            }
        }
        return result;
    }

    static int[][] parseBoard(String block) {
        return block.lines().map(String::strip)
                .map(line -> Arrays.stream(line.split("\\s+")).mapToInt(Integer::parseInt).toArray())
                .toArray(int[][]::new);
    }
}