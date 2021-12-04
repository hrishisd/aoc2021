def score_game(boards, draws):
    def fill_if_contains(draw, board):
        for i, row in enumerate(board):
            for j, x in enumerate(row):
                if x == draw:
                    board[i][j] = None

    def is_winner(board):
        def columns(board):
            return ((row[i] for row in board) for i in range(len(board)))

        def any_seq_of_all_none(seqs):
            return any(all(x is None for x in seq) for seq in seqs)

        return any_seq_of_all_none(board) or any_seq_of_all_none(columns(board))

    def partition(seq, predicate):
        yes, no = [], []
        for x in seq:
            if predicate(x):
                yes.append(x)
            else:
                no.append(x)
        return yes, no

    def sum_remaining(board):
        return sum(x for row in board for x in row if x is not None)

    candidates = boards
    scores = []
    candidates = boards
    for draw in draws:
        for board in boards:
            fill_if_contains(draw, board)
        winners, candidates = partition(candidates, is_winner)
        for board in winners:
            scores.append(draw * sum_remaining(board))
    return scores


with open("../input.txt") as f:

    def parse_boards(lines):
        def parse_board(from_idx):
            def parse_row(row):
                return [int(s) for s in row.split() if s != ""]

            return [parse_row(lines[from_idx + i]) for i in range(5)]

        return [parse_board(i) for i in range(1, len(lines), 6)]

    lines = (line for line in f)
    draws = [int(s) for s in next(lines).strip().split(",")]
    remaining_lines = list(lines)
    boards = parse_boards(remaining_lines)
    scores = score_game(boards, draws)
    print("part 1:", scores[0])
    print("part 2:", scores[-1])
