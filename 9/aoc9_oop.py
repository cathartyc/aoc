# This exists because my original code was not enough oop.
from aoc9_oop_utils import Knot, Direction, TailHistory


head = Knot()
tail = Knot()
tail_history = TailHistory()
tail_history.add_position(tail.coordinates)


def move_head(direction: Direction, amount: int) -> None:
    """Moves the head a certain amount of positions in the given direction."""
    for _ in range(amount):
        match(direction):
            case Direction.RIGHT:
                head.move_right()
            case Direction.LEFT:
                head.move_left()
            case Direction.UP:
                head.move_up()
            case Direction.DOWN:
                head.move_down()
        move_next(head, head.next)
        tail_history.add_position(tail.coordinates)

def move_next(knot: Knot, next: Knot) -> None:
    """Moves the next knot in order to follow the previous one."""
    if next.coordinates == knot.coordinates or next.is_near_enough_to(knot):
        return
    is_diag = next.is_not_in_the_same_row_or_column_as(knot)
    if knot.is_upper_and_more_to_the_right_than(next):
        next.move_diagonal_up_right()
    elif knot.is_upper_and_more_to_the_left_than(next):
        next.move_diagonal_up_left()
    elif knot.is_lower_and_more_to_the_right_than(next):
        next.move_diagonal_down_right()
    elif knot.is_lower_and_more_to_the_left_than(next):
        next.move_diagonal_down_left()
    elif knot.is_more_to_the_right_than(next):
        next.move_right()
        if is_diag:
            next.coordinates.y = knot.coordinates.y
    elif knot.is_more_to_the_left_than(next):
        next.move_left()
        if is_diag:
            next.coordinates.y = knot.coordinates.y
    elif knot.is_upper_than(next):
        next.move_up()
        if is_diag:
            next.coordinates.x = knot.coordinates.x
    elif knot.is_lower_than(next):
        next.move_down()
        if is_diag:
            next.coordinates.x = knot.coordinates.x
    if next.next is not None:
        move_next(next, next.next)


# Part 1
head.next = tail

with open('inputs/9', 'r') as file:
    for line in file:
        l = line.rstrip('\n').split(' ', 2)
        move_head(Direction(l[0]), int(l[1]))

print(tail_history.length())

# Part 2

head = Knot()
tail = Knot()
tail_history.reset()
tail_history.add_position(tail.coordinates)

curr = head
for _ in range(7):
    curr.next = Knot(prev=curr)
    curr = curr.next
curr.next = Knot(prev=curr, next=tail)

with open('inputs/9', 'r') as file:
    for line in file:
        l = line.rstrip('\n').split(' ', 2)
        move_head(Direction(l[0]), int(l[1]))

print(tail_history.length())
