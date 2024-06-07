"""
Test suites from

https://github.com/segviol/indigo/

and test script from

https://github.com/segviol/indigo/blob/develop/scripts/test.py

which is modified by XHZGenius to run in this repository. 

Usage: 

Put the test suites in another folder, and run the command below: 

(Assume that ../Compiler-Lab-Test-Suites is the path)

```
python test.py target/debug/compiler-lab.exe ../Compiler-Lab-Test-Suites/sysyruntimelibrary/libsysy.a ../Compiler-Lab-Test-Suites/functional_test -r --compile_only
```

"""

from array import ArrayType, array
from copyreg import constructor
import os
from os import link, path, read
import subprocess
import argparse
import re
import sys
from sys import path_hooks
from tqdm import tqdm
import logging
import colorlog
import json
from typing import List, Any
import signal
import difflib

log_colors_config = {
    'DEBUG': 'white',  # cyan white
    'INFO': 'green',
    'WARNING': 'yellow',
    'ERROR': 'red',
    'CRITICAL': 'bold_red',
}

logger = logging.getLogger('logger_name')

console_handler = logging.StreamHandler()

file_handler = logging.FileHandler(filename='../Compiler-Lab-Test-Suites/test.log',
                                   mode='w',
                                   encoding='utf8')

logger.setLevel(logging.DEBUG)
console_handler.setLevel(logging.DEBUG)
file_handler.setLevel(logging.INFO)

file_formatter = logging.Formatter(
    fmt=
    '[%(asctime)s.%(msecs)03d] %(filename)s -> %(funcName)s line:%(lineno)d [%(levelname)s] : %(message)s',
    datefmt='%H:%M:%S')
console_formatter = colorlog.ColoredFormatter(
    fmt='%(log_color)s[%(asctime)s]  : %(message)s',
    datefmt='%H:%M:%S',
    log_colors=log_colors_config)
console_handler.setFormatter(console_formatter)
file_handler.setFormatter(file_formatter)

if not logger.handlers:
    logger.addHandler(console_handler)
    logger.addHandler(file_handler)

console_handler.close()
file_handler.close()
parser = argparse.ArgumentParser(description='Automaticly test')
parser.add_argument("compiler_path",
                    metavar='compiler_path',
                    type=str,
                    help='the path of the compiler')
parser.add_argument("linked_library_path",
                    metavar='linked_library_path',
                    type=str,
                    help='the path of the linked library')
parser.add_argument('test_path',
                    metavar='test_path',
                    type=str,
                    help='the path of the test codes')
parser.add_argument('-r',
                    "--recursively",
                    help="recursively for sub dirs",
                    action="store_true")
parser.add_argument('-t',
                    "--timeout",
                    default=8,
                    help="Kill tests after specified time",
                    type=float)
parser.add_argument('-z',
                    "--performance-test",
                    help="Test and log performance",
                    action="store_true")
parser.add_argument("--compile_only", action="store_true")
args = parser.parse_args()
root_path = args.test_path

output_folder_name = "../Compiler-Lab-Test-Suites/output"
os.makedirs(output_folder_name, exist_ok=True)

max_stdout = 512


class SuccessTest:
    def __init__(self, path: str, time: float) -> None:
        self.path = path
        self.time = time


class FailedTest:
    def __init__(self, path: str, error: str, desc: any) -> None:
        self.path = path
        self.error = error
        self.desc = desc


class TestResult:
    def __init__(self, num_tested: int, num_success: int,
                 success: List[SuccessTest], failed: List[FailedTest]) -> None:
        self.num_tested = num_tested
        self.num_passed = num_success
        self.passed = success
        self.failed = failed

    def combine(self, other):
        self.num_tested += other.num_tested
        self.num_passed += other.num_passed
        self.passed.extend(other.passed)
        self.failed.extend(other.failed)


class TestEncoder(json.JSONEncoder):
    def default(self, o: Any) -> Any:
        if isinstance(o, SuccessTest):
            return {"path": o.path, "time": o.time}
        elif isinstance(o, FailedTest):
            return {"path": o.path, "error": o.error, "desc": o.desc}
        elif isinstance(o, TestResult):
            return {
                "num_tested": o.num_tested,
                "num_passed": o.num_passed,
                "passed": o.passed,
                "failed": o.failed
            }
        else:
            return json.JSONEncoder.default(self, o)


def format_compiler_output_file_name(name: str, ty: str):
    name = name.replace('/', '_')
    return f"{output_folder_name}/compiler-{ty}-{name}.txt"


def format_linker_output_file_name(name: str, ty: str):
    name = name.replace('/', '_')
    return f"{output_folder_name}/linker-{ty}-{name}.txt"


