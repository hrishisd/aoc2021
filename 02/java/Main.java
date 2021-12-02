import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;
import static java.util.stream.Collectors.toList;

class Main {
    enum Direction {
        Vertical, Horizontal;
    }

    record Command(Direction direction, int val) {
        static Command parse(String s) {
            var split = s.split(" ");
            var direction = split[0];
            int val = Integer.parseInt(split[1]);
            return switch (direction) {
                case "up" -> new Command(Direction.Vertical, -val);
                case "down" -> new Command(Direction.Vertical, val);
                case "forward" -> new Command(Direction.Horizontal, val);
                default -> throw new IllegalArgumentException("bad command string: " + s);
            };
        }
    };

    record Position(int x, int depth) {
        static final Position ZERO = new Position(0, 0);

        Position update(Command cmd) {
            return switch (cmd.direction) {
                case Vertical -> new Position(x, depth + cmd.val);
                case Horizontal -> new Position(x + cmd.val, depth);
            };
        }

        int product() {
            return x * depth;
        }
    }

    record PositionWithAim(Position pos, int aim) {
        PositionWithAim update(Command c) {
            return switch (c.direction) {
                case Vertical -> new PositionWithAim(pos, aim + c.val);
                case Horizontal -> new PositionWithAim(new Position(pos.x + c.val, pos.depth + aim * c.val), aim);
            };
        }
    }

    static int part1(List<Command> commands) {
        var p = Position.ZERO;
        for (var cmd : commands) {
            p = p.update(cmd);
        }
        return p.product();
    }

    static int part2(List<Command> commands) {
        var p = new PositionWithAim(Position.ZERO, 0);
        for (var command : commands) {
            p = p.update(command);
        }
        return p.pos.product();
    }

    public static void main(String[] args) throws IOException {
        var lines = Files.readAllLines(Path.of("../input.txt"));
        var commands = lines.stream().map(Command::parse).collect(toList());
        System.out.println(part1(commands));
        System.out.println(part2(commands));
    }
}