from __future__ import annotations
import re
from collections import namedtuple

target_row: int = 2000000

Coordinates = namedtuple("Coordinates", ["x", "y"])


def manhattan_distance(start: Coordinates, end: Coordinates):
    return abs(end.x - start.x) + abs(end.y - start.y)


class EmptyRange:
    def __init__(self, begin: int, end: int) -> None:
        self.begin = begin
        self.end = end

    def overlaps_with(self, range: EmptyRange) -> bool:
        """It is assumed that self is at the left of range."""
        return self.begin <= range.begin <= self.end

    def contains(self, range: EmptyRange) -> bool:
        """Check if range_1 contains range_2"""
        return self.begin <= range.begin and range.end <= self.end


sensors_and_beacons: set[Coordinates] = set()
empty_ranges: list[EmptyRange] = []
total = 0
with open("inputs/input15.txt", "r") as file:
    for line in file:
        # Coordinates of sensors and beacons
        coords = re.findall(r"x=(-?\d+), y=(-?\d+)", line)
        sensor = Coordinates(*map(lambda x: int(x), coords[0]))
        beacon = Coordinates(*map(lambda x: int(x), coords[1]))
        sensors_and_beacons.add(sensor)
        sensors_and_beacons.add(beacon)
        # skip sensor if the target row is not reached by the sensor
        # detection range
        max_distance = manhattan_distance(sensor, beacon)
        if not abs(target_row - sensor.y) <= max_distance:
            continue
        # compute the coverage of the target row by the sensor detection
        new_range = EmptyRange(
            begin=sensor.x - (max_distance - abs(target_row - sensor.y)),
            end=sensor.x + (max_distance - abs(target_row - sensor.y)),
        )
        print(f"{new_range.begin} - {new_range.end}")
        rev_ranges = empty_ranges.copy()
        rev_ranges.reverse()
        nearest_left_range = next(
            filter(lambda range: range.begin < new_range.begin, rev_ranges), None
        )
        if nearest_left_range is not None:
            if nearest_left_range.overlaps_with(new_range):
                if nearest_left_range.contains(new_range):
                    new_range.end = nearest_left_range.end
                new_range.begin = nearest_left_range.begin 
                empty_ranges.remove(nearest_left_range)
        nearest_right_range = next(
            filter(lambda range: range.begin >= new_range.begin and range != new_range, empty_ranges), None
        )
        if nearest_right_range is not None:
            if new_range.overlaps_with(nearest_right_range):
                merged = True
                if not new_range.contains(nearest_right_range):
                    new_range.end = nearest_right_range.end
                empty_ranges.remove(nearest_right_range)
        empty_ranges.append(new_range)
        empty_ranges.sort(key=lambda range: range.begin)

print("Final ranges")
for range in empty_ranges:
    print(f"{range.begin} - {range.end}")
    total += range.end - range.begin + 1
    for stuff in sensors_and_beacons:
        if stuff.y == target_row:
            if range.begin <= stuff.x <= range.end:
                total -= 1

print(total)