N = 9
crate_lines = 8

def get_stacks_and_commands():
    with open("../src/5.in") as fin:
        parts = fin.read().split('\n\n')

        crates_info = parts[0].split("\n")
        stacks = [[] for _ in range(N)]

        for i in range(crate_lines):
            line = crates_info[i]
            crates = line[1::4]
            for s in range(len(crates)):
                if crates[s] != " ":
                    stacks[s].append(crates[s])

    stacks = [stack[::-1] for stack in stacks]

    commands = parts[1].split('\n')

    return [stacks, commands]

def one():        
    with open("../src/5.in") as fin:
        stacks, commands = get_stacks_and_commands()

    for command in commands:
        p = command.split(' ')
        times, src, dst = map(int, [p[1], p[3], p[5]])

        src -= 1
        dst -= 1

        for i in range(times):
            item = stacks[src].pop()
            stacks[dst].append(item)


    print("".join([stack[-1] for stack in stacks]))

def two():        
    with open("../src/5.in") as fin:
        stacks, commands = get_stacks_and_commands()
        
    for command in commands:
        p = command.split(' ')
        times, src, dst = map(int, [p[1], p[3], p[5]])

        src -= 1
        dst -= 1

        move_list = []
        for i in range(times):
            move_list.append(stacks[src].pop())

        stacks[dst] = stacks[dst] + move_list[::-1]

    print("".join([stack[-1] for stack in stacks]))


one()
two()