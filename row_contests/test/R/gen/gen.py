from random import randint, random
from math import exp
import sys
sys.set_int_max_str_digits(0)
MAX = 9.7
T = 1000

def R():
    return int(exp(random() * MAX))

for i in [1, 2, 3, 4]:
    with open(f'/Users/tatyam/GitHub/kyopro/ICPCDomesticJudge_Contests/test/R/R{i}', 'w') as fin, open(f'/Users/tatyam/GitHub/kyopro/ICPCDomesticJudge_Contests/test/R/R{i}.ans', 'w') as fout:
        for t in range(T):
            A = randint(1, 1 << R())
            B = randint(1, 1 << R())
            fin.write(f'{A} {B}\n')
            fout.write(f'{A + B}\n')
        fin.write('0 0\n')
