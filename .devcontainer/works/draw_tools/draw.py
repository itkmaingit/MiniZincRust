class Drawer:
    # epcに応じてパラメータからグリッドを描画する関数
    # それぞれepcの値に応じて使い分けること。
    # p,cの中に2桁以上の数値が入る場合は表示がバグるので極力避けること。
    def visualize_epc(self, H, W, h, v, p, c):
        result = []

        # 上端の行
        row = [str(p[0][0])]
        for w in range(W):
            row.append("---" if h[0][w] else "   ")
            row.append(str(p[0][w + 1]))
        result.append("".join(row))

        for h_i in range(H):
            # 縦辺と細胞を含む行
            row = ["|" if v[h_i][0] else " "]
            for w in range(W):
                row.append(" " + str(c[h_i][w]) + " ")
                row.append("|" if v[h_i][w + 1] else " ")
            result.append("".join(row))

            # 次の水平辺の行
            row = [str(p[h_i + 1][0])]
            for w in range(W):
                row.append("---" if h[h_i + 1][w] else "   ")
                row.append(str(p[h_i + 1][w + 1]))
            result.append("".join(row))

        result.append("\n========================\n")

        return result

    def visualize_ec(self, H, W, h, v, c):
        p = [["+" for _ in range(W + 1)] for _ in range(H + 1)]
        result = self.visualize_epc(H, W, h, v, p, c)

        return result

    def visualize_ep(self, H, W, h, v, p):
        c = [[" " for _ in range(W)] for _ in range(H)]
        result = self.visualize_epc(H, W, h, v, p, c)

        return result

    def visualize_e(self, H, W, h, v):
        p = [["+" for _ in range(W + 1)] for _ in range(H + 1)]
        c = [[" " for _ in range(W)] for _ in range(H)]
        result = self.visualize_epc(H, W, h, v, p, c)

        return result

    def visualize_p(self, H, W, p):
        h = [[1 for _ in range(W)] for _ in range(H + 1)]
        v = [[1 for _ in range(W + 1)] for _ in range(H)]
        c = [[" " for _ in range(W)] for _ in range(H)]
        result = self.visualize_epc(H, W, h, v, p, c)

        return result

    def visualize_c(self, H, W, c):
        h = [[1 for _ in range(W)] for _ in range(H + 1)]
        v = [[1 for _ in range(W + 1)] for _ in range(H)]
        p = [["+" for _ in range(W + 1)] for _ in range(H + 1)]
        result = self.visualize_epc(H, W, h, v, p, c)

        return result

    @staticmethod
    def output_to_console(grid):
        for line in grid:
            print(line)

    @staticmethod
    def output_to_file(grid, filename):
        with open(filename, "w") as f:
            for line in grid:
                f.write(line + "\n")


# # # サンプルデータでテスト
# h = [[0, 1], [1, 0], [1, 1]]
# v = [[1, 0, 1], [1, 1, 0]]
# p = [[3, 4, 5], [7, 6, 8], [9, 1, 1]]
# c = [[5, 6], [1, 1]]

# H = 2
# W = 2

# drawer = Drawer()

# result = drawer.visualize_epc(H, W, h, v, p, c)
# for line in result:
#     print(line)
