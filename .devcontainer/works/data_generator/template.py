import sys


def main(*args):
    output_csv_path = args[0][0]
    output_epc_txt_path = args[0][1]
    epc = args[0][2]
    H = int(args[0][3])
    W = int(args[0][4])
    print("それぞれのh, v, p, cについてデフォルト値は何にしますか？")
    print("特になければ - を挿入します。")
    h = ""
    v = ""
    p = ""
    c = ""

    if epc[0] == "1":
        print("\nhのデフォルト値は何にしますか？")
        print("h: ", end="")
        h = input()
        print("\nvのデフォルト値は何にしますか？")
        print("v: ", end="")
        v = input()

    if epc[1] == "1":
        print("\npのデフォルト値は何にしますか？")
        print("p: ", end="")
        p = input()

    if epc[2] == "1":
        print("\ncのデフォルト値は何にしますか？")
        print("c: ", end="")
        c = input()

    if not h:
        h = "-"
    if not v:
        v = "-"
    if not p:
        p = "-"
    if not c:
        c = "-"

    with open(output_csv_path, "w") as file:
        for i in range(2 * H + 1):
            row = []
            for j in range(2 * W + 1):
                if i % 2 == 0:
                    if j % 2 == 0:
                        row.append(f"{p},")
                    else:
                        row.append(f"{h},")
                else:
                    if j % 2 == 0:
                        row.append(f"{v},")
                    else:
                        row.append(f"{c},")
            row[-1] = row[-1].replace(",", "\n")
            file.write("".join(row))

    with open(output_epc_txt_path, "w") as file:
        file.write(f"{epc} {H} {W}")


if __name__ == "__main__":
    args = sys.argv[1:]
    main(args)
