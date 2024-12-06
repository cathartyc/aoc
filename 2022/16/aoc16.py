import re


class Valve:
    def __init__(self, name: str, flow_rate: int, adjacent_valves: list[str]):
        self.name: str = name
        self.flow_rate: int = flow_rate
        self.adjacent_valves: list[str] = adjacent_valves


def shortest_paths(
    valves: list[Valve],
) -> dict[tuple[Valve, Valve], int]:
    """Compute the shortest path between every pair of valves."""
    INFTY = 1000
    dist = {(valve, valve): 0 for valve in valves}
    # Apply to every node
    for valve in [start] + valves_that_matter:
        # Dijkstra's algorithm
        dist.update({(valve, v): INFTY for v in valves if v != valve})
        unvisited = [v for v in valves]
        while len(unvisited) > 0:
            unvisited.sort(key=lambda v: dist[(valve, v)])
            curr = unvisited.pop(0)
            for adj in curr.adjacent_valves:
                adjacent = valves_dict[adj]
                if adjacent not in unvisited:
                    continue
                if dist[(valve, adjacent)] > dist[(valve, curr)] + 1:
                    dist[(valve, adjacent)] = dist[(valve, curr)] + 1
    return dist


valves: list[Valve] = []
valves_that_matter: list[Valve] = []
start: Valve

with open("inputs/16", "r") as file:
    id_catcher = re.compile(r"[A-Z]{2}")
    for line in file:
        valve_names: list[str] = re.findall(id_catcher, line)
        flow_rate = re.findall(r"\d+", line)
        assert isinstance(flow_rate[0], str)
        flow_rate = int(flow_rate[0])
        new_valve = Valve(valve_names.pop(0), flow_rate, valve_names)
        # Frigging lost days because I did not understand that the start
        # valve was not the first one in the list but _always_ the one named "AA"
        if new_valve.name == "AA":
            start = new_valve
        valves.append(new_valve)
        if flow_rate > 0:
            valves_that_matter.append(new_valve)

assert start is not None

valves_dict = {valve.name: valve for valve in valves}

dist = shortest_paths(valves)

# Part 1

opened_valves: list[str] = []
total_time = 30


def max_score(curr_valve: Valve, opened: list[str], remaining_time: int, score: int) -> tuple[int, list[str]]:
    """Find recursively the best score for a given starting valve."""
    best_score = 0
    best_opened = opened
    for valve in valves_that_matter:
        if valve.name in opened:
            continue
        new_time = remaining_time - 1 - dist[(curr_valve, valve)]
        if new_time <= 0:
            continue
        new_score, new_opened = max_score(
            valve, opened + [valve.name], new_time, score + valve.flow_rate * new_time)
        if new_score > best_score:
            best_score = new_score
            best_opened = new_opened
    if best_score == 0:
        return score, opened
    return best_score, best_opened


score, path = max_score(start, opened_valves, total_time, 0)

print(f"Part 1: {score} of pressure released by opening those valves: {path}.")

# Part 2

opened_valves = []
total_time = 26

my_score, my_path = max_score(start, opened_valves, total_time, 0)
elephant_score, elephant_path = max_score(start, my_path, total_time, 0)

print(f"\
Part 2: {my_score + elephant_score} of pressure released by opening those valves: {my_path} and {elephant_path}.")
