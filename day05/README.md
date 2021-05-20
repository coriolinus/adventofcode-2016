# Day 05

## Parallelism

While I upgraded this with the explicit intention to enable parallelism, it turns out that
parallelism doesn't actually help much in this case. Hyperfine results, release mode:

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `./day05-no-parallel` | 2.025 ± 0.011 | 2.016 | 2.054 | 1.00 |
| `./day05-parallel` | 2.152 ± 0.005 | 2.145 | 2.159 | 1.06 ± 0.01 |

Therefore, parallelism remains disabled by default. In the future, it might be worth trying this
on a more powerful/more parallel machine to see if the situation has improved.

Of interest is the fact that though part2 needs to do significantly more work than part1, parallelism
still didn't help there; the part2 benchmarks had almost identical ratios as the part1. Apparently
whatever problem-space-division / locking heuristics rayon uses under the hood are poorly suited to
this particular problem.

## Testing

The tests for this day can be fairly slow. It is recommended to test in release mode.