def format_function_output_file_name(name: str):
    name = name.replace('/', '_')
    return f"{output_folder_name}/program-stdout-{name}.txt"


def dump_stdout(name: str, stdout: bytes):
    filename = format_function_output_file_name(name)
    with open(filename, "wb") as f:
        f.write(stdout)


def format_function_diff_file_name(name: str):
    name = name.replace('/', '_')
    return f"{output_folder_name}/program-stdout-{name}.diff"


def dump_diff(name: str, diff: str):
    filename = format_function_diff_file_name(name)
    with open(filename, "w") as f:
        f.write(diff)


def decode_stderr_timer(stderr: str) -> float:
    time_match = re.search('TOTAL:\s*(\d+)H-(\d+)M-(\d+)S-(\d+)us\s*$', stderr)
    if time_match == None:
        return None
    else:
        hrs = float(time_match.group(1))
        mins = float(time_match.group(2))
        secs = float(time_match.group(3))
        us = float(time_match.group(4))
        return hrs * 3600 + mins * 60 + secs + us / 1000000


def test_dir(dir, test_performance: bool, is_root: bool = True) -> TestResult:
    result = TestResult(0, 0, [], [])

    files = os.listdir(dir)
    for file in tqdm(files):
        new_path = "%s/%s"%(dir, file)
        if os.path.isdir(new_path) and args.recursively:
            child = test_dir(new_path, test_performance, False)
            result.combine(child)
        elif file.split('.')[-1] == 'sy':
            result.num_tested += 1
            prefix = file.split('.')[0]

            try:
                compiler_output = subprocess.run(
                    [args.compiler_path, "-riscv", new_path, "-o", "../Compiler-Lab-Test-Suites/tmp.s"],
                    capture_output=True,
                    timeout=30)

                # compiler error
                if compiler_output.returncode != 0:
                    logger.error(f"{new_path} encountered a compiler error")

                    with open(
                            format_compiler_output_file_name(
                                new_path, 'stdout'), "w") as f:
                        f.write(compiler_output.stdout.decode('utf-8'))

                    result.failed.append(
                        FailedTest(
                            new_path, "compile_error",
                            {"return_code": compiler_output.returncode}))
                    continue
                
                if args.compile_only:
                    logger.info("%s compile succeeded. "%new_path)
                    continue

                link_output = subprocess.run([
                    'gcc', 'tmp.s', args.linked_library_path, '-march=armv7-a',
                    "-mcpu=cortex-a7", "-mfpu=neon", "-o", "tmp"
                ],
                                             capture_output=True)

                # linker error
                if link_output.returncode != 0:
                    logger.error(f"{new_path} encountered a linker error")

                    with open(
                            format_linker_output_file_name(new_path, 'stdout'),
                            "w") as f:
                        f.write(link_output.stdout.decode('utf-8'))
                    with open(
                            format_compiler_output_file_name(
                                new_path, 'stdout'), "w") as f:
                        f.write(compiler_output.stdout.decode('utf-8'))
                    with open(
                            format_linker_output_file_name(new_path, 'stderr'),
                            "w") as f:
                        f.write(link_output.stderr.decode('utf-8'))

                    result.failed.append(
                        FailedTest(new_path, "link_error",
                                   {"return_code": link_output.returncode}))
                    continue

            except subprocess.TimeoutExpired as t:
                logger.error(
                    f"{prefix} compiler time out!(longer than {t.timeout} seconds)"
                )
                stdout = t.stdout
                if stdout != None:
                    str_stdout = stdout.decode("utf-8")
                else:
                    str_stdout = ""

                with open(format_compiler_output_file_name(new_path, 'stdout'),
                          "wb") as f:
                    f.write(stdout)

                result.failed.append(
                    FailedTest(new_path, "compiler_timeout", None))

                continue

            try:
                if os.path.exists(os.path.join(dir, f"{prefix}.in")):
                    f = open(os.path.join(dir, f"{prefix}.in"), 'r')
                    process = subprocess.run(["./tmp"],
                                             stdin=f,
                                             capture_output=True,
                                             timeout=args.timeout)
                else:
                    process = subprocess.run(["./tmp"],
                                             capture_output=True,
                                             timeout=args.timeout)

                return_code = process.returncode % 256
                my_output = process.stdout.decode("utf-8").replace("\r", "")
                my_output = my_output.strip()
                limited_output = my_output[0:max_stdout]
                if len(my_output) > max_stdout:
                    limited_output += "...(stripped)"

                if return_code < 0:
                    sig = -return_code
                    sig_def = signal.strsignal(sig)
                    logger.error(
                        f"{new_path} raised a runtime error with signal {sig} ({sig_def})"
                    )

                    with open(
                            format_compiler_output_file_name(
                                new_path, 'stdout'), "w") as f:
                        f.write(compiler_output.stdout.decode('utf-8'))

                    dump_stdout(new_path, process.stdout)
                    result.failed.append(
                        FailedTest(new_path, "runtime_error", {
                            "got": limited_output,
                            "error": sig_def
                        }))

                else:
                    with open(os.path.join(dir, f"{prefix}.out"), 'r') as f:
                        std_output = f.read().replace("\r", "").strip()

                    limited_stdout = std_output[0:max_stdout]
                    if len(std_output) > max_stdout:
                        limited_stdout += "...(stripped)"

                    my_output_lines = [
                        x.strip() for x in my_output.splitlines()
                        if x.strip() != ''
                    ]
                    std_output_lines = [
                        x.strip() for x in std_output.splitlines()
                        if x.strip() != ''
                    ]
                    real_std_output = '\n'.join(std_output_lines)

                    std_ret = int(std_output_lines.pop()) % 256
                    dump_stdout(new_path, process.stdout)

                    if my_output_lines != std_output_lines:
                        diff = '\n'.join(
                            difflib.unified_diff(std_output_lines,
                                                 my_output_lines,
                                                 fromfile='std_output',
                                                 tofile='my_output',
                                                 lineterm=''))

                        logger.error(
                            f"mismatched output for {new_path}: \ndiff:\n{diff}"
                        )
                        with open(
                                format_compiler_output_file_name(
                                    new_path, 'stdout'), "w") as f:
                            f.write(compiler_output.stdout.decode('utf-8'))

                        dump_diff(new_path, diff)

                        result.failed.append(
                            FailedTest(new_path, "output_mismatch", {
                                "got": limited_output,
                                "expected": limited_stdout
                            }))
                    elif return_code != std_ret:
                        logger.error(
                            f"mismatched return code for {new_path}: \nexpected: {std_ret}\ngot {return_code}"
                        )
                        with open(
                                format_compiler_output_file_name(
                                    new_path, 'stdout'), "w") as f:
                            f.write(compiler_output.stdout.decode('utf-8'))

                        dump_stdout(new_path, process.stdout)
                        result.failed.append(
                            FailedTest(
                                new_path, "wrong_return_code", {
                                    "got_return_code": return_code,
                                    "expected_return_code": std_ret,
                                }))

                    else:
                        logger.info(f"Successfully passed {prefix}")
                        if not test_performance:
                            result.passed.append(SuccessTest(new_path, None))
                        else:
                            stderr = process.stderr.decode("utf-8")
                            t = decode_stderr_timer(stderr)
                            if t != None:
                                result.passed.append(SuccessTest(new_path, t))
                            else:
                                logger.error(f"Cannot find timer in {stderr}")

                        result.num_passed += 1
                    f.close()

            except subprocess.TimeoutExpired as t:
                logger.error(
                    f"{prefix} time out!(longer than {t.timeout} seconds)")
                stdout = t.stdout
                if stdout != None:
                    str_stdout = stdout.decode("utf-8")
                    if len(str_stdout) > max_stdout:
                        str_stdout = str_stdout[0:max_stdout] + "..."
                else:
                    str_stdout = ""

                result.failed.append(
                    FailedTest(new_path, "timeout", {
                        "got": str_stdout,
                    }))

    return result


