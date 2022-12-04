def is_fully_contained(s1_start, s1_end, s2_start, s2_end):
    s1_set = {*range(s1_start, s1_end+1)}
    s2_set = {*range(s2_start, s2_end+1)}
    print(s1_set)
    return s1_set.issubset(s2_set) or s2_set.issubset(s1_set)


def is_overlap(s1_start, s1_end, s2_start, s2_end):
    return s1_start <= s2_end and s2_start <= s1_end

if __name__ == '__main__':
    f = open("../inputs/4/input", "r")
    lines = f.readlines()
    score = 0
    for line in lines:
        section1, section2 = line.split(",")
        s1_start, s1_end = list(map(lambda s: int(s), section1.split("-")))
        s2_start, s2_end = list(map(lambda s: int(s), section2.split("-")))
        if is_overlap(s1_start, s1_end, s2_start, s2_end):
            score += 1
    print(score)
