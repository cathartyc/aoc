from __future__ import annotations
from enum import Enum


class Coordinates:
    """Wrapper for the coordinates of a knot."""

    def __init__(self, x: int = 0, y: int = 0) -> None:
        self.x = x
        self.y = y

    def increment_x(self) -> None:
        """Increments the x coordinate."""
        self.x += 1

    def increment_y(self) -> None:
        """Increments the y coordinate."""
        self.y += 1

    def decrement_x(self) -> None:
        """Decrements the x coordinate."""
        self.x -= 1

    def decrement_y(self) -> None:
        """Decrements the y coordinate."""
        self.y -= 1

    def __eq__(self, __o: Coordinates) -> bool:
        if not isinstance(__o, Coordinates):
            return False
        return self.x == __o.x and self.y == __o.y


class Knot:
    """Represents a knot in the rope."""

    def __init__(self,
                 x: int = 0,
                 y: int = 0,
                 prev: Knot = None,
                 next: Knot = None) -> None:
        self.coordinates = Coordinates(x, y)
        self.prev = prev
        if prev is not None:
            prev.next = self
        self.next = next
        if next is not None:
            next.prev = self

    def move_up(self) -> None:
        """Moves the knot up."""
        self.coordinates.increment_y()

    def move_down(self) -> None:
        """Moves the knot down."""
        self.coordinates.decrement_y()

    def move_right(self) -> None:
        """Moves the knot to the right."""
        self.coordinates.increment_x()

    def move_left(self) -> None:
        """Moves the knot to the left."""
        self.coordinates.decrement_x()

    def move_diagonal_up_right(self) -> None:
        """Moves the knot to the up-right diagonal."""
        self.move_up()
        self.move_right()

    def move_diagonal_up_left(self) -> None:
        """Moves the knot to the up-left diagonal."""
        self.move_up()
        self.move_left()

    def move_diagonal_down_right(self) -> None:
        """Moves the knot to the down-right diagonal."""
        self.move_down()
        self.move_right()

    def move_diagonal_down_left(self) -> None:
        """Moves the knot to the down-left diagonal."""
        self.move_down()
        self.move_left()

    def is_upper_than(self, knot: Knot) -> bool:
        """Checks if the knot is upper than the given knot."""
        return self.coordinates.y > 1 + knot.coordinates.y

    def is_lower_than(self, knot: Knot) -> bool:
        """Checks if the knot is lower than the given knot."""
        return self.coordinates.y < knot.coordinates.y - 1

    def is_more_to_the_right_than(self, knot: Knot) -> bool:
        """Checks if the knot is more to the right than the given knot."""
        return self.coordinates.x > 1 + knot.coordinates.x

    def is_more_to_the_left_than(self, knot: Knot) -> bool:
        """Checks if the knot is more to the left than the given knot."""
        return self.coordinates.x < knot.coordinates.x - 1

    def is_upper_and_more_to_the_right_than(self, knot: Knot) -> bool:
        """Checks if the knot is upper and more to the right than the
        given knot.
        """
        return self.is_upper_than(knot) and self.is_more_to_the_right_than(knot)

    def is_upper_and_more_to_the_left_than(self, knot: Knot) -> bool:
        """Checks if the knot is upper and more to the left than the
        given knot.
        """
        return self.is_upper_than(knot) and self.is_more_to_the_left_than(knot)

    def is_lower_and_more_to_the_right_than(self, knot: Knot) -> bool:
        """Checks if the knot is lower and more to the right than the
        given knot.
        """
        return self.is_lower_than(knot) and self.is_more_to_the_right_than(knot)

    def is_lower_and_more_to_the_left_than(self, knot: Knot) -> bool:
        """Checks if the knot is lower and more to the left than the
        given knot.
        """
        return self.is_lower_than(knot) and self.is_more_to_the_left_than(knot)

    def is_near_enough_to(self, knot: Knot) -> bool:
        """Checks if the knot is near enough (Manhattan distance <= 1) to
        the given knot.
        """
        return (abs(self.coordinates.x - knot.coordinates.x) <= 1 and
                abs(self.coordinates.y - knot.coordinates.y) <= 1)

    def is_not_in_the_same_row_or_column_as(self, knot: Knot) -> bool:
        """Checks if the knot is not in the same row or column as the 
        given knot.
        """
        return (self.coordinates.x != knot.coordinates.x and
                self.coordinates.y != knot.coordinates.y)


class Direction(Enum):
    """Wrapper for the possible movements of the head."""
    RIGHT = 'R'
    LEFT = 'L'
    UP = 'U'
    DOWN = 'D'


class TailHistory:
    """Wrapper for a set of coordinates."""

    def __init__(self) -> None:
        self.set_of_coordinates: set[tuple[int, int]] = set()

    def add_position(self, coordinates: Coordinates) -> None:
        """Adds a pair of coordinates to the set."""
        self.set_of_coordinates.add((coordinates.x, coordinates.y))

    def length(self) -> int:
        """Returns the length of the set."""
        return len(self.set_of_coordinates)

    def reset(self) -> None:
        """Removes the coordinates currently into the set."""
        self.set_of_coordinates.clear()