result = test_dir(root_path, args.performance_test)
logger.info(json.dumps(result, indent=2, cls=TestEncoder))
logger.info(f"Passed {result.num_passed} of {result.num_tested} tests")

if args.performance_test:
    if os.path.exists("time.log"):
        with open("time.log", 'r', encoding="utf-8") as f:
            last_performance: dict = json.loads(f.read())
            print(
                f"Found log from last run with {len(last_performance)} entries."
            )
    else:
        last_performance = {}

    d = {}
    for test in result.passed:
        d[test.path] = test.time

    print("PERFORMANCE: ")
    for key_ in list(d):
        key = str(key_)
        last = last_performance.get(key_)
        this = d[key]
        if len(key) > 30:
            key = ".." + key[-30:]
        if last != None:
            difference = (this / last) - 1
            if difference < -0.02:
                note = "FASTER!"
            elif difference > 0.02:
                note = "SLOWER!"
            else:
                note = ""
            print("{:<32}: {:>10.6f}s; Prev: {:>10.6f}s, {:>+9.3%} {}".format(
                key, this, last, difference, note))
        else:
            print("{:<32}: {:>10.6f}s".format(key, this))

    with open('time.log', 'w', encoding="utf-8") as f:
        f.write(json.dumps(d))

if result.num_passed != result.num_tested:
    exit(1)
else:
    exit(0)