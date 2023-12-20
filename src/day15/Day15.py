
import time


def h(s):
    v = 0
    for c in s:
        v += ord(c)
        v *= 17
        v %= 256
    return v


def run(line):
    boxes = [[] for _ in range(256)]
    for op in line.split(','):
        label, num = (op[:-2], int(op[-1])
                      ) if op[-1].isdigit() else (op[:-1], -1)

        l = boxes[h(label)]
        index = next((i for i, (k, _) in enumerate(l) if k == label), None)
        if index is None:
            if num != -1:
                l.append((label, num))
        elif num == -1:
            l.pop(index)
        else:
            l[index] = (label, num)

    return sum((bi+1) * (si+1) * fl for bi, l in enumerate(boxes) for si, (_, fl) in enumerate(l))


line = open('input.txt').read().strip()
print('Part1:', sum(h(op) for op in line.split(',')))
print('Part2:', run(line))

for _ in range(1000):
    run(line)
count = 20000
start = time.time()
for _ in range(count):
    run(line)
duration = time.time() - start
print(f'Part2 duration avg: {duration / count} s')
