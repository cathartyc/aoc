from __future__ import annotations
import re

MAX_SIZE = 4000000


class Coordinates:
    def __init__(self, x: int, y: int) -> None:
        self.x = x
        self.y = y

    def manhattan_distance(self, to: Coordinates):
        return abs(to.x - self.x) + abs(to.y - self.y)


class Sensor(Coordinates):
    def __init__(self, x: int, y: int) -> None:
        super().__init__(x, y)
        self.range: int

    def get_lines(self) -> set[Line]:
        """Get all lines that surround the detection field of the sensors.
        The distress beacon must necessarily be in the intersection
        of some of those lines.
        """
        lines = set()
        y_min = self.y - self.range - 1
        y_max = self.y + self.range + 1
        lines.add(Line(-1, y_min + self.x))
        lines.add(Line(-1, y_max + self.x))
        lines.add(Line(1, y_min - self.x))
        lines.add(Line(1, y_max - self.x))
        return lines


class Line:
    def __init__(self, m: int, q: int) -> None:
        self.m = m
        self.q = q

    def __eq__(self, value: object, /) -> bool:
        if not isinstance(value, Line):
            return False
        return value.m == self.m and value.q == self.q

    def __hash__(self) -> int:
        return self.m + self.q * 10

    def intersect(self, line: Line) -> Coordinates | None:
        if self.m == line.m:
            return None
        x = (line.q - self.q) // (self.m - line.m)
        y = self.m * x + self.q
        return Coordinates(x, y)


def is_detected(loc: Coordinates, sensors: set[Sensor]) -> bool:
    for other_sensor in sensors:
        if loc.manhattan_distance(other_sensor) <= other_sensor.range:
            return True
    return False


def parse() -> tuple[set[Sensor], dict[int, set[int]]]:
    sensors: set[Sensor] = set()
    beacons: dict[int, set[int]] = {}
    with open("inputs/input15.txt", "r") as file:
        for line in file:
            # Coordinates of sensors and beacons
            coords = re.findall(r"x=(-?\d+), y=(-?\d+)", line)
            sensor = Sensor(*map(lambda x: int(x), coords[0]))
            beacon = Coordinates(*map(lambda x: int(x), coords[1]))
            sensor.range = sensor.manhattan_distance(beacon)
            sensors.add(sensor)
            try:
                beacons[beacon.y].add(beacon.x)
            except KeyError:
                beacons[beacon.y] = {beacon.x}
        return (sensors, beacons)


def main():
    sensors, beacons = parse()

    lines: set[Line] = set()
    for sensor in sensors:
        lines.update(sensor.get_lines())
    intersections: set[Coordinates] = set()
    for line in lines:
        for other_line in lines:
            if line == other_line:
                continue
            intersection_point = line.intersect(other_line)
            if intersection_point is not None:
                intersections.add(intersection_point)
    for point in intersections:
        if (
            not is_detected(point, sensors)
            and point.x >= 0
            and point.x <= MAX_SIZE
            and point.y >= 0
            and point.y <= MAX_SIZE
        ):
            print(f"[Part_2] The result is {4000000 * point.x + point.y}")
            exit()
    raise AssertionError("No solution for the given input")


if __name__ == "__main__":
    main()
