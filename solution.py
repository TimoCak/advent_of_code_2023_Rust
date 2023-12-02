def main():
    with open('./assets/day02_input.txt', 'rt') as in_file:
        data = in_file.readlines()

    print(conundrum(data))


def conundrum(data):
    red, green, blue = 12, 13, 14
    counter = 1
    possible = []
    for line in data:
        colon = line.find(':')
        space = len('Game ')
        game_number = int(line[space:colon])
        game = line[colon + 2:].rstrip().split('; ')
        answers = []
        for set in game:
            for cube in set.split(', '):
                num = int(cube[:2].strip())
                color = cube[2:].strip()
                if color == 'red' and num > red:
                    print(counter)
                    answers.append(False)
                elif color == 'blue' and num > blue:
                    print(counter)
                    answers.append(False)
                elif color == 'green' and num > green:
                    print(counter)
                    answers.append(False)
                else:  
                    answers.append(True)

       
        if all(answers):
            possible.append(game_number)

        counter = counter + 1    

    return sum(possible)


if __name__ == '__main__':
    main()