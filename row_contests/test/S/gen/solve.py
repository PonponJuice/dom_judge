INF = 1 << 61
def solve():
    N = int(input())
    if N == 0:
        exit(0)
    L = [int(x) for x in input().split()]
    dp = [[INF] * (i + 1) for i in range(N + 1)]
    for i in range(N):
        dp[i + 1][i] = 0
    for j in range(N + 1):
        for k in range(j - 1, -1, -1):
            for i in range(k - 1, -1, -1):
                x = dp[j][k] + dp[k][i] + L[j] * L[k] * L[i]
                if dp[j][i] > x:
                    dp[j][i] = x
    print(dp[N][0], flush=True)

while 1: solve()
