with open('./assets/day02_input.txt') as in_file:
    games = in_file.readlines()

possible_games = 0

for game in games:
    g = game.split(": ")[1]
    sets = g.split(';')
    greens = 0
    reds = 0
    blues = 0
    for s in sets:
        for c in s.split(', '):
            number, color = c.strip().split(' ')
            if color == 'green':
                if int(number) > greens:
                    greens = int(number)
            elif color == 'red':
                if int(number) > reds:
                    reds = int(number)
            elif color == 'blue':
                if int(number) > blues:
                    blues = int(number)
            else:
                print("Bad color:", color)
    possible_games += (greens * reds * blues)

print(possible_games)