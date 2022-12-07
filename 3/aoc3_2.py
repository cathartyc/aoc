# Part 2
from typing import Generator
from io import TextIOWrapper


def priority(item: str) -> int:
    """Returns the priority of the item."""
    return ord(item) + (27 - ord('A') if item.isupper() else 1 - ord('a'))


def read_n_lines(file: TextIOWrapper, n: int) -> Generator[str, None, None]:
    """Reads the specified amount of lines each time from a file descriptor."""
    l = ''
    while True:
        lines = ''
        for _ in range(n):
            l = file.readline()
            if not l:
                break
            lines += l
        if not l:
            break
        yield lines
    if lines:
        yield lines


groups_total_priority = 0
group_size = 3
with open('inputs/3', 'r') as file:
    for group in read_n_lines(file, group_size):
        baggages = group.split(sep='\n', maxsplit=group_size - 1)
        item = next(item for item in baggages[0] if not any(
            item not in baggage for baggage in baggages[1:]))
        groups_total_priority += priority(item)

print(groups_total_priority)
