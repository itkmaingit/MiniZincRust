import csv
import os
import sys
import draw

UNKNOWN_NUMBER = "99999"


def main(*args):
    input_file_path = args[0][0]
    output_dzn_path = args[0][1]
    output_txt_path = args[0][2]
    if os.path.exists(output_dzn_path):
        os.remove(output_dzn_path)

    h, v, p, c = create_variables(input_file_path)

    variables = Variables(h, v, p, c)
    drawer = draw.Drawer(UNKNOWN_NUMBER, variables)
    grid = drawer.visualize()
    drawer.output_to_dzn(output_dzn_path)
    drawer.output_to_console(grid)
    drawer.output_to_txt(grid, output_txt_path)


def create_variables(input_file_path):
    h = []
    v = []
    p = []
    c = []
    with open(input_file_path, "r") as file:
        reader = csv.reader(file)

        for i, row in enumerate(reader):
            if i % 2 == 0:
                h_row = []
                p_row = []
                for j, value in enumerate(row):
                    value = value if value != "-" else UNKNOWN_NUMBER
                    if j % 2 == 0:
                        p_row.append(value)
                    else:
                        h_row.append(value)
                p.append(p_row)
                h.append(h_row)

            elif i % 2 == 1:
                v_row = []
                c_row = []
                for j, value in enumerate(row):
                    value = value if value != "-" else UNKNOWN_NUMBER
                    if j % 2 == 0:
                        v_row.append(value)
                    else:
                        c_row.append(value)
                v.append(v_row)
                c.append(c_row)

    return h, v, p, c


class Variables:
    def __init__(self, h, v, p, c):
        self.h = h
        self.v = v
        self.p = p
        self.c = c
        self.H = len(c)
        self.W = len(c[0])
        self.__validate()

    def __validate(self):
        if len(self.h) != self.H + 1 or len(self.h[0]) != self.W:
            print("c, あるいはhのデータ構造が間違っています!")
            sys.exit(1)
        if len(self.v) != self.H or len(self.v[0]) != self.W + 1:
            print("c, あるいはvのデータ構造が間違っています!")
            sys.exit(1)
        if len(self.p) != self.H + 1 or len(self.p[0]) != self.W + 1:
            print("c, あるいはhのデータ構造が間違っています!")
            sys.exit(1)


if __name__ == "__main__":
    args = sys.argv[1:]
    main(args)
