from json import loads
from functools import cmp_to_key


def in_right_order(l1: list, l2: list) -> int:
    """Checks if the two given lists are sorted:
    - every item of the first list should be lower of the item of the same
      index in the other list;
    - if the two items are lists, they should respect the same rule;
    - if one item is an integer and the other is a list, the check should
      be done wrapping the integer into a list and they should respect the
      first rule.

    Returns 1 if the first list is lower than the second, -1 if the opposite
    is true and 0 if they are equal.
    """
    ordered = 0
    for val1, val2 in zip(l1, l2):
        match (type(val1).__name__, type(val2).__name__):
            case ('int', 'int'):
                val_diff = val2 - val1
                # I found this idea cool, sorry.
                ordered = val_diff and (1, -1)[val_diff < 0]
            case('list', 'list'):
                ordered = in_right_order(val1, val2)
            case('list', 'int'):
                ordered = in_right_order(val1, [val2])
            case('int', 'list'):
                ordered = in_right_order([val1], val2)
        if ordered:
            break
    if ordered:
        return ordered
    diff = len(l2) - len(l1)
    return diff and (1, -1)[diff < 0]  # And I used it twice, sorry again.


count = 1
index_sum = 0

packets_list = []

with open('inputs/13', 'r') as file:
    while True:
        l1, l2 = file.readline(), file.readline()
        if not (l1 and l2):
            break
        list1, list2 = loads(l1), loads(l2)
        packets_list.extend((list1, list2))
        # Part 1
        if in_right_order(list1, list2) == 1:
            index_sum += count
        file.readline()
        count += 1
print(f'The sum of the indexes of right-ordered pairs is {index_sum}.')

# Part 2
packets_list.extend(([[2]], [[6]]))
packets_list.sort(key=cmp_to_key(in_right_order), reverse=True)
index_of_2 = packets_list.index([[2]])
index_of_6 = packets_list.index([[6]])
print(
    f'The product of the indexes of {[[2]]} and {[[6]]} is '
    f'{(index_of_2+1) * (index_of_6+1)}.'
)
