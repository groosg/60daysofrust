# 60 Days of Rust

This repo keeps track of my learning experience with Rust.

- Day 1: [Development environment with Neovim](./day1/)
- Day 2: [Programming a Guessing Game](./day2/)
- Day 3: [Guessing Game Web](./day3/)
- Day 4: [Programming a Single-Threaded Web Server](./day4/)
- Day 5: [relaxing](./day5/)
- Day 6: [still relaxing...](./day6/)
- Day 7: [Multithreaded Web Server + Graceful Shutdown](./day7/)

## Bootstrap a day

```shell
day=4; mkdir day${day}; cd $_ && cargo init --vcs none && sed s/{day}/${day}/g ../day_readme_tpl > README.md
```


