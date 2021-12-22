import itertools
import functools
from collections import Counter


PLAYER_1_START_POS = 10
PLAYER_2_START_POS = 4


def part1():
    curr_dice_roll = 0

    def roll():
        nonlocal curr_dice_roll
        curr_dice_roll = (curr_dice_roll % 100) + 1
        return curr_dice_roll

    player1_pos, player2_pos = PLAYER_1_START_POS, PLAYER_2_START_POS
    player1_score, player2_score = 0, 0
    for turn in itertools.count(start=1):
        cumulative_roll = roll() + roll() + roll()
        if turn % 2 == 1:
            player1_pos = (player1_pos + cumulative_roll - 1) % 10 + 1
            player1_score += player1_pos
        else:
            player2_pos = (player2_pos + cumulative_roll - 1) % 10 + 1
            player2_score += player2_pos

        if player1_score >= 1000 or player2_score >= 1000:
            return min(player1_score, player2_score) * 3 * turn


CUMULATIVE_ROLL_COUNTS = Counter(map(sum, itertools.product((1, 2, 3), repeat=3)))


def part2():
    @functools.lru_cache(maxsize=None)
    def rec(curr_player_score, curr_player_pos, other_player_score, other_player_pos):
        if other_player_score >= 21:
            return 0, 1
        num_wins, num_losses = 0, 0
        for cum_roll, count in CUMULATIVE_ROLL_COUNTS.items():
            next_pos = (curr_player_pos - 1 + cum_roll) % 10 + 1
            other_player_wins, other_player_losses = rec(
                other_player_score,
                other_player_pos,
                curr_player_score + next_pos,
                next_pos,
            )
            num_wins += count * other_player_losses
            num_losses += count * other_player_wins

        return num_wins, num_losses

    return max(rec(0, PLAYER_1_START_POS, 0, PLAYER_2_START_POS))


print(part1())
print(part2())
