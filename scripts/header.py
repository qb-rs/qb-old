# SPDX-License-Identifier: AGPL-3.0-only

# ████████████████████████████████████████████████
# █─▄▄▄─█▄─██─▄█▄─▄█▄─▀─▄█▄─▄─▀█▄─█─▄█─▄─▄─█▄─▄▄─█
# █─██▀─██─██─███─███▀─▀███─▄─▀██▄─▄████─████─▄█▀█
# ▀───▄▄▀▀▄▄▄▄▀▀▄▄▄▀▄▄█▄▄▀▄▄▄▄▀▀▀▄▄▄▀▀▀▄▄▄▀▀▄▄▄▄▄▀
# https://github.com/QuixByte/qb/blob/main/LICENSE
#
# (c) Copyright 2023 The QuixByte Authors

import os

dn = os.path.dirname(os.path.realpath(__file__))

with open(os.path.join(dn, "../HEADER")) as f:
    header_template = f.readlines()

spdx_license = "AGPL-3.0-only"
langs = {
    "rs": "// ",
    "py": "# ",
    "yaml": "# ",
    "toml": "# ",
    "bash": "# ",
    "md": "[//]: # ",
}

def construct(ext: str) -> str:
    comment = langs[ext]
    spdx = f"{comment}SPDX-License-Identifier: {spdx_license}\n\n"
    header = "".join([ comment + line for line in header_template ])

    return spdx + header

# Code for checking
root = os.path.normpath(os.path.join(dn, ".."))

ignore_exts = [
    "lock"
]
ignore_files = [
    "AUTHORS",
    "HEADER",
    "LICENSE",
    ".gitignore"
]
ignore_dirs = [
    "target",
    "scripts/__pycache__",
    ".git"
]

fail = False

for subdir, _, files in os.walk(root):
    for file in files:
        relsubdir = subdir[len(root) + 1:] + "/"

        found = False
        for ignore_dir in ignore_dirs:
            if relsubdir.startswith(ignore_dir + "/"):
                found = True

        if found:
            continue

        path = os.path.join(subdir, file)
        
        if file in ignore_files:
            continue

        print("SCAN:", file, path)

        ext = file.split(".")[-1]
        if ext in ignore_exts:
            continue

        with open(path) as f:
            content = f.readlines()
        expected = construct(ext)
    
        for (i, line) in enumerate(expected.split("\n")):
            if i >= len(content) or content[i].strip() != line.strip():
                print("WARN:", file, "does not contain the correct header:", i)
                print(expected)
                fail = True
                break

if fail:
    exit(1)
