from random import randint
import sys
sys.set_int_max_str_digits(0)
MAX = 10000
T = 100

for i in [1, 2, 3, 4]:
    with open(f'/Users/tatyam/GitHub/kyopro/ICPCDomesticJudge_Contests/test/S/S{i}', 'w') as fin:
        for t in range(T):
            N = t // 3 + 2
            if T - t <= 9:
                N = 3000
            L = [randint(1, MAX) for _ in range(N + 1)]
            fin.write(f'{N}\n')
            fin.write(' '.join(map(str, L)) + '\n')
        fin.write('0\n')
