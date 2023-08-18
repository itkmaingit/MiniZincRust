import sys

import draw


def main(*args):
    file_path = args[0][0]
    drawer = draw.Drawer()
    solution_nums = 0
    with open(file_path, "r") as file:
        while True:
            grid = parse_grid(file, drawer, solution_nums)
            drawer.output_to_console(grid)

            line = file.readline()

            # ファイルの終端もしくは ----- を検出した場合、新しいループに入る
            if line.startswith("-----"):
                solution_nums += 1
                continue


def parse_grid(file, drawer, solution_nums):
    line = file.readline()
    # ファイルの終端の場合、プログラムを終了させる。
    if not line or line.startswith("===="):
        print(f"solutions: {solution_nums}\n")
        sys.exit(0)
    # epc, H, W を読み込む
    epc, H, W = line.split()
    H, W = int(H), int(W)
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

    if epc == "111":
        return drawer.visualize_epc(H, W, h, v, p, c)
    elif epc == "110":
        return drawer.visualize_ep(H, W, h, v, p)
    elif epc == "101":
        return drawer.visualize_ec(H, W, h, v, c)
    elif epc == "100":
        return drawer.visualize_e(H, W, h, v)
    elif epc == "010":
        return drawer.visualize_p(H, W, p)
    elif epc == "001":
        return drawer.visualize_c(H, W, c)


if __name__ == "__main__":
    args = sys.argv[1:]
    main(args)
