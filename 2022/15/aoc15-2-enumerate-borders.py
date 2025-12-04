from __future__ import annotations
import re
from collections import namedtuple

Coordinates = namedtuple("Coordinates", ["x", "y"])


def manhattan_distance(start: Coordinates, end: Coordinates):
    return abs(end.x - start.x) + abs(end.y - start.y)


def get_frontier(sensor: Coordinates, distance: int) -> set[Coordinates]:
    points = set()
    range_min = max(sensor.y - distance - 1, 0)
    range_max = min(sensor.y + distance + 1, MAX_SIZE)
    print(f"{sensor}: {range_min} -> {range_max}")
    for y in range(range_min, range_max + 1):
        offset = distance - abs(y - sensor.y) + 1
        # left
        loc = Coordinates(sensor.x - offset, y)
        if loc.x >= 0 and loc.x <= MAX_SIZE:
            points.add(loc)
        # right
        loc = Coordinates(sensor.x + offset, y)
        if loc.x >= 0 and loc.x <= MAX_SIZE:
            points.add(loc)
    return points


def is_detected(loc: Coordinates) -> bool:
    for other_sensor in sensors:
        if manhattan_distance(loc, other_sensor) <= distances[other_sensor]:
            return True
    return False


sensors: list[Coordinates] = []
distances: dict[Coordinates, int] = {}
points: dict[int, set[int]] = {}

MAX_SIZE = 4000000


def main():
    beacons: dict[int, set[int]] = {}
    parse(sensors, beacons, distances)
    # skip sensor if the target row is not reached by the sensor
    # detection range
    # x=3435885, row=2639657
    # exclusion_points = set()
    for sensor, distance in distances.items():
        points = get_frontier(sensor, distance)
        # points.difference_update(exclusion_points)
        # print(len(points))
        for point in points:
            for other_sensor, other_distance in distances.items():
                if sensor == other_sensor:
                    continue
                if is_detected(point):
                    break
            else:
                print(f"{point}")
                print(f"{4000000 * point.x + point.y}")
                exit()
        # exclusion_points.update(points)


def parse(sensors, beacons, distances):
    with open("inputs/input15.txt", "r") as file:
        for line in file:
            # Coordinates of sensors and beacons
            coords = re.findall(r"x=(-?\d+), y=(-?\d+)", line)
            sensor = Coordinates(*map(lambda x: int(x), coords[0]))
            beacon = Coordinates(*map(lambda x: int(x), coords[1]))
            sensors.append(sensor)
            try:
                beacons[beacon.y].add(beacon.x)
            except KeyError:
                beacons[beacon.y] = {beacon.x}
            distances[sensor] = manhattan_distance(sensor, beacon)


if __name__ == "__main__":
    main()
