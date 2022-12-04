#!/usr/bin/python3

def one():
    with open("../src/2.in") as file:
        input = file.read().strip().split("\n")

    hand_mapping = {
        "A": 1,
        "B": 2,
        "C": 3,
        "X": 1,
        "Y": 2,
        "Z": 3
    }

    ans = 0

    for play in input:
        enemy, player = [hand_mapping[i] for i in play.split(" ")]

        if (player - enemy) % 3 == 1:
            ans += 6
        elif player == enemy:
            ans += 3

        ans += player

    print(ans)


one()
