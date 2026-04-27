#!/usr/bin/python
import pathlib
import sys
import subprocess
import math
import enum
import shutil

EXAMPLE_DIR = "examples"
BENCHMARK_DIR = "benchmarks/suite"


def progress(num_files: int, num_done: int, current_file: pathlib.Path) -> None:
    term_width = shutil.get_terminal_size()[0]
    bar_template = "[{h}{s}] {p}% (compiling {n})"
    bar_len = term_width - (len(bar_template) - 12) - 3 - len(current_file.name)
    percent = float(num_done) / float(num_files)
    num_hashes = math.floor(float(bar_len) * percent)
    hashes = "#" * num_hashes
    spaces = " " * (bar_len - num_hashes)
    print(
        bar_template.format(
            h=hashes,
            s=spaces,
            p=str(math.floor(percent * 100.0)).zfill(3),
            n=current_file.name,
        ),
        end="\r",
    )


class CMD(enum.Enum):
    COMPILE_ALL = "compile-all"
    COMPILE_ONE = "compile-one"
    RUN_ALL = "run-all"
    RUN_ONE = "run-one"


def cmd_from_str(cmd_str: str) -> CMD:
    for cmd in CMD:
        if cmd_str == cmd.value:
            return cmd
    raise ValueError("Not a valid command: {cmd}".format(cmd=cmd_str))


def describe_cmd(cmd: CMD) -> str:
    match cmd:
        case CMD.COMPILE_ALL:
            return "Compile all examples in {examples} and {benchmarks}".format(
                examples=EXAMPLE_DIR, benchmarks=BENCHMARK_DIR
            )
        case CMD.COMPILE_ONE:
            return "Compile one example (n times), requires --{arg_file}".format(
                arg_file=ARG.FILE_NAME.value[0]
            )
        case CMD.RUN_ALL:
            return "Compile and run all examples in {examples} and {benchmarks}".format(
                examples=EXAMPLE_DIR, benchmarks=BENCHMARK_DIR
            )
        case CMD.RUN_ONE:
            return (
                "Compile and run one example (n times), requires --{arg_file}".format(
                    arg_file=ARG.FILE_NAME.value[0]
                )
            )


def command_help() -> list[str]:
    help_strs = []
    for cmd in CMD:
        help_strs.append("{val}\t{desc}".format(val=cmd.value, desc=describe_cmd(cmd)))
    return help_strs


class ARG(enum.Enum):
    FILE_NAME = ("file", "f")
    NUM = ("num", "n")
    HELP = ("help", "h")


def describe_arg(arg: ARG) -> str:
    match arg:
        case ARG.FILE_NAME:
            return "Name of the file to use (only applies to commands for single files)"
        case ARG.NUM:
            return "Number of times to run (optional, only applies to commands for single files)"
        case ARG.HELP:
            return "Print help and exit"


def arg_help() -> list[str]:
    help_strs = []
    for arg in ARG:
        help_strs.append(
            "-{arg_short}, --{arg_long} \t{desc}".format(
                arg_short=arg.value[1], arg_long=arg.value[0], desc=describe_arg(arg)
            )
        )
    return help_strs


def arg_from_str(arg_str: str) -> ARG:
    for arg in ARG:
        if arg_str == arg.value[0] or arg_str == arg.value[1]:
            return arg
    raise ValueError("Not a valid argument: {arg}".format(arg=arg_str))


