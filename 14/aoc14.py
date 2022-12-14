import re
from colorama import Fore, Style
from collections import namedtuple
import time

Solid = namedtuple('Solid', ['x', 'y'])

rocks: set[Solid] = set()
biggest_y: int = 0
smallest_x = 1000
biggest_x = 0


def draw(last_sand: Solid, y_interval: int = None):
    """Draws the cave, identifying the rocks, the sand, the last fallen
    sand block and the air.
    The last parameter allows to print only a certain range on the y,
    around the given sand block.
    """
    drawing = ''
    if y_interval is None:
        ys = range(biggest_y + 1)
    else:
        ys = range(last_sand.y - y_interval // 2,
                   last_sand.y + y_interval // 2 + 1)
    for y in ys:
        for x in range(smallest_x, biggest_x + 1):
            if (x, y) in rocks or y >= biggest_y:
                drawing += '#'
            elif (x, y) == last_sand:
                drawing += Fore.GREEN + 'o' + Style.RESET_ALL
            elif (x, y) in sand_depot:
                drawing += 'o'
            else:
                drawing += '.'
        drawing += '\n'
    print(drawing)


def parse_coords(coords: list[tuple[str]]) -> set[Solid]:
    """Reads the list of rocks and saves them - one at a time - in a set
    that then is returned.
    """
    global smallest_x, biggest_x, biggest_y
    new_rockset: set[Solid] = set()
    start = None
    for coord in coords:
        c = (int(coord[0]), int(coord[1]))
        if c[0] > biggest_x:
            biggest_x = c[0]
        elif c[0] < smallest_x:
            smallest_x = c[0]
        if c[1] > biggest_y:
            biggest_y = c[1]
        if start is not None:
            if start[0] != c[0]:
                x0, x1 = start[0], c[0]
                if x0 > x1:
                    x0, x1 = x1, x0
                for x in range(x0, x1+1):
                    new_rockset.add(Solid(x, start[1]))
            elif start[1] != c[1]:
                y0, y1 = start[1], c[1]
                if y0 > y1:
                    y0, y1 = y1, y0
                for y in range(y0, y1+1):
                    new_rockset.add(Solid(start[0], y))
        start = c
    return new_rockset


def sand_next_step(sand: Solid) -> Solid:
    """Finds the next place for the given falling block of sand."""
    global curr_start
    if sand.y == biggest_y-1:
        return sand
    down_square = Solid(sand.x, sand.y+1)
    left_square = Solid(sand.x-1, sand.y+1)
    right_square = Solid(sand.x+1, sand.y+1)
    if not (down_square in sand_depot or down_square in rocks):
        return down_square
    if not (left_square in sand_depot or left_square in rocks):
        return left_square
    if not (right_square in sand_depot or right_square in rocks):
        return right_square
    return sand

start = time.time()
with open('inputs/142', 'r') as file:
    while True:
        l = file.readline()
        if not l:
            break
        lines = set()
        rocks.update(parse_coords(re.findall(r'(\d+),(\d+)', l)))
end_parsing = time.time()
biggest_y += 2

sand_depot: set[Solid] = set()
sand_start = Solid(500, 0)
curr_start = sand_start

curr_sand: Solid = None
bottom_reached = False
while curr_sand != sand_start:
    curr_sand = curr_start
    while True:
        sand_new_position = sand_next_step(curr_sand)
        if sand_new_position.x > biggest_x:
            biggest_x = sand_new_position.x
        elif sand_new_position.x < smallest_x:
            smallest_x = sand_new_position.x
        if curr_sand == sand_new_position:
            break
        curr_sand = sand_new_position
    if not bottom_reached and curr_sand.y == biggest_y - 1:
        print(f'Reached bottom after {len(sand_depot)} sand blocks.')
        bottom_reached = True
    sand_depot.add(sand_new_position)


print(f'Reached starting point after {len(sand_depot)} sand blocks.')
draw(sand_new_position)
