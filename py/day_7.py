from collections import defaultdict
import sys

SYS_SIZE = 70_000_000
MAX_FOLDER_SIZE = 100_000
FOR_UPDATE_SIZE = 30_000_000
def one():
    with open("../src/7.in") as file:
        input = file.read().strip().split('\n')

    dir_sum = defaultdict(int)
    dir_tree = []

    sum_of_total = 0

    for command in input:
        if command[0:4] == '$ ls':
            continue
        if command[0:4] == '$ cd':
            if command[5:] == '..':
                dir_tree.pop()
            else:
                dir_tree.append(command[5:])
            continue

        [out, _] = command.split(' ')
        if out.isdigit():
            s = int(out)
            temp_path = ''
            for k in dir_tree:
                temp_path += k
                dir_sum[temp_path] += s 

    for [k, v] in dir_sum.items():
        if(v <= MAX_FOLDER_SIZE):
            sum_of_total += v

    available = SYS_SIZE - dir_sum['/']
    minimum = FOR_UPDATE_SIZE - available
    ans = sys.maxsize

    for [k,v] in dir_sum.items():
        if v >= minimum and v < ans:
            ans = v
                
    print(f"Folder size that could be removed: [{ans}]")
    print(f"Sum of folders <= 100_000: [{sum_of_total}]")

def two():
    with open("../src/7.in") as fin:
        blocks = ("\n" + fin.read().strip()).split("\n$ ")[1:]


    path = []

    dir_sizes = defaultdict(int)
    children = defaultdict(list)


    def parse(block):
        lines = block.split("\n")
        command = lines[0]
        outputs = lines[1:]

        parts = command.split(" ")
        op = parts[0]
        if op == "cd":
            if parts[1] == "..":
                path.pop()
            else:
                path.append(parts[1])

            return

        abspath = "/".join(path)
        assert op == "ls"

        sizes = []
        for line in outputs:
            if not line.startswith("dir"):
                sizes.append(int(line.split(" ")[0]))
            else:
                dir_name = line.split(" ")[1]
                children[abspath].append(f"{abspath}/{dir_name}")

        dir_sizes[abspath] = sum(sizes)


    for block in blocks:
        parse(block)


    # Do DFS
    def dfs(abspath):
        size = dir_sizes[abspath]
        for child in children[abspath]:
            size += dfs(child)
        return size


    ans = 0
    l = 0
    for abspath in dir_sizes:
        if dfs(abspath) <= 100000:
            l += 1
            a = dfs(abspath)
            print('dir with less than 100_000', abspath, ' with size ', a, l)
            ans += a

    print("two ", ans)


one()
print("\n\n\n\n\n")
two()