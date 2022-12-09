# Part 2
def move_head(direction: str, amount: int):
    """Moves the head a certain amount of positions in the given direction."""
    for _ in range(amount):
        match(direction):
            case 'R':
                knots[0][0] += 1
            case 'L':
                knots[0][0] -= 1
            case 'U':
                knots[0][1] += 1
            case 'D':
                knots[0][1] -= 1
        for i in range(1, len(knots)):
            move_knot(i)
        tail_history.add(tuple(knots[-1]))


def find_movement(knot: list[int, int], next_knot: list[int, int]) -> str:
    """Finds the movement of a knot relatively to the next one."""
    mov = ''
    if knot[0] > 1 + next_knot[0]:
        mov += 'R'
    if knot[0] < next_knot[0] - 1:
        mov += 'L'
    if knot[1] > 1 + next_knot[1]:
        mov += 'U'
    if knot[1] < next_knot[1] - 1:
        mov += 'D'
    return mov


def move_knot(knot_index: int):
    """Moves the knot at the given index, according to the movement of
    the previous one.
    """
    knot = knots[knot_index]
    rel_head = knots[knot_index-1]
    if (knot == rel_head or
        abs(knot[0] - rel_head[0]) <= 1 and
        abs(knot[1] - rel_head[1]) <= 1):
            return
    is_diag = knot[0] != rel_head[0] and knot[1] != rel_head[1]
    head_direction = find_movement(rel_head, knot)
    for m in head_direction:
        match (m):
            case 'R':
                knot[0] += 1
                if is_diag and len(head_direction) == 1:
                    knot[1] = rel_head[1]
            case 'L':
                knot[0] -= 1
                if is_diag and len(head_direction) == 1:
                    knot[1] = rel_head[1]
            case 'U':
                knot[1] += 1
                if is_diag and len(head_direction) == 1:
                    knot[0] = rel_head[0]
            case 'D':
                knot[1] -= 1
                if is_diag and len(head_direction) == 1:
                    knot[0] = rel_head[0]


knots = tuple([0, 0] for _ in range(10))
tail_history: set[tuple[int, int]] = set()

with open('inputs/9', 'r') as file:
    for line in file:
        l = line.rstrip('\n').split(' ', 2)
        move_head(l[0], int(l[1]))
        tail_history.add(tuple(knots[-1]))

print(len(tail_history))
