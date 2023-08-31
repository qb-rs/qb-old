# SPDX-License-Identifier: AGPL-3.0-only

# ████████████████████████████████████████████████
# █─▄▄▄─█▄─██─▄█▄─▄█▄─▀─▄█▄─▄─▀█▄─█─▄█─▄─▄─█▄─▄▄─█
# █─██▀─██─██─███─███▀─▀███─▄─▀██▄─▄████─████─▄█▀█
# ▀───▄▄▀▀▄▄▄▄▀▀▄▄▄▀▄▄█▄▄▀▄▄▄▄▀▀▀▄▄▄▀▀▀▄▄▄▀▀▄▄▄▄▄▀
# https://github.com/QuixByte/qb/blob/main/LICENSE

# (c) Copyright 2023 The QuixByte Authors

import os
import sys

dn = os.path.dirname(os.path.realpath(__file__))

with open(os.path.join(dn, "../HEADER")) as f:
    header_template = f.readlines()

def prefix(pre):
    return lambda s: pre + s

def xml(s):
    return "<!-- " + s + " -->"

def css(s):
    return "/* " + s + " */"

spdx_license = "AGPL-3.0-only"
langs = {
    "rs": prefix("// "),
    "js": prefix("// "),
    "ts": prefix("// "),
    "py": prefix("# "),
    "yaml": prefix("# "),
    "toml": prefix("# "),
    "bash": prefix("# "),
    "env": prefix("# "),
    "conf": prefix("# "),
    "Dockerfile": prefix("# "),
    "sql": prefix("-- "),
    "md": xml,
    "html": xml,
    "svelte": xml,
    "css": css,
}

def construct(ext: str) -> str:
    comment = langs[ext]
    header = "".join([
        (comment(line.rstrip()) if line.rstrip() else "") + "\n"
        for line in header_template
    ])

    return header

# Code for checking
root = os.path.normpath(os.path.join(dn, ".."))

ignore_exts = [
    "json",

    # Image files
    "png",
    "jpg",
    "jpeg",
    "webp",
    "bmp",
    "svg",
    "ico",
]
ignore_files = [
    "AUTHORS",
    "HEADER",
    "LICENSE",

    ".gitignore",
    ".dockerignore",
    ".eslintrc",

    # Package lock files
    "yarn.lock",
    "bun.lockb",
    "package-lock.json",
    "pnpm-lock.yaml",
    "Cargo.lock",
]
ignore_dirs = [
    "__pycache__/",
    "node_modules/",
    "build/",
    ".svelte-kit/",
    "target/",
    ".git/"
]

write = len(sys.argv) == 2 and sys.argv[1] == "write"

if write:
    print("INFO: write enabled")

fail = False

for subdir, _, files in os.walk(root):
    for file in files:
        relsubdir = subdir[len(root) + 1:] + "/"

        found = False
        for ignore_dir in ignore_dirs:
            if ignore_dir in relsubdir:
                found = True

        if found:
            continue

        path = os.path.join(subdir, file)
        
        if file in ignore_files:
            continue

        ext = file.split(".")[-1]
        if ext in ignore_exts:
            continue

        print("SCAN:", file, path)

        with open(path) as f:
            content = f.read()
        content_lines = content.split("\n")
        expected = construct(ext)
        expected_lines = expected.split("\n")
    
        for (i, line) in enumerate(expected_lines):
            if i >= len(content_lines) or content_lines[i].strip() != line.strip():
                print("WARN:", file, "does not contain the correct header:", i)
                if write:
                    print("WRITE:", file)
                    with open(path, "w") as f:
                        f.write(expected + "\n" + content)
                else:
                    print(expected)
                    fail = True
                break

if fail:
    exit(1)
