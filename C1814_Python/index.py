import sys
from collections import deque

t, *data = map(str.split, sys.stdin.read().splitlines())

for i in range(int(t[0])):
    _, s1, s2 = map(int, data[i*2])
    r = list(map(int, data[i*2+1]))
    r_sorted = sorted(r, reverse=True)
    l1, l2 = deque(), deque()
    t1, t2 = s1, s2
    m = {}

    for i, el in enumerate(r):
        if el in m:
            m[el].append(i+1)
        else:
            m[el] = [i+1]

    for el in r_sorted:
        b = m[el].pop(0)
        if t1 > t2:
            l2.append(b)
            t2 += s2
        else:
            l1.append(b)
            t1 += s1
    print(len(l1), ' '.join(map(str, l1)))
    print(len(l2), ' '.join(map(str, l2)))
