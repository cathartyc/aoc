import re
from collections import namedtuple

y_of_analysis = 2000000

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
        if not -max_distance <= y_of_analysis - sensor.y <= max_distance:
            continue
        for i in range(sensor.x - (max_distance - abs(y_of_analysis - sensor.y)), sensor.x + (max_distance - abs(y_of_analysis - sensor.y)) + 1):
            covered_cells.add(Coordinates(i, y_of_analysis))
        if beacon in covered_cells:
            covered_cells.remove(beacon)
        sensors_and_beacons.add(sensor)
        sensors_and_beacons.add(beacon)

print(len(covered_cells))
