#!/usr/bin/env python3

# ========== Advent of Code 2022 - helper script ===========
# This script is used to create a new day's folder and files

import os
import sys


def main(day_number):
    path = f'src/Day{day_number}'
    os.mkdir(path)
    open(f'{path}/input.txt', 'w').close()
    open(f'{path}/task1.rs', 'w').close()
    open(f'{path}/task2.rs', 'w').close()
    os.system(f'echo "# Day{day_number}" >> {path}/README.md')

    def new_task(nr):
        return f"""
[[bin]]
name = "day{day_number}-task{nr}"
path = "src/Day{day_number}/task{nr}.rs"
"""
    with open('Cargo.toml') as f:
        cargo = f.read()
    with open('Cargo.toml', 'w') as f:
        cargo = cargo.replace(f'\n[dependencies]', f'{new_task(1)}{new_task(2)}\n[dependencies]')
        print(cargo)
        f.write(cargo)


if __name__ == "__main__":
    day_nr = int(sys.argv[1])
    main(day_nr)
