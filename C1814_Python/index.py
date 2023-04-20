import sys

t, *data = map(str.split, sys.stdin.read().splitlines())

for i in range(int(t[0])):
    _, s1, s2 = map(int, data.pop(0))
    r = list(map(int, data.pop(0)))
    r_sorted = sorted(r, reverse=True)
    l1, l2 = [], []
    t1, t2 = s1, s2
    m = {el: [i+1 for i, x in enumerate(r) if x == el] for el in r}

    for el in r_sorted:
        b = m[el].pop(0)
        (l2 if t1 > t2 else l1).append(b)
        if t1 > t2:
            t2 += s2
        else:
            t1 += s1
    print(len(l1), ' '.join(map(str, l1)))
    print(len(l2), ' '.join(map(str, l2)))
