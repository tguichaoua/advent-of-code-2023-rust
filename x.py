import datetime
import os
import sys
import webbrowser


def run(command):
    print(">", command)
    if os.system(command) != 0:
        sys.exit(1)


def get_day():
    if len(sys.argv) > 1:
        try:
            return int(sys.argv[1])
        except:
            ...
    return datetime.datetime.today().day


def main():
    day = get_day()
    run(f"cargo scaffold {day}")
    run(f"cargo download {day}")
    run("code .")
    webbrowser.open(f"https://adventofcode.com/2023/day/{day}")


if __name__ == "__main__":
    main()
