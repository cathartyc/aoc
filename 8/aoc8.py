# Part 1
def is_visible(height: int, x: int, y: int) -> bool:
    """Checks if there is no tree in any direction (except diagonally)
    that is taller than the considered tree.
    """
    return (x == 0 or y == 0 or x == garden_size - 1 or y == garden_size - 1 or
            max(garden[i, y] for i in range(x)) < height or
            max(garden[i, y] for i in range(x + 1, garden_size)) < height or
            max(garden[x, i] for i in range(y)) < height or
            max(garden[x, i] for i in range(y + 1, garden_size)) < height)

# Part 2
def scenic_score(height: int, x: int, y: int) -> int:
    """Evaluates the distance between the tree and the nearest tree with
    same or greater height, in any direction, and returns their product.
    """
    if x == 0 or y == 0 or x == garden_size - 1 or y == garden_size - 1:
        return 0
    left_score = x - next((k for k in reversed(range(x))
                          if garden[k, y] >= height), 0)
    right_score = next((k for k in range(x + 1, garden_size)
                       if garden[k, y] >= height), garden_size - 1) - x
    up_score = y - next((k for k in reversed(range(y))
                        if garden[x, k] >= height), 0)
    down_score = next((k for k in range(y + 1, garden_size)
                      if garden[x, k] >= height), garden_size - 1) - y
    return left_score * right_score * up_score * down_score


visible_trees = 0
max_scenic_score = 0
garden: dict[(int, int), int] = {}
garden_size = 0
curr_x, curr_y = 0, 0

with open('inputs/8', 'r') as file:
    for line in file:
        if not line:
            break
        if garden_size == 0:
            garden_size = len(line) - 1
        for d in line.rstrip('\n'):
            garden[(curr_x, curr_y)] = int(d)
            curr_x += 1
        curr_x = 0
        curr_y += 1

for coordinates, height in garden.items():
    if is_visible(height, *coordinates):
        visible_trees += 1
    sce_score = scenic_score(height, *coordinates)
    if max_scenic_score < sce_score:
        max_scenic_score = sce_score

print(visible_trees, max_scenic_score)
