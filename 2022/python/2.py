mapping = {
    "X": 1,
    "Y": 2,
    "Z": 3,
    "A": 1,
    "B": 2,
    "C": 3
}


def get_score(enemy, me):
    return [3, 6, 0][((me - 1) - (enemy - 1)) % 3]

    scores = [
        [3, 6, 0],
        [0, 3, 6],
        [6, 0, 3]
    ]
    return scores[enemy - 1][me - 1]


def get_action(enemy, guide):
    return [1, 2, 3][((guide - 1) + (enemy + 1)) % 3]

    actions = [
        [3, 1, 2],
        [1, 2, 3],
        [2, 3, 1]
    ]
    return actions[enemy - 1][guide - 1]


if __name__ == '__main__':
    print(sum(list(
        map(lambda line: [1, 2, 3][((line[1] - 1) + (line[0] + 1)) % 3] + [3, 6, 0][
            (([1, 2, 3][((line[1] - 1) + (line[0] + 1)) % 3] - 1) - (line[0] - 1)) % 3], list(
            map(lambda x: list(map(lambda entry: mapping[entry], x[:-1].split(" "))),
                open("../inputs/2/input", "r").readlines()))))))
