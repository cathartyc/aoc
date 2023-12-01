# Part 2
shapes = ('A', 'B', 'C')
outcome = ('X', 'Y', 'Z')


def move(opponent: str, oc: str) -> int:
    """Returns the move to do in order to obtain a certain outcome, given
    the move of the opponent.
    """
    return (outcome.index(oc) + shapes.index(opponent) - 1) % 3


points = 0

with open('inputs/2.txt', 'r') as file:
    for line in file:
        opponent, oc = line.split()
        points += outcome.index(oc) * 3 + move(opponent, oc) + 1

print(points)
