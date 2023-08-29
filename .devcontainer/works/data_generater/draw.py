from enum import IntEnum, auto
import sys


class Drawer:
    def __init__(self, UNKNOWN_NUMBER, variables):
        self.__UNKNOWN_NUMBER = UNKNOWN_NUMBER
        self.__variables = variables

    # epcに応じてパラメータからグリッドを描画する関数
    # それぞれepcの値に応じて使い分けること。
    # p,cの中に2桁以上の数値が入る場合は表示がバグるので極力避けること。
    def visualize(self):
        result = []

        # 上端の行
        row = [self.__convert_variable(self.__variables.p[0][0], VariableAttribute.P)]
        for w in range(self.__variables.W):
            row.append(
                self.__convert_variable(self.__variables.h[0][w], VariableAttribute.H)
            )
            row.append(
                self.__convert_variable(
                    self.__variables.p[0][w + 1], VariableAttribute.P
                )
            )

        result.append("".join(row))

        for h_i in range(self.__variables.H):
            # 縦辺と細胞を含む行
            row = [
                self.__convert_variable(self.__variables.v[h_i][0], VariableAttribute.V)
            ]
            for w in range(self.__variables.W):
                row.append(
                    self.__convert_variable(
                        self.__variables.c[h_i][w], VariableAttribute.C
                    )
                )
                row.append(
                    self.__convert_variable(
                        self.__variables.v[h_i][w + 1], VariableAttribute.V
                    )
                )
            result.append("".join(row))

            # 次の水平辺の行
            row = [
                self.__convert_variable(
                    self.__variables.p[h_i + 1][0], VariableAttribute.P
                )
            ]
            for w in range(self.__variables.W):
                row.append(
                    self.__convert_variable(
                        self.__variables.h[h_i + 1][w], VariableAttribute.H
                    )
                )
                row.append(
                    self.__convert_variable(
                        self.__variables.p[h_i + 1][w + 1], VariableAttribute.P
                    )
                )
            result.append("".join(row))

        return result

    @staticmethod
    def output_to_console(grid):
        for line in grid:
            print(line)

    @staticmethod
    def output_to_txt(grid, filename):
        with open(filename, "w") as f:
            for line in grid:
                f.write(line + "\n")

    def output_to_dzn(self, output_dzn_path):
        with open(output_dzn_path, "w") as file:
            file.write(f"UNKNOWN_NUMBER = {self.__UNKNOWN_NUMBER};\n\n")
            file.write(f"W = {self.__variables.W};\n")
            file.write(f"H = {self.__variables.H};\n\n")
            file.write(self.__string_builder(self.__variables.h, "h"))
            file.write(self.__string_builder(self.__variables.v, "v"))
            file.write(self.__string_builder(self.__variables.p, "p"))
            file.write(self.__string_builder(self.__variables.c, "c"))

    def __string_builder(self, matrix, attribute):
        result = f"initial_{attribute}=[|"
        for row in matrix:
            for value in row:
                result += f"{value},"
            result = f"{result[:-1]}|"
        result += f"];\n"
        return result

    def __convert_variable(self, value, attribute):
        if attribute == VariableAttribute.H:
            if value == self.__UNKNOWN_NUMBER:
                return "┄┄┄"
            elif value == "0":
                return "   "
            elif value == "1":
                return "---"
            else:
                print("hに新仕様が追加されたようです! draw.pyの更新をお願いします!")
                sys.exit(1)

        elif attribute == VariableAttribute.V:
            if value == self.__UNKNOWN_NUMBER:
                return "¦"
            elif value == "0":
                return ""
            elif value == "1":
                return "|"
            else:
                print("vに新仕様が追加されたようです! draw.pyの更新をお願いします!")
                sys.exit(1)

        elif attribute == VariableAttribute.P:
            if value == self.__UNKNOWN_NUMBER:
                return "+"
            else:
                return value

        elif attribute == VariableAttribute.C:
            if value == self.__UNKNOWN_NUMBER:
                return "   "
            else:
                return f" {value} "

        else:
            print("不正なAttributeが渡されました! draw.pyの更新をお願いします!")
            sys.exit(1)


class VariableAttribute(IntEnum):
    H = auto()
    V = auto()
    P = auto()
    C = auto()
