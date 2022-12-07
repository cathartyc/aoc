with open('inputs/1', 'r') as file:
    values = file.read().split(sep='\n\n')
elves = [
    sum(
        [int(i) for i in j.strip('\n').split(sep='\n',)])
    for j in values
]
elves.sort(reverse=True)
# Part 1
print(elves[0])
# Part 2
print(sum(elves[0:3]))
