# Part 2
from re import findall

l = ''
crates: list[list[str]] = []
columns = 0


def add_to_crates(line: str) -> None:
    """Inserts the elements inside the proper stack."""
    for i in range(columns):
        if line[4*i + 1].isalpha():
            crates[i].insert(0, line[4*i + 1])


def exec_instruction(line: str) -> None:
    """Moves a specific amount of crates from a stack to another."""
    amount, source, destination = (int(i) for i in findall(r'\d+', line))
    crates[destination-1].extend(crates[source-1][-amount:])
    del crates[source-1][-amount:]


with open('inputs/5', 'r') as file:
    while True:
        l = file.readline()
        if l[1] == '1':
            next(file)
            break
        if not columns:
            columns = len(l) // 4
            crates.extend([] for _ in range(columns))
        add_to_crates(l)
    for line in file:
        if line:
            exec_instruction(line)

for i in crates:
    print(i[-1], end='')
