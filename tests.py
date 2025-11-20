import os
import subprocess
import pathlib

EXAMPLES_DIR = "examples"
BENCH_DIR = "benchmarks/suite"


def load_files(path: pathlib.Path) -> list[pathlib.Path]:
    files = []
    for sub_path in path.iterdir():
        if sub_path.is_dir():
            files.extend(load_files(sub_path))
        if sub_path.name.endswith("sc"):
            files.append(sub_path)
    return files


def run_examples(paths: pathlib.Path):
    succ = []
    fail = []
    for path in paths:
        path_str = "/".join(path.parts)
        print("Running {name}".format(name=path.name))
        exit_code = subprocess.call(
            ["target/debug/scc", "codegen", path_str, "x86-64"],
            stderr=subprocess.DEVNULL,
        )
        if exit_code == 0:
            succ.append(path.name)
        else:
            fail.append(path.name)
    print("Successfully ran: {succ}".format(succ=", ".join(succ)))
    print("Failures: {fail}".format(fail=", ".join(fail)))


if __name__ == "__main__":
    paths = load_files(pathlib.Path(EXAMPLES_DIR))
    paths.extend(load_files(pathlib.Path(BENCH_DIR)))
    run_examples(paths)
