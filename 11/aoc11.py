from __future__ import annotations
"""Follow the TODOs in order to change the solution of the puzzle."""
from io import TextIOWrapper
import re
from math import lcm


class Monke:
    """Return to monke."""
    id = 0

    def __init__(
            self,
            items: list[Item],
            operation_op: str,
            operation_value: str,
            test_value: int,
            monkey_true_id: int,
            monkey_false_id: int) -> None:
        self.id = Monke.id
        Monke.id += 1
        self.items = items
        self.inspected_items = 0
        self.operation_op = operation_op
        self.operation_value = operation_value
        self.test_value = test_value
        self.monkey_true_id = monkey_true_id
        self.monkey_false_id = monkey_false_id

    def inspect_item(self, item: Item) -> None:
        """Let the monke inspect the item. This worries me a lot."""
        self.inspected_items += 1
        operation_value = (int(self.operation_value) if self.operation_value.isdigit()
                           else item.worry_level)
        match(self.operation_op):
            case '+':
                item.worry_level += operation_value
            case '*':
                item.worry_level *= operation_value
        # item.worry_level //= 3  # TODO: COMMENT FOR PART 2

    def new_thief(self, item: Item) -> int:
        """Decides the monke to whom to throw the item."""
        mod = item.worry_level % self.test_value
        return self.monkey_false_id if mod else self.monkey_true_id

    def throw(self, item: Item, monke: Monke) -> None:
        """Throws the item to the monke."""
        self.items.remove(item)
        monke.items.append(item)


class Item:
    """Simple wrapper for items."""

    def __init__(self, worry_level: int) -> None:
        self.worry_level = worry_level


monkeys: list[Monke] = []
test_values: set[int] = set()
chinese_theorem_value: int


def parse_monkeys(file: TextIOWrapper):
    line = file.readline()
    while line:
        # Starting items
        line = file.readline()
        starting_items = [Item(int(i)) for i in re.findall(r'\d+', line)]
        # Operation
        line = file.readline()
        operation_op, operation_val = re.search(
            r'Operation: new = old (["*""+"]) (old|\d+)', line).groups()    # type: ignore
        # Test
        line = file.readline()
        test_value = next(int(i) for i in re.findall(r'\d+', line))
        test_values.add(test_value)
        # Test: true
        line = file.readline()
        monkey_true_id = next(int(i) for i in re.findall(r'\d+', line))
        # Test: false
        line = file.readline()
        monkey_false_id = next(int(i) for i in re.findall(r'\d+', line))
        monkeys.append(
            Monke(
                starting_items,
                operation_op,
                operation_val,
                test_value,
                monkey_true_id,
                monkey_false_id)
        )
        file.readline()
        line = file.readline()


def exec_round():
    for monkey in monkeys:
        for item in list(monkey.items):
            monkey.inspect_item(item)
            item.worry_level %= chinese_theorem_value
            new_id = monkey.new_thief(item)
            monkey.throw(item, monkeys[new_id])


round = 0
with open('inputs/11', 'r') as file:
    parse_monkeys(file)
chinese_theorem_value = lcm(*test_values)
rounds = 10000     # TODO: change to 10000 FOR PART 2!
for i in range(rounds):
    round += 1
    exec_round()
monkeys.sort(key=(lambda m: m.inspected_items), reverse=True)
print(monkeys[0].inspected_items * monkeys[1].inspected_items)
