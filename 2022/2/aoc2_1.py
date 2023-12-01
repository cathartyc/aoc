# Part 1
opponent_shapes = ('A', 'B', 'C')
my_shapes = ('X', 'Y', 'Z')


def result(opponent: str, me: str) -> int:
    """Returns the result of the round: 0 for loss, 1 for draw, 2 for win."""
    return (my_shapes.index(me) - opponent_shapes.index(opponent) + 1) % 3


points = 0

with open('inputs/2', 'r') as file:
    for line in file:
        opponent, me = line.split()
        points += result(opponent, me) * 3 + my_shapes.index(me) + 1

print(points)
