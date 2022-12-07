# Part 2
from collections import Counter
l = ''
buffer = 14
count = buffer
with open('inputs/6', 'r') as file:
    l = file.read(2*buffer - 1)
    h = Counter(l[:buffer - 1])
    while True:
        if len(l) < buffer:
            break
        for i in range(len(l[buffer - 1:])):
            h[l[i + buffer - 1]] += 1
            if h.most_common(1)[0][1] == 1:
                print(l[i:i + buffer], count)
                quit()
            h[l[i]] -= 1
            count += 1
        l = l[buffer:] + file.read(buffer)
    print('error')
