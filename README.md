Advent of Code 2025
---

Solutions to all 12 days of Advent of Code 2025 written in (mostly) rust.

Dependencies: `rustc` (obviously), python `z3-solver` for day 10 part 2, python `requests` for the script in root directory.

Each day contains a single `main.rs` file, to be compiled with `rustc main.rs` (optionally with `-O`).
Day 10 has an additional `main.py` file for part 2.

Most days were pretty fun and easy.
Difficulty massively increased on day 9.
Day 10 felt like it required a math library to complete (maybe you could implement simplex? I tried sympy first and it didn't work and I think that uses simplex).
Day 12 is dumb and stupid "let's give you an NP-hard problem but the input is specifically designed in a way that you dont even need to solve the actual problem".
