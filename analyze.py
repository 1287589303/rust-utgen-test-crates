import argparse
import ast
import json
import os
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

parser = argparse.ArgumentParser()
parser.add_argument("dir", help="the directory of the csv file")


def get_includes(target_dirs) -> set[str, int, int]:
    includes: set[str] = set()
    for target_dir in target_dirs:
        for root, _, files in os.walk(target_dir):
            for file in files:
                if file.endswith(".json"):
                    with open(os.path.join(root, file), "r") as f:
                        data = json.load(f)
                        if data["tests_run"] > 0:
                            includes.add(
                                (
                                    data["function_name"],
                                    data["lines"],
                                    data["branches"],
                                )
                            )
    return includes


def main():
    crate_directory = "strsim-rs"
    target_dirs = [
        os.path.join(crate_directory, "rutgen_N/result"),
        os.path.join(crate_directory, "rutgen_r/result"),
        os.path.join(crate_directory, "rutgen_c/result"),
        os.path.join(crate_directory, "rutgen_r_c/result"),
        os.path.join(crate_directory, "rutgen_r_c/fixed_result"),
        os.path.join(crate_directory, "rutgen_r_c_o/result"),
        os.path.join(crate_directory, "rutgen_r_c_o/fixed_result"),
    ]

    includes = get_includes(target_dirs)
    for target_dir in target_dirs:
        all_tests_gen = 0
        all_tests_lines_list = []
        all_can_compile = 0
        all_tests_run = 0
        all_tests_pass = 0
        line_cov_list = []
        branch_cov_list = []
        project_lines = 0
        project_lines_covered = 0
        project_branches = 0
        project_branches_covered = 0
        effective_lines = 0
        effective_lines_covered = 0
        effective_branches = 0
        effective_branches_covered = 0
        function_num = 0
        for root, _, files in os.walk(target_dir):
            for file in files:
                if file.endswith(".json"):
                    with open(os.path.join(root, file), "r") as f:
                        data = json.load(f)
                        function = data["function_name"]
                        function_num += 1
                        tests_gen = data["tests"]
                        tests_lines = data["tests_lines"]
                        can_compile = data["tests_compiled"]
                        tests_run = data["tests_run"]
                        tests_pass = data["tests_passed"]
                        lines = data["lines"]
                        lines_covered = data["lines_covered"]
                        line_cov = data["lines_coveraged_rate"]
                        branches = data["branches"]
                        branches_covered = data["branches_covered"]
                        branch_cov = data["branches_coveraged_rate"]
                        # if is_real and tests_run and int(tests_run) > int(can_compile):
                        #     print(
                        #         f"Tests run: {tests_run}, Can compile: {can_compile} at {function} in {file}"
                        #     )
                        all_tests_gen += tests_gen
                        all_tests_lines_list.extend(tests_lines)
                        all_can_compile += can_compile
                        if tests_run > 0:
                            all_tests_run += tests_run
                            all_tests_pass += tests_pass
                            line_cov_list.append(float(line_cov))
                            branch_cov_list.append(float(branch_cov))
                        project_lines += lines
                        project_lines_covered += lines_covered
                        project_branches += branches
                        project_branches_covered += branches_covered
                        for include in includes:
                            if include[0] == function:
                                effective_lines += include[1]
                                effective_lines_covered += lines_covered
                                effective_branches += include[2]
                                effective_branches_covered += branches_covered
                                break
        compile_pass_rate = all_can_compile / all_tests_gen
        tests_pass_rate = all_tests_pass / all_tests_run if all_tests_run > 0 else 0.0
        mean_line_cov = np.mean(line_cov_list)
        median_line_cov = np.median(line_cov_list)
        mean_branch_cov = np.mean(branch_cov_list)
        median_branch_cov = np.median(branch_cov_list)
        project_line_cov = (
            project_lines_covered / project_lines if project_lines > 0 else 0.0
        )
        project_branch_cov = (
            project_branches_covered / project_branches if project_lines > 0 else 0.0
        )
        effective_lines_cov = (
            effective_lines_covered / effective_lines if project_lines > 0 else 0.0
        )
        effective_branches_cov = (
            effective_branches_covered / effective_branches
            if project_lines > 0
            else 0.0
        )
        print(f"Target dir: {target_dir}")
        print(f"Function number: {function_num}")
        print(
            f"Generated tests: {all_tests_gen}, All tests lines: {np.sum(all_tests_lines_list)}, Mean: {np.mean(all_tests_lines_list)}, Median: {np.median(all_tests_lines_list)}."
        )
        print(
            f"Tests pass compilation: {all_can_compile}, Compile pass rate: {100*compile_pass_rate:.2f}%."
        )
        print(
            f"Running tests: {all_tests_run}, Tests pass: {all_tests_pass}, Tests pass rate: {100*tests_pass_rate:.2f}%."
        )
        print(
            f"Line coverage: Mean: {mean_line_cov:.2f}%, Median: {median_line_cov:.2f}%."
        )
        print(
            f"Branch coverage: Mean: {mean_branch_cov:.2f}%, Median: {median_branch_cov:.2f}%."
        )
        print(
            f"Effective Line coverage: {100*effective_lines_cov:.2f}%, Effective Branch coverage: {100*effective_branches_cov:.2f}%."
        )
        print(
            f"Project Line coverage: {100*project_line_cov:.2f}%, Project Branch coverage: {100*project_branch_cov:.2f}%."
        )
        print(f"All Lines: {project_lines}, All Branches: {project_branches}")
        print("\n")


if __name__ == "__main__":
    main()
