from __future__ import annotations
import re
from collections import namedtuple
import bisect

Coordinates = namedtuple("Coordinates", ["x", "y"])

MAX_SIZE = 4000000


class RowStat:
    def __init__(self) -> None:
        self.ranges: list[tuple[int, int]] = []
        self.full = False

    def add(self, range: tuple[int, int]):
        if self.full:
            return
        # insert
        index = bisect.bisect_right(self.ranges, range[0], key=lambda x: x[0])
        if index != 0:
            prev_range = self.ranges[index - 1]
            # the new range is totally contained into an already present one
            # -> discard
            if prev_range[1] >= range[1]:
                return
            # the new range is overlapping (or totally adjacent) with the previous one
            # -> merge
            if prev_range[1] >= range[0] - 1:
                range = (prev_range[0], range[1])
                del self.ranges[index - 1]
                index -= 1
        self.ranges.insert(index, range)
        while index + 1 < len(self.ranges):
            next_range = self.ranges[index + 1]
            # new range contains entirely the next one
            if range[1] >= next_range[1]:
                # print(f"{range} contains {next_range}, removing it")
                del self.ranges[index + 1]
                continue
            # new range overlaps with the next one
            if range[1] >= next_range[0] - 1:
                # print(f"{range} overlaps with {next_range}")
                range = (range[0], next_range[1])
                # tuples are not dynamic objects
                self.ranges[index] = range
                # print(f"New range: {range}")
                del self.ranges[index + 1]
            else:
                break
        if len(self.ranges) == 1 and range[0] == 0 and range[1] == MAX_SIZE:
            self.full = True


def manhattan_distance(start: Coordinates, end: Coordinates):
    return abs(end.x - start.x) + abs(end.y - start.y)


def get_frontier(sensor: Coordinates, distance: int):
    range_min = max(sensor.y - distance, 0)
    range_max = min(sensor.y + distance, MAX_SIZE)
    # print(f"{sensor}: {range_min} -> {range_max}")
    for y in range(range_min, range_max + 1):
        offset = distance - abs(y - sensor.y)
        x_1 = max(sensor.x - offset, 0)
        x_2 = min(sensor.x + offset, MAX_SIZE)
        ranges[y].add((x_1, x_2))


def is_detected(loc: Coordinates) -> bool:
    for other_sensor in sensors:
        if manhattan_distance(loc, other_sensor) <= distances[other_sensor]:
            return True
    return False


sensors: list[Coordinates] = []
distances: dict[Coordinates, int] = {}
points: dict[int, set[int]] = {}
ranges: list[RowStat] = []


def main():
    for i in range(MAX_SIZE + 1):
        ranges.append(RowStat())
    print(len(ranges))
    beacons: dict[int, set[int]] = {}
    parse(sensors, beacons, distances)
    # skip sensor if the target row is not reached by the sensor
    # detection range
    # solution: x=3435885, row=2639657
    for sensor, distance in distances.items():
        get_frontier(sensor, distance)
        # points.difference_update(exclusion_points)
        # print(len(points))
    #print(f"{[r.ranges for r in ranges]}")
    point_y = next(y for y in ranges if not y.full)
    print(f"{(point_y.ranges[0][1] + 1) * 4000000 + ranges.index(point_y)}")
    exit()


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
