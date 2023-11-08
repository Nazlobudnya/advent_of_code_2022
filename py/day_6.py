with open("../src/6.in") as fin:
    parts = fin.read().strip()

for i in range(len(parts)):
    p = parts[i:i+26]
    if len(set(p)) == 26:
        print(p);
        print(i+26);
        break;