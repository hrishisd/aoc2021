with open("../input.txt") as f:

    def parse_initial_state(line):
        timers = [int(i) for i in line.strip().split(",")]
        state = [0] * 9
        for t in timers:
            state[t] += 1
        return state

    def iter():
        births = state[0]
        for i in range(8):
            state[i] = state[i+1]
        state[6] += births
        state[8] = births

    state = parse_initial_state(f.readline())
    for day in range(1, 257):
        iter()
        if day in (80, 256):
            print(f"day {day}: {sum(state)}")
