# Part 2
sprite = 1


class Clock:
    """Wrapper for the clock. It has the only purpose of doing stuff after
    each increment.
    """

    def __init__(self) -> None:
        self.cycle = 0

    def increment_cycle(self, increment: int) -> None:
        """Increments the clock cycle and prints a pixel. Prints a part
        of the sprite if the sprite is located on the current pixel (i.e.
        the clock cycle).
        """
        self.cycle += increment
        print('#' if sprite - 1 <= (self.cycle - 1) % 40 <= sprite + 1
              else '.', end='')
        if not self.cycle % 40:
            print()


clk = Clock()


def noop() -> None:
    """Does nothing, except wasting a clock cycle."""
    clk.increment_cycle(1)


def add_x(value: int) -> None:
    """Uses two clock cycles to add a value to x."""
    global sprite
    clk.increment_cycle(1)
    clk.increment_cycle(1)
    sprite += value


def parse_instruction(line: str) -> None:
    """Reads the instruction."""
    instr = line.rstrip('\n').split(' ', 2)
    match (instr[0]):
        case 'addx':
            add_x(int(instr[1]))
        case 'noop':
            noop()


with open('inputs/10', 'r') as file:
    for line in file:
        if line:
            parse_instruction(line)
