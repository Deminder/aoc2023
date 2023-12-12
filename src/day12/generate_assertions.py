import re

def arrangements(conditions):
    if not conditions:
        yield ''
    else:
        first, tail = conditions[0], conditions[1:]
        if first == '?':
            yield from arrangements('.' + tail)
            yield from arrangements('#' + tail)
        else:
            for comb in arrangements(tail):
                yield first + comb


def combinations_count(line):
    conds, nums = line.split(' ')
    nums = [int(n) for n in nums.split(',')]
    rg = re.compile('^\\.*' + '\\.+'.join(f'#{{{n}}}' for n in nums) + '\\.*$')
    return sum(1 for comb in arrangements(conds) if rg.match(comb))


for (l, c) in ((line, combinations_count(line)) for line in open('input.txt').readlines()):
    print(f'assert_eq!(arrangements("{l[:-1]}", false), {c});')

