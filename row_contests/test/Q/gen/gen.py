from random import randint
MAX = 10 ** 18
T = 100

for i in [1, 2, 3, 4]:
    with open(f'/Users/tatyam/GitHub/kyopro/ICPCDomesticJudge_Contests/test/Q/Q{i}', 'w') as fin, open(f'/Users/tatyam/GitHub/kyopro/ICPCDomesticJudge_Contests/test/Q/Q{i}.ans', 'w') as fout:
        for t in range(T):
            mx = int(MAX ** ((t + 1) / T))
            A = randint(-mx, mx)
            B = randint(-mx, mx)
            while A == 0:
                A = randint(-mx, mx)
            while B == 0:
                B = randint(-mx, mx)
            fin.write(f'{A} {B}\n')
            fout.write(f'{A + B}\n')
        fin.write('0 0\n')
