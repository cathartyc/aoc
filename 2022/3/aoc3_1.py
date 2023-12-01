# Part 1
def priority(item: str) -> int:
    """Returns the priority of the item."""
    return ord(item) + (27 - ord('A') if item.isupper() else 1 - ord('a'))


total_priority = 0
with open('inputs/3', 'r') as file:
    for baggage in file:
        mid = (len(baggage) - 1) // 2
        total_priority += priority(
            next(item for item in baggage[:mid] if item in baggage[mid:]))

print(total_priority)
