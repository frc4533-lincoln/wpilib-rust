import subprocess
from typing import List, Optional
import git
import os
import sys


def run_cargo_command(command: List[str]) -> Optional[str]:
    # run via subprocess
    handle = subprocess.run(["cargo"] + command)
    if handle.returncode != 0:
        return handle.stderr


def main(msg: str):
    fmt_err = run_cargo_command(["fmt"])
    if fmt_err:
        print(fmt_err)
        sys.exit(1)


def get_msg():
    if len(sys.argv) < 2:
        print("Please provide a commit message")
        sys.exit(1)
    return sys.argv[1]


main(get_msg())
