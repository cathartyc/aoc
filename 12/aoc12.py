from __future__ import annotations
from io import TextIOWrapper
from colorama import Fore, Style


class Node:
    """Represents every square of the map. It stores information for the
    minimum path search."""

    def __init__(self, x: int, y: int, height: int, parent: Node | None = None) -> None:
        self.x = x
        self.y = y
        self.height = height
        self.parent = parent
        if parent is not None:
            self.distance = parent.distance + 1
        else:
            self.distance = 0
        self.successors: list[Node] = []
        self.reached = False

    def expand(self, problem_map: Map) -> None:
        """Finds the next compatible nodes in the map."""
        if self.x > 0:
            if self.height >= problem_map.data[self.x - 1, self.y].height - 1:
                self.successors.append(problem_map.data[self.x - 1, self.y])
        if self.x < problem_map.v_len - 1:
            if self.height >= problem_map.data[self.x + 1, self.y].height - 1:
                self.successors.append(problem_map.data[self.x + 1, self.y])
        if self.y > 0:
            if self.height >= problem_map.data[self.x, self.y - 1].height - 1:
                self.successors.append(problem_map.data[self.x, self.y - 1])
        if self.y < problem_map.h_len - 1:
            if self.height >= problem_map.data[self.x, self.y + 1].height - 1:
                self.successors.append(problem_map.data[self.x, self.y + 1])


class Map:
    """Wrapper for the map. It contains a dictionary of nodes and the data
    to solve the problem.
    """

    def __init__(self, file: TextIOWrapper) -> None:
        self.start: Node
        self.goal: Node
        self.start_for_part_2: list[Node] = []
        self.data: dict[tuple[int, int], Node] = {}
        self.v_len = 0
        self.h_len = 0
        curr_row, curr_col = 0, 0
        for line in file:
            if not self.h_len:
                self.h_len = len(line.rstrip('\n'))
            for char in line.rstrip('\n'):
                node = Node(curr_row, curr_col, ord(char) - ord('a'), None)
                self.data[curr_row, curr_col] = node
                if char == 'S':
                    self.start = node
                    node.height = ord('a')
                    self.start_for_part_2.append(node)
                elif char == 'E':
                    self.goal = node
                    node.height = ord('z') - ord('a')
                elif char == 'a':
                    self.start_for_part_2.append(node)
                curr_col += 1
            curr_col = 0
            curr_row += 1
        self.v_len = curr_row
        for node in self.data.values():
            node.expand(self)

    def draw_path(self, path: list[Node] = None) -> str:
        """Draws the given path, or the reached nodes if no path is given."""
        temp_map = ''
        curr_row, curr_col = 0, 0
        for i in range(self.v_len):
            for j in range(self.h_len):
                node = self.data[i, j]
                if path is not None:
                    if node in path:
                        if node == path[0]:
                            temp_map += Fore.GREEN + 'S' + Style.RESET_ALL
                        elif node == path[-1]:
                            temp_map += Fore.GREEN + 'E' + Style.RESET_ALL
                        else:
                            temp_map += Fore.GREEN + \
                                chr(node.height + ord('a')) + Style.RESET_ALL
                    else:
                        temp_map += chr(node.height + ord('a'))
                elif node.reached:
                    temp_map += Fore.GREEN + \
                        chr(node.height + ord('a')) + Style.RESET_ALL
                else:
                    temp_map += chr(node.height + ord('a'))
                curr_col += 1
            curr_col = 0
            curr_row += 1
            temp_map += '\n'
        temp_map += '\n'
        print(temp_map)

    def reset(self):
        """Removes all the data stored during a path search."""
        for node in self.data.values():
            node.reached = False
            node.parent = None
            node.distance = 0

    def unitary_cost_search(self, start: Node, goal: Node) -> list[Node]:
        """Finds the minimum path between the start and the goal."""
        front: list[Node] = []
        front.append(start)
        start.reached = True
        end: Node = None

        while len(front):
            current_node = front.pop(0)
            for child in current_node.successors:
                if child == goal:
                    end = child
                    child.parent = current_node
                    child.distance = current_node.distance + 1
                    break
                if not child.reached:
                    child.reached = True
                    front.append(child)
                if (current_node.distance + 1 < child.distance or
                        child.parent is None and child != start):
                    child.parent = current_node
                    child.distance = current_node.distance + 1
            front.sort(key=(lambda n: n.distance))
            # If you want to visualize the actual search process uncomment the following:
            # problem_map.draw_path()
            if end is not None:
                break
        if end is None:
            return []
        curr = end
        path: list[Node] = []
        while curr:
            path.insert(0, curr)
            curr = curr.parent
        assert path[0] == start
        return path


problem_map: Map


with open('inputs/12', 'r') as file:
    problem_map = Map(file)

min_path = problem_map.unitary_cost_search(
    problem_map.start, problem_map.goal)
min_value = problem_map.goal.distance
print('Minimum path from S:', min_value)
problem_map.draw_path(min_path)

for a in problem_map.start_for_part_2:
    problem_map.reset()
    new_path = problem_map.unitary_cost_search(a, problem_map.goal)
    if new_path == []:
        continue
    # Uncomment the following to visualize every minimum path that has been found.
    # problem_map.draw_path(new_path)
    if new_path[-1].distance < min_value:
        min_path = new_path
        min_value = new_path[-1].distance
print('Shortest minimum path from an "a" square:', min_value)
problem_map.draw_path(min_path)
