import re
from collections import namedtuple

grid_size = 4000000

Coordinates = namedtuple('Coordinates', ['x', 'y'])
y_ranges: dict[int, set[tuple[int, int]]] = {}

def manhattan_distance(start: Coordinates, end: Coordinates):
    return abs(end.x - start.x) + abs(end.y - start.y)


with open('inputs/15', 'r') as file:
    for line in file:
        coords = re.findall('x=(-?\d+), y=(-?\d+)', line)
        sensor = Coordinates(*map(lambda x: int(x), coords[0]))
        beacon = Coordinates(*map(lambda x: int(x), coords[1]))
        max_distance = manhattan_distance(sensor, beacon)
        for y in range(max(0, sensor.y - max_distance), min(grid_size, sensor.y + max_distance) + 1):
            x_start = max(sensor.x - (max_distance - abs(y - sensor.y)), 0)
            x_end = min(sensor.x + (max_distance - abs(y - sensor.y)), grid_size)
            if y not in y_ranges.keys():
                y_ranges[y] = set()
            elif (0, grid_size) in y_ranges[y]:
                continue
            for interval in set(y_ranges[y]):
                if interval[0] <= x_start and  x_end <= interval[1]:
                    break
                if (x_start < interval[0] and interval[1] <= x_end or 
                    x_start <= interval[0] and interval[1] < x_end):
                    y_ranges[y].remove(interval)
                elif abs(x_start - interval[1]) <= 1 or x_start < interval[1] < x_end:
                    y_ranges[y].remove(interval)
                    x_start = interval[0]
                elif abs(x_end - interval[0]) <= 1 or x_start < interval[0] < x_end:
                    y_ranges[y].remove(interval)
                    x_end = interval[1]
            else:
                y_ranges[y].add((x_start, x_end))
                    
                

possible_places = set()
for y in range(grid_size + 1):
    if (0, grid_size) in y_ranges[y]:
        continue
    elif len(y_ranges[y]) == 2:
        x = y_ranges[y].pop()[0] - 1
        print(grid_size*x + y)
        quit()
    for x in range(grid_size + 1):
        for interval in y_ranges[y]:
            if interval[0] <= x <= interval[1]:
                break
        else:
            possible_places.add(Coordinates(x, y))
            print(grid_size*x + y)
            quit()
