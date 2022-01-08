import sys
import pathlib
import re
import fileinput
from templates import SOLVE_TEMPLATE, MOD_TEMPLATE, DAY_MAIN_TEMPLATE, SOLUTION_TEMPLATE, BIN_TEMPLATE, MAIN_TEMPLATE


def create_template(day):
    template = SOLVE_TEMPLATE.replace("{{day}}", day)
    return template

def create_day(day, template):
    day_dir = pathlib.Path(f"src/bin/day{int(day):02}")
    if day_dir.exists():
        return

    day_dir.mkdir(parents=True, exist_ok=True)

    pathlib.Path(day_dir/"input.txt").touch()
    pathlib.Path(day_dir/"test.txt").touch()

    with open(day_dir/"solution.txt", "w") as fp:
        fp.write(SOLUTION_TEMPLATE)

    with open(day_dir/"solve.rs", "w") as fp:
        fp.write(template)

    with open(day_dir/"mod.rs", "w") as fp:
        fp.write(MOD_TEMPLATE)

    with open(day_dir/"main.rs", "w") as fp:
        fp.write(DAY_MAIN_TEMPLATE)

    bin_file = pathlib.Path("src/bin.rs")
    if not bin_file.is_file():
        with open(bin_file, "w") as fp:
            fp.write(BIN_TEMPLATE)

    for line in fileinput.input(bin_file, inplace=True):
        if re.match(f"(// pub mod day{int(day):02};)", line):
            line = f"pub mod day{int(day):02};\n"
        if re.match(f"(    \/\/ day{int(day):02}::solve::solve\(\);)", line):
            line = f"    day{int(day):02}::solve::solve();\n"
        sys.stdout.write(line)

    with open("src/main.rs", "w") as fp:
        fp.write(MAIN_TEMPLATE)


def main():
    day = sys.argv[1]

    if day == "all":
        for i in range(1, 26):
            template = create_template(str(i))
            create_day(str(i), template)
    else:
        template = create_template(day)
        create_day(day, template)


if __name__ == "__main__":
    main()
