# Part 1
x = 1
sum = 0


class Clock:
    """Wrapper for the clock. It has the only purpose of doing stuff after
    each increment.
    """

    def __init__(self) -> None:
        self.cycle = 0

    def increment_cycle(self, increment: int) -> None:
        """Increments the clock cycle.
        If the cycle is one of the listed (hardcoded kek) values, it
        prints out the signal strength, that is, the product between the
        clock cycle and x.
        """
        global sum
        self.cycle += increment
        if not (self.cycle + 20) % 40:
            print("Signal strength at", self.cycle,
                  "clock cycles:", self.cycle * x)
            sum += self.cycle * x


clk = Clock()


def noop() -> None:
    """Does nothing, except wasting a clock cycle."""
    clk.increment_cycle(1)


def add_x(value: int) -> None:
    """Uses two clock cycles to add a value to x."""
    global x
    clk.increment_cycle(1)
    clk.increment_cycle(1)
    x += value


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

print(sum)
