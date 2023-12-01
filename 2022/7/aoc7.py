from __future__ import annotations

from typing import Generator


class Directory:
    """Represents a directory with the total size.
    It keeps track of the parent directory and collects subdirectories
    in a set.
    """
    def __init__(self, name: str, parent: Directory | None) -> None:
        self.name = name
        self.parent = parent
        self.directories: set[Directory] = set()
        self.size = 0

    def subdirectory(self, name: str) -> Directory | None:
        """Returns the subdirectory with the given name, if present."""
        return next(d for d in self.directories if d.name == name)

    def add_directory(self, name: str) -> None:
        """Adds a subdirectory."""
        new_dir = Directory(name, self)
        self.directories.add(new_dir)

    def add_file(self, size: int) -> None:
        """Increments the size of the directory and the size of its
        parents.
        """
        self.size += size
        if self.parent is not None:
            self.parent.add_file(size)


def get_all_directories(parent: Directory) -> Generator[Directory, None, None]:
    """Walks over the directory tree, starting from the given directory."""
    for dir in parent.directories:
        yield from get_all_directories(dir)
    yield parent


def parse_directory(arg: str) -> None:
    """Changes the current directory."""
    global curr
    if arg == '..':
        curr = curr.parent
    elif arg == root.name:
        curr = root
    else:
        curr = curr.subdirectory(arg)
        if curr is None:
            print(f'error: dir {arg} not found')


def parse_content(arg1: str, arg2: str) -> None:
    """Adds files (their weight) or directories to the current directory."""
    global curr
    if arg1.isdigit():
        curr.add_file(int(arg1))
    elif arg1 == 'dir':
        curr.add_directory(arg2)


root = Directory('/', None)
curr = root


with open('inputs/7', 'r') as file:
    for line in file:
        l = line.rstrip('\n')
        if not l:
            break
        if l.startswith('$ cd'):
            parse_directory(l.split(' ', 2)[2])
        else:
            parse_content(*l.split(' '))

# Part 1
print(sum(d.size for d in get_all_directories(root) if d.size <= 100000))

# Part 2
print(min(d.size for d in get_all_directories(root)
      if 70000000 - root.size + d.size >= 30000000))
