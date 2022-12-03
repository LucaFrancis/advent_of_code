if __name__ == '__main__':
    f = open("../inputs/1/input", "r")
    lines = f.readlines()

    elves = []
    elf = []
    for line in lines:
        if line == "\n":
            elves.append(elf)
            elf = []
        else:
            elf.append(line)
    elves = list(map(lambda inventory: list(map(lambda food: int(food), inventory)), elves))
    elves = list(map(lambda inventory: sum(inventory), elves))
    elves.sort(reverse=True)
    top3 = 0
    for elf in range(3):
        print(elves[elf])
        top3 += elves[elf]
    print(max(elves))
    print(top3)