class Cli:
    file_name: str
    command: CMD
    args: list[tuple[ARG, str]]

    def __init__(self):
        self.args = []
        self.file_name = sys.argv[0]

        default_command = CMD.RUN_ALL
        args = sys.argv[1:]
        if len(args) == 0:
            self.command = default_command
            return

        if args[0].startswith("-"):
            self.command = default_command
        else:
            self.command = cmd_from_str(args[0])
            args = args[1:]

        arg_ind = 0
        while arg_ind < len(args):
            arg_stripped = (
                args[arg_ind][2:]
                if args[arg_ind].startswith("--")
                else args[arg_ind][1:]
            )
            arg = arg_from_str(arg_stripped)
            if arg == ARG.HELP:
                self.args.append((arg, ()))
                arg_ind += 1
                continue

            if arg_ind + 1 == len(args) or args[arg_ind + 1].startswith("-"):
                raise ValueError("No value for argument {arg}".format(arg=arg_stripped))
            arg_val = args[arg_ind + 1]
            self.args.append((arg, arg_val.strip()))
            arg_ind += 2

    def help_text(self) -> str:
        return """
Usage {name} COMMAND [ARGUMENT]
Compile examples and report results

Commands:
        {cmd_help}

Arguments
        {arg_help}

When running compiled files, arguments are loaded from the ".args" file in the same directory
""".format(
            name=self.file_name,
            cmd_help="\n\t".join(command_help()),
            arg_help="\n\t".join(arg_help()),
        )

    def get_file_path(self) -> pathlib.Path:
        for arg in self.args:
            if arg[0] == ARG.FILE_NAME:
                file_path = pathlib.Path(arg[1])
                if file_path.exists():
                    return file_path
                else:
                    raise FileNotFoundError(file_path)
        raise ValueError("Missing argument file name")

    def get_num(self) -> int:
        for arg in self.args:
            if arg[0] == ARG.NUM:
                return int(arg[1])
        return 1

    def run(self):
        if (ARG.HELP, ()) in self.args:
            print(self.help_text(), end="")
            exit(0)

        match self.command:
            case CMD.COMPILE_ALL:
                print(
                    "Compiling all examples in {examples} and {benchmarks}".format(
                        examples=EXAMPLE_DIR, benchmarks=BENCHMARK_DIR
                    )
                )
                report_results(compile_all())
            case CMD.COMPILE_ONE:
                to_compile = self.get_file_path()
                num_compile = self.get_num()
                print(
                    "Compiling {file} {n} times".format(
                        file=to_compile.name, n=num_compile
                    )
                )
                report_results(compile_n_times(to_compile, num_compile))
            case CMD.RUN_ONE:
                to_run = self.get_file_path()
                num_run = self.get_num()
                print("Running {file} {n} times".format(file=to_run.name, n=num_run))
                report_results(run_n_times(to_run, num_run))
            case CMD.RUN_ALL:
                print(
                    "Running all examples in {examples} and {benchmarks}".format(
                        examples=EXAMPLE_DIR, benchmarks=BENCHMARK_DIR
                    )
                )
                report_results(run_all())


class RESULT(enum.Enum):
    SUCCESS = 1
    COMPILE_FAIL = 2
    RUN_FAIL = 3
    UNEXPECTED_OUT = 4


def is_fail(res: RESULT) -> bool:
    return not (res == RESULT.SUCCESS)


def describe_res(res: RESULT) -> str:
    out = ""
    match res:
        case RESULT.SUCCESS:
            out = "success"
        case RESULT.COMPILE_FAIL:
            out = "failed compiling"
        case RESULT.RUN_FAIL:
            out = "failed running"
        case RESULT.UNEXPECTED_OUT:
            out = "failed with unexpected output"
    return out


class RunResult:
    file: pathlib.Path
    result: RESULT
    msg: str

    def __init__(self, file: pathlib.Path, res: RESULT, msg: str):
        self.file = file
        self.result = res
        self.msg = msg

    def describe(self) -> str:
        return "{file} {res}{msg}".format(
            file=self.file.name,
            res=describe_res(self.result),
            msg="\n\t{msg}".format(msg=self.msg) if self.msg != "" else "",
        )


def report_results(results: list[RunResult]):
    num_tests = len(results)
    fails = list(filter(lambda res: is_fail(res.result), results))
    num_fails = len(fails)
    num_succ = num_tests - num_fails
    print()
    print(
        "Ran {num} files, {succ} successses, {fail} fails".format(
            num=num_tests, succ=num_succ, fail=num_fails
        )
    )
    if num_fails != 0:
        print(
            "Fails: {fails}".format(
                fails="\n".join(list(map(lambda res: res.describe(), fails)))
            )
        )


