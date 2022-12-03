from typing import List

scores = [
    'a',
    'b',
    'c',
    'd',
    'e',
    'f',
    'g',
    'h',
    'i',
    'j',
    'k',
    'l',
    'm',
    'n',
    'o',
    'p',
    'q',
    'r',
    's',
    't',
    'u',
    'v',
    'w',
    'x',
    'y',
    'z',
]
scores = scores + list(map(lambda c: c.upper(), scores))


def getScore(char):
    return scores.index(char) + 1


def pad(char_arr: List[str]):
    newArr = []
    index = 0
    for x in scores:
        if x in char_arr:
            newArr.append(x)
        else:
            newArr.append("_")
    return newArr


if __name__ == '__smain__':
    f = open("../inputs/3/input", "r")
    lines = f.readlines()

    sum = 0
    for line in lines:
        line = list(line)
        first = line[:int(len(line) / 2)]
        second = line[int(len(line) / 2):]
        for x in first:
            if x in second:
                sum += getScore(x)
                break
    print(sum)

if __name__ == '__main__':
    f = open("../inputs/3/input", "r")
    lines = f.readlines()
    i = iter(lines)
    lines = list(zip(i, i, i))
    sum = 0
    for group in lines:
        group = list(group)
        for x in group[0]:
            if x in group[1]:
                if x in group[2]:
                    sum += getScore(x)
                    break
    print(sum)
