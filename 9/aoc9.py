from __future__ import annotations
from dataclasses import dataclass, field
from enum import Enum


class Direction(Enum):
    RIGHT = 'R'
    LEFT = 'L'
    UP = 'U'
    DOWN = 'D'
    DIAGONAL = 'd'


@dataclass
class Knot:
    x: int = field(default=0)
    y: int = field(default=0)
    next: Knot | None = field(default=None, compare=False, repr=False)


head, tail = Knot(), Knot()
tail_history: set[tuple[int, int]] = set()
tail_history.add((tail.x, tail.y))


def move_head(direction: Direction, amount: int):
    """Moves the head a certain amount of positions in the given direction."""
    for _ in range(amount):
        match(direction):
            case Direction.RIGHT:
                head.x += 1
            case Direction.LEFT:
                head.x -= 1
            case Direction.UP:
                head.y += 1
            case Direction.DOWN:
                head.y -= 1
        move_next(head, head.next)
        tail_history.add((tail.x, tail.y))


def find_movement(knot: Knot, next_knot: Knot) -> Direction:
    """Finds the movement of a knot relatively to the next one."""
    mov: set[Direction] = set()
    if knot.x > 1 + next_knot.x:
        mov.add(Direction.RIGHT)
    if knot.x < next_knot.x - 1:
        mov.add(Direction.LEFT)
    if knot.y > 1 + next_knot.y:
        mov.add(Direction.UP)
    if knot.y < next_knot.y - 1:
        mov.add(Direction.DOWN)
    if len(mov) > 1:
        return Direction.DIAGONAL
    return mov.pop()


def move_next(knot: Knot, next: Knot) -> None:
    """Moves the next knot in order to follow the previous one."""
    if (next == knot or
            abs(next.x - knot.x) <= 1 and abs(next.y - knot.y) <= 1):
        return
    is_diag = next.x != knot.x and next.y != knot.y
    move = find_movement(knot, next)
    match(move):
        case Direction.RIGHT:
            next.x += 1
            if is_diag:
                next.y = knot.y
        case Direction.LEFT:
            next.x -= 1
            if is_diag:
                next.y = knot.y
        case Direction.UP:
            next.y += 1
            if is_diag:
                next.x = knot.x
        case Direction.DOWN:
            next.y -= 1
            if is_diag:
                next.x = knot.x
        case Direction.DIAGONAL:
            next.x = (next.x + knot.x) / 2
            next.y = (next.y + knot.y) / 2
    if next.next is not None:
        move_next(next, next.next)


# Part 1
head.next = tail

with open('inputs/9', 'r') as file:
    for line in file:
        l = line.rstrip('\n').split(' ', 2)
        move_head(Direction(l[0]), int(l[1]))

print(len(tail_history))

# Part 2
head.x, head.y, tail.x, tail.y = 0, 0, 0, 0
tail_history.clear()
tail_history.add((tail.x, tail.y))

curr = head
for _ in range(8):
    curr.next = Knot()
    curr = curr.next
curr.next = tail

with open('inputs/9', 'r') as file:
    for line in file:
        l = line.rstrip('\n').split(' ', 2)
        move_head(Direction(l[0]), int(l[1]))

print(len(tail_history))
