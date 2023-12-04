import re
from collections import namedtuple

Coordinates = namedtuple('Coordinates', ['x', 'y'])

def manhattan_distance(start: Coordinates, end: Coordinates):
    return abs(end.x - start.x) + abs(end.y - start.y)

covered_cells: set[Coordinates] = set()
sensors_and_beacons: set[Coordinates] = set()

with open('inputs/15', 'r') as file:
    for line in file:
        coords = re.findall('x=(-?\d+), y=(-?\d+)', line)
        sensor = Coordinates(*map(lambda x: int(x), coords[0]))
        beacon = Coordinates(*map(lambda x: int(x), coords[1]))
        max_distance = manhattan_distance(sensor, beacon)
        for x in range(sensor.x - max_distance, sensor.x + max_distance + 1):
            for y in range(sensor.y - (max_distance - abs(x - sensor.x)), sensor.y + (max_distance - abs(x - sensor.x)) + 1):
                if 0 <= x <= 4000000 and 0 <= y <= 4000000:
                    covered_cells.add(Coordinates(x, y))

possible_places = set()
for x in range(4000000 + 1):
    for y in range(4000000 + 1):
        if Coordinates(x, y) not in covered_cells:
            possible_places.add(Coordinates(x, y))

print(len(possible_places))
pass