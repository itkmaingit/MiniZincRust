import sys
import os
import shutil
import subprocess
from tqdm import tqdm


def main(*args):
    input_file_path = args[0][0]
    input_epc_path = args[0][1]
    output_dir_path = args[0][2]

    index = 0
    if os.path.exists(output_dir_path):
        shutil.rmtree(output_dir_path)
    os.makedirs(output_dir_path, exist_ok=True)
    with open(input_epc_path, "r") as epc_txt:
        line = epc_txt.readline()
        epc, H, W = line.split()
        H = int(H)
        W = int(W)

    file_line_count = int(
        subprocess.check_output(["wc", "-l", input_file_path]).decode().split(" ")[0]
    )
    line_counts_per_solution = 2
    if epc[0] == "1":
        line_counts_per_solution += 2 * H + 1
    if epc[1] == "1":
        line_counts_per_solution += H + 1
    if epc[2] == "1":
        line_counts_per_solution += H

    solution_counts = file_line_count / line_counts_per_solution

    with tqdm(total=int(solution_counts)) as pbar:
        with open(input_file_path, "r") as file:
            while True:
                variables = parse_grid(epc, H, W, file)
                output(
                    output_dir_path,
                    index,
                    variables.variables_list(),
                )

                line = file.readline()

                # ファイルの終端もしくは ----- を検出した場合、新しいループに入る
                if line.startswith("-----"):
                    index += 1
                    pbar.update(1)
                    continue


def parse_grid(epc, H, W, file):
    line = file.readline()
    # ファイルの終端の場合、プログラムを終了させる。
    if not line or line.startswith("===="):
        sys.exit(0)

    h = []
    v = []
    p = []
    c = []

    if epc[0] == "1":
        for _ in range(H + 1):
            h_row = list(map(int, file.readline().split()))
            h.append(h_row)

        for _ in range(H):
            v_row = list(map(int, file.readline().split()))
            v.append(v_row)

    if epc[1] == "1":
        for _ in range(H + 1):
            p_row = list(map(int, file.readline().split()))
            p.append(p_row)

    if epc[2] == "1":
        for _ in range(H):
            c_row = list(map(int, file.readline().split()))
            c.append(c_row)

    variables = Variables(H, W, h, v, p, c)

    return variables


def output(output_dir_path, file_index, variables_list):
    append_list = []
    for variable in variables_list:
        for row in variable:
            for value in row:
                append_list.append(f"{value} ")
            append_list[-1] = append_list[-1].replace(" ", "\n")

    output_file_path = os.path.join(output_dir_path, f"{file_index}.txt")
    with open(output_file_path, "w") as output_file:
        output_file.write("".join(append_list))


class Variables:
    def __init__(self, H, W, h, v, p, c):
        self.H = H
        self.W = W
        self.h = h
        self.v = v
        self.p = p
        self.c = c
        self.__validate()

    def __validate(self):
        if self.h != [] and (len(self.h) != self.H + 1 or len(self.h[0]) != self.W):
            print("hのデータ構造が間違っています!")
            sys.exit(1)
        if self.v != [] and (len(self.v) != self.H or len(self.v[0]) != self.W + 1):
            print("vのデータ構造が間違っています!")
            sys.exit(1)
        if self.p != [] and (len(self.p) != self.H + 1 or len(self.p[0]) != self.W + 1):
            print("pのデータ構造が間違っています!")
            sys.exit(1)
        if self.c != [] and (len(self.c) != self.H or len(self.c[0]) != self.W):
            print("cのデータ構造が間違っています!")
            sys.exit(1)

    def variables_list(self):
        return [self.h, self.v, self.p, self.c]


if __name__ == "__main__":
    print("\n-----MiniZinc終了-----\n")
    print("-----ファイル分割中-----\n")
    args = sys.argv[1:]
    main(args)
