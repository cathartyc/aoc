import re
from collections import namedtuple

Coordinates = namedtuple('Coordinates', ['x', 'y'])

def manhattan_distance(start: Coordinates, end: Coordinates):
    return abs(end.x - start.x) + abs(end.y - start.y)

sensors_and_manhattan_distance: set[tuple[Coordinates, int]] = set()

with open('inputs/15', 'r') as file:
    for line in file:
        coords = re.findall('x=(-?\d+), y=(-?\d+)', line)
        sensor = Coordinates(*map(lambda x: int(x), coords[0]))
        beacon = Coordinates(*map(lambda x: int(x), coords[1]))
        max_distance = manhattan_distance(sensor, beacon)
        sensors_and_manhattan_distance.add((sensor, max_distance))

possible_places = set()
for x in range(4000000 + 1):
    for y in range(4000000 + 1):
        for sam in sensors_and_manhattan_distance:
            if manhattan_distance(Coordinates(x, y), sam[0]) <= sam[1]:
                break
        else:
            possible_places.add(Coordinates(x, y))

print(len(possible_places))
pass