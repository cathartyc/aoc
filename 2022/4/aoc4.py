# Part 1
def is_contained(a0: int, a1: int, b0: int, b1: int) -> bool:
    """Compares the values to check if one of the intervals is fully
    contained inside the other.

    The arguments are supposed to be ordered as (a0, a1, b0, b1),
    with a- being the borders of the first elf and b- the borders of the
    other.
    """
    return a0 <= b0 and b1 <= a1 or b0 <= a0 and a1 <= b1

# Part 2
def is_overlapping(a0: int, a1: int, b0: int, b1: int) -> bool:
    """Compares the values to check if one of the intervals is overlapping
    on the other.

    The arguments are supposed to be ordered as (a0, a1, b0, b1),
    with a- being the borders of the first elf and b- the borders of the
    other.
    """
    return (a0 <= b0 <= a1 or a0 <= b1 <= a1)


def parse_line(line: str) -> tuple[int, int, int, int]:
    """Extracts the values from the given line."""
    pair = line.removesuffix('\n').split(sep=',')
    values = tuple(int(section)
                   for elf in pair for section in elf.split(sep='-'))
    return values


full_conflicting_couples = 0
overlapped_couples = 0
with open('inputs/4', 'r') as file:
    for line in file:
        values = parse_line(line)
        if is_contained(*values):
            full_conflicting_couples += 1
            overlapped_couples += 1
        elif is_overlapping(*values):
            overlapped_couples += 1


print(full_conflicting_couples, overlapped_couples)
