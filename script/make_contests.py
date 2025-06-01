import os
import sys

VALIDATE_PATH = './script/validate.h'

ROW_CONTESTS_PATH = './row_contests'
CONTESTS_PATH = './contests'
if not os.path.exists(CONTESTS_PATH):
    os.makedirs(CONTESTS_PATH)

row_contests = []
for filename in os.listdir(ROW_CONTESTS_PATH):
    if os.path.isdir(os.path.join(ROW_CONTESTS_PATH, filename)):
        row_contests.append(filename)

contests = []
for filename in os.listdir(CONTESTS_PATH):
    if os.path.isdir(os.path.join(CONTESTS_PATH, filename)):
        contests.append(filename)

new_contests = []
for row_contest in row_contests:
    if row_contest not in contests:
        new_contests.append(row_contest)

fix_problems = []

for contest in new_contests:
    print(f'Processing contest: {contest}')
    contest_path = os.path.join(CONTESTS_PATH, contest)
    row_contest_path = os.path.join(ROW_CONTESTS_PATH, contest)
    if not os.path.exists(contest_path):
        os.makedirs(contest_path)
    
    problems = []

    for filename in os.listdir(row_contest_path):
        if os.path.isdir(os.path.join(row_contest_path, filename)):
            problems.append(filename)
    
    for problem in problems:
        row_testcase_path = os.path.join(row_contest_path, problem)
        problem_path = os.path.join(contest_path, problem)
        if not os.path.exists(problem_path):
            os.makedirs(problem_path)

        # 必要なディレクトリを作成
        os.makedirs(os.path.join(problem_path, 'data', 'secret'), exist_ok=True)
        os.makedirs(os.path.join(problem_path, 'data', 'sample'), exist_ok=True)
        os.makedirs(os.path.join(problem_path, 'submissions'), exist_ok=True)

        # 問題情報を記述するファイルの作成
        with open(os.path.join(problem_path, 'problem.yaml'), 'w') as f:
            f.write(f'name: {problem}\n')
        
        with open(os.path.join(problem_path, '.timelimit'), 'w') as f:
            f.write('5\n')

        # テストケースのコピー
        for testcase_id in range(1, 4 + 1):
            testcase_in = os.path.join(row_testcase_path, f'{problem}{testcase_id}')
            new_testcase_in = os.path.join(problem_path, 'data', 'secret', f'{problem}{testcase_id}.in')
            os.system(f'cp {testcase_in} {new_testcase_in}')

            testcase_ans = os.path.join(row_testcase_path, f'{problem}{testcase_id}.ans')
            new_testcase_ans = os.path.join(problem_path, 'data', 'secret', f'{problem}{testcase_id}.ans')

            os.system(f'cp {testcase_ans} {new_testcase_ans}')
        
        # output_checker.cpp があるなら output_validators/different_validator に追加し、 fix_problems に追加
        output_checker_cpp = os.path.join(row_testcase_path, f'output_checker.cpp')
        if os.path.exists(output_checker_cpp):
            output_validator_path = os.path.join(problem_path, 'output_validators', 'different_validator')
            if not os.path.exists(output_validator_path):
                os.makedirs(output_validator_path)
            new_output_checker_cpp = os.path.join(output_validator_path, f'output_checker.cpp')
            validate_h = os.path.join(output_validator_path, 'validate.h')
            os.system(f'cp {output_checker_cpp} {new_output_checker_cpp}')
            os.system(f'cp {VALIDATE_PATH} {validate_h}')
            fix_problems.append(f'{contest}/{problem}')

for problem in fix_problems:
    print(f"{problem} の output_checker.cpp を修正してください。")