class Example:
    args: list[str]
    heap_size: None | int
    expected: str
    source_file: pathlib.Path

    def __init__(self, file: pathlib.Path):
        self.source_file = file
        self.heap_size = None
        args_dir = file.parent
        args_name = file.name.replace(".sc", ".args")
        args_path = args_dir.joinpath(args_name)
        if not args_path.exists():
            raise FileNotFoundError(args_path)
        args_file = open(args_path, "r")

        for line in args_file.readlines():
            parts = line.split("=")
            (key, val) = (parts[0].strip(), parts[1].strip())
            if key == "test_args":
                val = val.replace("[", "").replace("]", "").replace('"', "")
                vals = val.split(",")
                self.args = list(
                    filter(lambda val: val != "", map(lambda val: val.strip(), vals))
                )
            elif key == "expected":
                self.expected = val.replace('"', "").replace("\\n", "\n")
            elif key == "heap_size":
                self.heap_size = int(val.strip())

        if self.args is None:
            raise ValueError(
                "Could not get test arguments for {file}".format(file=file.name)
            )
        if self.expected is None:
            raise ValueError("Could not get expected for {file}".format(file=file.name))

    def compile(self) -> RunResult:
        run_args = ["target/debug/scc", "codegen", self.source_file, "x86-64"]
        if self.heap_size is not None:
            run_args.append("--heap-size")
            run_args.append(str(self.heap_size))
        result = subprocess.run(
            run_args,
            capture_output=True,
        )
        if result.returncode == 0:
            return RunResult(self.source_file, RESULT.SUCCESS, "")
        else:
            return RunResult(
                self.source_file, RESULT.COMPILE_FAIL, result.stderr.decode("utf-8")
            )

    def compile_n_times(self, n: int) -> list[RunResult]:
        results = []
        for i in range(0, n):
            progress(n, i, self.source_file)
            result = self.compile()
            results.append(result)
        return results

    def run(self) -> RunResult:
        compile_res = self.compile()
        if is_fail(compile_res.result):
            return compile_res
        file_name = self.source_file.name.replace(".sc", "")
        run_args = ["./target_scc/bin/x86_64/{file}".format(file=file_name)]
        run_args.extend(self.args)
        run_res = subprocess.run(run_args, capture_output=True)
        if run_res.returncode != 0:
            return RunResult(
                self.source_file,
                RESULT.RUN_FAIL,
                'code:{exit},stdout:"{out}", stderr:"{err}"'.format(
                    exit=run_res.returncode,
                    out=run_res.stdout.decode("utf-8").strip(),
                    err=run_res.stderr.decode("utf-8").strip(),
                ),
            )
        out = run_res.stdout.decode("utf-8").strip()
        if out != self.expected:
            return RunResult(
                self.source_file,
                RESULT.UNEXPECTED_OUT,
                'Got: "{got}", expected: "{expected}"'.format(
                    got=out, expected=self.expected
                ),
            )

        return RunResult(self.source_file, RESULT.SUCCESS, "")

    def run_n_times(self, n: int) -> list[RunResult]:
        results = []
        for i in range(0, n):
            progress(n, i, self.source_file)
            res = self.run()
            results.append(res)
        progress(n, n, self.source_file)
        return results


def get_files(dir: pathlib.Path) -> list[Example]:
    files = []
    for file in dir.iterdir():
        if file.is_dir():
            files.extend(get_files(file))
        if file.is_file() and file.suffix == ".sc":
            files.append(Example(file))
    return files


def compile_files(examples: list[Example]) -> list[RunResult]:
    results = []
    num_done = 0
    for example in examples:
        progress(len(examples), num_done, example.source_file)
        res = example.compile()
        results.append(res)
        num_done += 1
    progress(len(examples), num_done, example.source_file)
    print()
    return results


def run_files(examples: list[Example]) -> list[RunResult]:
    results = []
    num_done = 0
    for example in examples:
        progress(len(examples), num_done, example.source_file)
        res = example.run()
        results.append(res)
        num_done += 1
    progress(len(examples), num_done, example.source_file)
    print()
    return results


def compile_all() -> list[RunResult]:
    files = get_files(pathlib.Path(EXAMPLE_DIR))
    files.extend(get_files(pathlib.Path(BENCHMARK_DIR)))
    return compile_files(files)


def run_all() -> list[RunResult]:
    files = get_files(pathlib.Path(EXAMPLE_DIR))
    files.extend(get_files(pathlib.Path(BENCHMARK_DIR)))
    return run_files(files)


if __name__ == "__main__":
    print("Recompiling Scc")
    build_res = subprocess.run(["cargo", "build"], capture_output=True)
    if build_res.returncode != 0:
        raise RuntimeError(
            '"cargo build" exited with exit code {exit}'.format(
                exit=build_res.returncode
            )
        )
    args = Cli()
    args.run()
