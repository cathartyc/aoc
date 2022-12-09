# Part 1
def move_head(direction: str, amount: int):
    """Moves the head a certain amount of positions in the given direction."""
    for _ in range(amount):
        match(direction):
            case 'R':
                head_pos[0] += 1
            case 'L':
                head_pos[0] -= 1
            case 'U':
                head_pos[1] += 1
            case 'D':
                head_pos[1] -= 1
        move_tail(direction)


def move_tail(head_direction: str):
    """Moves the tail in order to follow the head."""
    if (tail_pos == head_pos or
        abs(tail_pos[0] - head_pos[0]) <= 1 and
        abs(tail_pos[1] - head_pos[1]) <= 1):
            return
    is_diag = tail_pos[0] != head_pos[0] and tail_pos[1] != head_pos[1]
    match (head_direction):
        case 'R':
            tail_pos[0] += 1
            if is_diag:
                tail_pos[1] = head_pos[1]
        case 'L':
            tail_pos[0] -= 1
            if is_diag:
                tail_pos[1] = head_pos[1]
        case 'U':
            tail_pos[1] += 1
            if is_diag:
                tail_pos[0] = head_pos[0]
        case 'D':
            tail_pos[1] -= 1
            if is_diag:
                tail_pos[0] = head_pos[0]
    tail_history.add(tuple(tail_pos))
    pass


head_pos, tail_pos = [0, 0], [0, 0]
tail_history: set[tuple[int, int]] = set()
tail_history.add(tuple(tail_pos))

with open('inputs/9', 'r') as file:
    for line in file:
        l = line.rstrip('\n').split(' ', 2)
        move_head(l[0], int(l[1]))

print(len(tail_history))
