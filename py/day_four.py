
def one():
    with open("../src/4.in") as file:
        input = file.read().strip().split("\n")

    ans = 0
    for pair in input:
        [first, second] = pair.split(',')

        [f, s] = map(lambda x: int(x), first.split('-'))
        [t, fr] = map(lambda x: int(x), second.split('-'))

        intersect = set(range(f, s+1)).issubset(range(t, fr+1)
                                                ) or set(range(t, fr+1)).issubset(range(f, s+1))

        if intersect:
            ans += 1

    print("one ", ans)


def two():
    with open("../src/4.in") as file:
        input = file.read().strip().split("\n")

    ans = 0
    for pair in input:
        [first, second] = pair.split(',')

        [f, s] = map(lambda x: int(x), first.split('-'))
        [f1, s1] = map(lambda x: int(x), second.split('-'))

        inter = set(range(f, s+1)) & set(range(f1, s1+1))

        if len(inter) > 0:
            ans += 1

    print("two ", ans)


one()

two()

a = {1, 2, 3}
b = {4, 5, 6}
