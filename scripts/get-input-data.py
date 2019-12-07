from aocd.models import Puzzle
from datetime import date

today = date.today()
puzzle = Puzzle(year=today.year, day=today.day)

with open('day{}/src/input'.format(today.day), 'w') as fp:
    fp.write(puzzle.input_data)
