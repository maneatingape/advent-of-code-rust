# Advent of Code [![checks-badge]][checks-link] [![docs-badge]][docs-link]

Complete 2024 to 2015 entries written in Rust for the annual [Advent of Code] challenge,
solving 500 stars in less than 1 second.

## Features

* Each solution uses the most efficient algorithms to the best of my knowledge.
* Self contained depending only on the `std` Rust library. No use of `unsafe` features.
* Consistently formatted with `rustfmt` and linted by `clippy`.
* Thoroughly commented with `rustdoc` generated [documentation online][docs-link].
* Test coverage with continuous integration provided by [GitHub Actions][checks-link].

## Quickstart

<details>
<summary>Show details</summary><p/>

**Input**

Place input files in `input/yearYYYY/dayDD.txt` including leading zeroes. For example:
* `input/year2015/day23.txt`
* `input/year2023/day02.txt`

**Run**
* Everything `cargo run`
* Specific year `cargo run year2023`
* Specific day `cargo run year2023::day01`
* Release profile (faster) `cargo run --release`
* Optimized for current CPU architecture (fastest) `RUSTFLAGS="-C target-cpu=native" cargo run --release`

**Test**
* Everything `cargo test`
* Specific year `cargo test year2023`
* Specific day `cargo test year2023::day01`
* Show STDOUT for debugging `cargo test -- --nocapture`

**Benchmark**
* Everything `cargo bench`
* Specific year `cargo bench year2023`
* Specific day `cargo bench year2023::day01`

**Document**
* Build docs including private items `cargo doc --document-private-items`
* Build doc then open HTML landing page `cargo doc --document-private-items --open`

**Miscellaneous**
* Code quality lints `cargo clippy`
* Consistent code formatting `cargo fmt`

</details>

## Performance

Benchmarks are measured using the built-in `cargo bench` tool run on an [Apple M2 Max][apple-link].
All 250 solutions from 2024 to 2015 complete sequentially in **580 milliseconds**.
Interestingly 84% of the total time is spent on just 9 solutions.
Performance is reasonable even on older hardware, for example a 2011 MacBook Pro with an
[Intel i7-2720QM][intel-link] processor takes 3.5 seconds to run the same 225 solutions.

![pie-all]

| Year | 2015 | 2016 | 2017 | 2018 | 2019 | 2020 | 2021 | 2022 | 2023 | 2024 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Benchmark (ms) | 24 | 119 | 88 | 35 | 16 | 270 | 9 | 8 | 6 | 5 |

## 2024

![pie-2024]

| Day | Problem | Solution | Benchmark (μs) |
| --- | --- | --- | --: |
| 1 | [Historian Hysteria](https://adventofcode.com/2024/day/1) | [Source](src/year2024/day01.rs) | 21 |
| 2 | [Red-Nosed Reports](https://adventofcode.com/2024/day/2) | [Source](src/year2024/day02.rs) | 43 |
| 3 | [Mull It Over](https://adventofcode.com/2024/day/3) | [Source](src/year2024/day03.rs) | 8 |
| 4 | [Ceres Search](https://adventofcode.com/2024/day/4) | [Source](src/year2024/day04.rs) | 77 |
| 5 | [Print Queue](https://adventofcode.com/2024/day/5) | [Source](src/year2024/day05.rs) | 18 |
| 6 | [Guard Gallivant](https://adventofcode.com/2024/day/6) | [Source](src/year2024/day06.rs) | 331 |
| 7 | [Bridge Repair](https://adventofcode.com/2024/day/7) | [Source](src/year2024/day07.rs) | 136 |
| 8 | [Resonant Collinearity](https://adventofcode.com/2024/day/8) | [Source](src/year2024/day08.rs) | 8 |
| 9 | [Disk Fragmenter](https://adventofcode.com/2024/day/9) | [Source](src/year2024/day09.rs) | 106 |
| 10 | [Hoof It](https://adventofcode.com/2024/day/10) | [Source](src/year2024/day10.rs) | 38 |
| 11 | [Plutonian Pebbles](https://adventofcode.com/2024/day/11) | [Source](src/year2024/day11.rs) | 248 |
| 12 | [Garden Groups](https://adventofcode.com/2024/day/12) | [Source](src/year2024/day12.rs) | 289 |
| 13 | [Claw Contraption](https://adventofcode.com/2024/day/13) | [Source](src/year2024/day13.rs) | 14 |
| 14 | [Restroom Redoubt](https://adventofcode.com/2024/day/14) | [Source](src/year2024/day14.rs) | 74 |
| 15 | [Warehouse Woes](https://adventofcode.com/2024/day/15) | [Source](src/year2024/day15.rs) | 303 |
| 16 | [Reindeer Maze](https://adventofcode.com/2024/day/16) | [Source](src/year2024/day16.rs) | 390 |
| 17 | [Chronospatial Computer](https://adventofcode.com/2024/day/17) | [Source](src/year2024/day17.rs) | 2 |
| 18 | [RAM Run](https://adventofcode.com/2024/day/18) | [Source](src/year2024/day18.rs) | 42 |
| 19 | [Linen Layout](https://adventofcode.com/2024/day/19) | [Source](src/year2024/day19.rs) | 118 |
| 20 | [Race Condition](https://adventofcode.com/2024/day/20) | [Source](src/year2024/day20.rs) | 1038 |
| 21 | [Keypad Conundrum](https://adventofcode.com/2024/day/21) | [Source](src/year2024/day21.rs) | 19 |
| 22 | [Monkey Market](https://adventofcode.com/2024/day/22) | [Source](src/year2024/day22.rs) | 1216 |
| 23 | [LAN Party](https://adventofcode.com/2024/day/23) | [Source](src/year2024/day23.rs) | 43 |
| 24 | [Crossed Wires](https://adventofcode.com/2024/day/24) | [Source](src/year2024/day24.rs) | 23 |
| 25 | [Code Chronicle](https://adventofcode.com/2024/day/25) | [Source](src/year2024/day25.rs) | 8 |

## 2023

![pie-2023]

| Day | Problem | Solution | Benchmark (μs) |
| --- | --- | --- | --: |
| 1 | [Trebuchet?!](https://adventofcode.com/2023/day/1) | [Source](src/year2023/day01.rs) | 37 |
| 2 | [Cube Conundrum](https://adventofcode.com/2023/day/2) | [Source](src/year2023/day02.rs) | 9 |
| 3 | [Gear Ratios](https://adventofcode.com/2023/day/3) | [Source](src/year2023/day03.rs) | 53 |
| 4 | [Scratchcards](https://adventofcode.com/2023/day/4) | [Source](src/year2023/day04.rs) | 20 |
| 5 | [If You Give A Seed A Fertilizer](https://adventofcode.com/2023/day/5) | [Source](src/year2023/day05.rs) | 18 |
| 6 | [Wait For It](https://adventofcode.com/2023/day/6) | [Source](src/year2023/day06.rs) | 1 |
| 7 | [Camel Cards](https://adventofcode.com/2023/day/7) | [Source](src/year2023/day07.rs) | 71 |
| 8 | [Haunted Wasteland](https://adventofcode.com/2023/day/8) | [Source](src/year2023/day08.rs) | 34 |
| 9 | [Mirage Maintenance](https://adventofcode.com/2023/day/9) | [Source](src/year2023/day09.rs) | 18 |
| 10 | [Pipe Maze](https://adventofcode.com/2023/day/10) | [Source](src/year2023/day10.rs) | 41 |
| 11 | [Cosmic Expansion](https://adventofcode.com/2023/day/11) | [Source](src/year2023/day11.rs) | 12 |
| 12 | [Hot Springs](https://adventofcode.com/2023/day/12) | [Source](src/year2023/day12.rs) | 387 |
| 13 | [Point of Incidence](https://adventofcode.com/2023/day/13) | [Source](src/year2023/day13.rs) | 66 |
| 14 | [Parabolic Reflector Dish](https://adventofcode.com/2023/day/14) | [Source](src/year2023/day14.rs) | 632 |
| 15 | [Lens Library](https://adventofcode.com/2023/day/15) | [Source](src/year2023/day15.rs) | 84 |
| 16 | [The Floor Will Be Lava](https://adventofcode.com/2023/day/16) | [Source](src/year2023/day16.rs) | 826 |
| 17 | [Clumsy Crucible](https://adventofcode.com/2023/day/17) | [Source](src/year2023/day17.rs) | 2289 |
| 18 | [Lavaduct Lagoon](https://adventofcode.com/2023/day/18) | [Source](src/year2023/day18.rs) | 17 |
| 19 | [Aplenty](https://adventofcode.com/2023/day/19) | [Source](src/year2023/day19.rs) | 100 |
| 20 | [Pulse Propagation](https://adventofcode.com/2023/day/20) | [Source](src/year2023/day20.rs) | 6 |
| 21 | [Step Counter](https://adventofcode.com/2023/day/21) | [Source](src/year2023/day21.rs) | 182 |
| 22 | [Sand Slabs](https://adventofcode.com/2023/day/22) | [Source](src/year2023/day22.rs) | 54 |
| 23 | [A Long Walk](https://adventofcode.com/2023/day/23) | [Source](src/year2023/day23.rs) | 640 |
| 24 | [Never Tell Me The Odds](https://adventofcode.com/2023/day/24) | [Source](src/year2023/day24.rs) | 95 |
| 25 | [Snowverload](https://adventofcode.com/2023/day/25) | [Source](src/year2023/day25.rs) | 179 |

## 2022

![pie-2022]

| Day | Problem | Solution | Benchmark (μs) |
| --- | --- | --- | --: |
| 1 | [Calorie Counting](https://adventofcode.com/2022/day/1) | [Source](src/year2022/day01.rs) | 14 |
| 2 | [Rock Paper Scissors](https://adventofcode.com/2022/day/2) | [Source](src/year2022/day02.rs) | 3 |
| 3 | [Rucksack Reorganization](https://adventofcode.com/2022/day/3) | [Source](src/year2022/day03.rs) | 13 |
| 4 | [Camp Cleanup](https://adventofcode.com/2022/day/4) | [Source](src/year2022/day04.rs) | 8 |
| 5 | [Supply Stacks](https://adventofcode.com/2022/day/5) | [Source](src/year2022/day05.rs) | 14 |
| 6 | [Tuning Trouble](https://adventofcode.com/2022/day/6) | [Source](src/year2022/day06.rs) | 3 |
| 7 | [No Space Left On Device](https://adventofcode.com/2022/day/7) | [Source](src/year2022/day07.rs) | 14 |
| 8 | [Treetop Tree House](https://adventofcode.com/2022/day/8) | [Source](src/year2022/day08.rs) | 51 |
| 9 | [Rope Bridge](https://adventofcode.com/2022/day/9) | [Source](src/year2022/day09.rs) | 107 |
| 10 | [Cathode-Ray Tube](https://adventofcode.com/2022/day/10) | [Source](src/year2022/day10.rs) | 2 |
| 11 | [Monkey in the Middle](https://adventofcode.com/2022/day/11) | [Source](src/year2022/day11.rs) | 1173 |
| 12 | [Hill Climbing Algorithm](https://adventofcode.com/2022/day/12) | [Source](src/year2022/day12.rs) | 57 |
| 13 | [Distress Signal](https://adventofcode.com/2022/day/13) | [Source](src/year2022/day13.rs) | 15 |
| 14 | [Regolith Reservoir](https://adventofcode.com/2022/day/14) | [Source](src/year2022/day14.rs) | 205 |
| 15 | [Beacon Exclusion Zone](https://adventofcode.com/2022/day/15) | [Source](src/year2022/day15.rs) | 2 |
| 16 | [Proboscidea Volcanium](https://adventofcode.com/2022/day/16) | [Source](src/year2022/day16.rs) | 59 |
| 17 | [Pyroclastic Flow](https://adventofcode.com/2022/day/17) | [Source](src/year2022/day17.rs) | 71 |
| 18 | [Boiling Boulders](https://adventofcode.com/2022/day/18) | [Source](src/year2022/day18.rs) | 121 |
| 19 | [Not Enough Minerals](https://adventofcode.com/2022/day/19) | [Source](src/year2022/day19.rs) | 74 |
| 20 | [Grove Positioning System](https://adventofcode.com/2022/day/20) | [Source](src/year2022/day20.rs) | 3785 |
| 21 | [Monkey Math](https://adventofcode.com/2022/day/21) | [Source](src/year2022/day21.rs) | 64 |
| 22 | [Monkey Map](https://adventofcode.com/2022/day/22) | [Source](src/year2022/day22.rs) | 61 |
| 23 | [Unstable Diffusion](https://adventofcode.com/2022/day/23) | [Source](src/year2022/day23.rs) | 1521 |
| 24 | [Blizzard Basin](https://adventofcode.com/2022/day/24) | [Source](src/year2022/day24.rs) | 62 |
| 25 | [Full of Hot Air](https://adventofcode.com/2022/day/25) | [Source](src/year2022/day25.rs) | 3 |

## 2021

![pie-2021]

| Day | Problem | Solution | Benchmark (μs) |
| --- | --- | --- | --: |
| 1 | [Sonar Sweep](https://adventofcode.com/2021/day/1) | [Source](src/year2021/day01.rs) | 6 |
| 2 | [Dive!](https://adventofcode.com/2021/day/2) | [Source](src/year2021/day02.rs) | 12 |
| 3 | [Binary Diagnostic](https://adventofcode.com/2021/day/3) | [Source](src/year2021/day03.rs) | 20 |
| 4 | [Giant Squid](https://adventofcode.com/2021/day/4) | [Source](src/year2021/day04.rs) | 12 |
| 5 | [Hydrothermal Venture](https://adventofcode.com/2021/day/5) | [Source](src/year2021/day05.rs) | 181 |
| 6 | [Lanternfish](https://adventofcode.com/2021/day/6) | [Source](src/year2021/day06.rs) | 1 |
| 7 | [The Treachery of Whales](https://adventofcode.com/2021/day/7) | [Source](src/year2021/day07.rs) | 8 |
| 8 | [Seven Segment Search](https://adventofcode.com/2021/day/8) | [Source](src/year2021/day08.rs) | 14 |
| 9 | [Smoke Basin](https://adventofcode.com/2021/day/9) | [Source](src/year2021/day09.rs) | 64 |
| 10 | [Syntax Scoring](https://adventofcode.com/2021/day/10) | [Source](src/year2021/day10.rs) | 25 |
| 11 | [Dumbo Octopus](https://adventofcode.com/2021/day/11) | [Source](src/year2021/day11.rs) | 55 |
| 12 | [Passage Pathing](https://adventofcode.com/2021/day/12) | [Source](src/year2021/day12.rs) | 25 |
| 13 | [Transparent Origami](https://adventofcode.com/2021/day/13) | [Source](src/year2021/day13.rs) | 22 |
| 14 | [Extended Polymerization](https://adventofcode.com/2021/day/14) | [Source](src/year2021/day14.rs) | 11 |
| 15 | [Chiton](https://adventofcode.com/2021/day/15) | [Source](src/year2021/day15.rs) | 2403 |
| 16 | [Packet Decoder](https://adventofcode.com/2021/day/16) | [Source](src/year2021/day16.rs) | 6 |
| 17 | [Trick Shot](https://adventofcode.com/2021/day/17) | [Source](src/year2021/day17.rs) | 7 |
| 18 | [Snailfish](https://adventofcode.com/2021/day/18) | [Source](src/year2021/day18.rs) | 404 |
| 19 | [Beacon Scanner](https://adventofcode.com/2021/day/19) | [Source](src/year2021/day19.rs) | 615 |
| 20 | [Trench Map](https://adventofcode.com/2021/day/20) | [Source](src/year2021/day20.rs) | 2066 |
| 21 | [Dirac Dice](https://adventofcode.com/2021/day/21) | [Source](src/year2021/day21.rs) | 278 |
| 22 | [Reactor Reboot](https://adventofcode.com/2021/day/22) | [Source](src/year2021/day22.rs) | 378 |
| 23 | [Amphipod](https://adventofcode.com/2021/day/23) | [Source](src/year2021/day23.rs) | 1714 |
| 24 | [Arithmetic Logic Unit](https://adventofcode.com/2021/day/24) | [Source](src/year2021/day24.rs) | 4 |
| 25 | [Sea Cucumber](https://adventofcode.com/2021/day/25) | [Source](src/year2021/day25.rs) | 551 |

## 2020

![pie-2020]

| Day | Problem | Solution | Benchmark (μs) |
| --- | --- | --- | --: |
| 1 | [Report Repair](https://adventofcode.com/2020/day/1) | [Source](src/year2020/day01.rs) | 12 |
| 2 | [Password Philosophy](https://adventofcode.com/2020/day/2) | [Source](src/year2020/day02.rs) | 35 |
| 3 | [Toboggan Trajectory](https://adventofcode.com/2020/day/3) | [Source](src/year2020/day03.rs) | 12 |
| 4 | [Passport Processing](https://adventofcode.com/2020/day/4) | [Source](src/year2020/day04.rs) | 49 |
| 5 | [Binary Boarding](https://adventofcode.com/2020/day/5) | [Source](src/year2020/day05.rs) | 11 |
| 6 | [Custom Customs](https://adventofcode.com/2020/day/6) | [Source](src/year2020/day06.rs) | 35 |
| 7 | [Handy Haversacks](https://adventofcode.com/2020/day/7) | [Source](src/year2020/day07.rs) | 69 |
| 8 | [Handheld Halting](https://adventofcode.com/2020/day/8) | [Source](src/year2020/day08.rs) | 8 |
| 9 | [Encoding Error](https://adventofcode.com/2020/day/9) | [Source](src/year2020/day09.rs) | 9 |
| 10 | [Adapter Array](https://adventofcode.com/2020/day/10) | [Source](src/year2020/day10.rs) | 1 |
| 11 | [Seating System](https://adventofcode.com/2020/day/11) | [Source](src/year2020/day11.rs) | 4537 |
| 12 | [Rain Risk](https://adventofcode.com/2020/day/12) | [Source](src/year2020/day12.rs) | 12 |
| 13 | [Shuttle Search](https://adventofcode.com/2020/day/13) | [Source](src/year2020/day13.rs) | 1 |
| 14 | [Docking Data](https://adventofcode.com/2020/day/14) | [Source](src/year2020/day14.rs) | 83 |
| 15 | [Rambunctious Recitation](https://adventofcode.com/2020/day/15) | [Source](src/year2020/day15.rs) | 147000 |
| 16 | [Ticket Translation](https://adventofcode.com/2020/day/16) | [Source](src/year2020/day16.rs) | 120 |
| 17 | [Conway Cubes](https://adventofcode.com/2020/day/17) | [Source](src/year2020/day17.rs) | 443 |
| 18 | [Operation Order](https://adventofcode.com/2020/day/18) | [Source](src/year2020/day18.rs) | 24 |
| 19 | [Monster Messages](https://adventofcode.com/2020/day/19) | [Source](src/year2020/day19.rs) | 362 |
| 20 | [Jurassic Jigsaw](https://adventofcode.com/2020/day/20) | [Source](src/year2020/day20.rs) | 42 |
| 21 | [Allergen Assessment](https://adventofcode.com/2020/day/21) | [Source](src/year2020/day21.rs) | 45 |
| 22 | [Crab Combat](https://adventofcode.com/2020/day/22) | [Source](src/year2020/day22.rs) | 5911 |
| 23 | [Crab Cups](https://adventofcode.com/2020/day/23) | [Source](src/year2020/day23.rs) | 110000 |
| 24 | [Lobby Layout](https://adventofcode.com/2020/day/24) | [Source](src/year2020/day24.rs) | 4320 |
| 25 | [Combo Breaker](https://adventofcode.com/2020/day/25) | [Source](src/year2020/day25.rs) | 20 |

## 2019

![pie-2019]

| Day | Problem | Solution | Benchmark (μs) |
| --- | --- | --- | --: |
| 1 | [The Tyranny of the Rocket Equation](https://adventofcode.com/2019/day/1) | [Source](src/year2019/day01.rs) | 1 |
| 2 | [1202 Program Alarm](https://adventofcode.com/2019/day/2) | [Source](src/year2019/day02.rs) | 1 |
| 3 | [Crossed Wires](https://adventofcode.com/2019/day/3) | [Source](src/year2019/day03.rs) | 17 |
| 4 | [Secure Container](https://adventofcode.com/2019/day/4) | [Source](src/year2019/day04.rs) | 12 |
| 5 | [Sunny with a Chance of Asteroids](https://adventofcode.com/2019/day/5) | [Source](src/year2019/day05.rs) | 3 |
| 6 | [Universal Orbit Map](https://adventofcode.com/2019/day/6) | [Source](src/year2019/day06.rs) | 29 |
| 7 | [Amplification Circuit](https://adventofcode.com/2019/day/7) | [Source](src/year2019/day07.rs) | 79 |
| 8 | [Space Image Format](https://adventofcode.com/2019/day/8) | [Source](src/year2019/day08.rs) | 4 |
| 9 | [Sensor Boost](https://adventofcode.com/2019/day/9) | [Source](src/year2019/day09.rs) | 1008 |
| 10 | [Monitoring Station](https://adventofcode.com/2019/day/10) | [Source](src/year2019/day10.rs) | 1092 |
| 11 | [Space Police](https://adventofcode.com/2019/day/11) | [Source](src/year2019/day11.rs) | 341 |
| 12 | [The N-Body Problem](https://adventofcode.com/2019/day/12) | [Source](src/year2019/day12.rs) | 1309 |
| 13 | [Care Package](https://adventofcode.com/2019/day/13) | [Source](src/year2019/day13.rs) | 2527 |
| 14 | [Space Stoichiometry](https://adventofcode.com/2019/day/14) | [Source](src/year2019/day14.rs) | 17 |
| 15 | [Oxygen System](https://adventofcode.com/2019/day/15) | [Source](src/year2019/day15.rs) | 360 |
| 16 | [Flawed Frequency Transmission](https://adventofcode.com/2019/day/16) | [Source](src/year2019/day16.rs) | 1956 |
| 17 | [Set and Forget](https://adventofcode.com/2019/day/17) | [Source](src/year2019/day17.rs) | 338 |
| 18 | [Many-Worlds Interpretation](https://adventofcode.com/2019/day/18) | [Source](src/year2019/day18.rs) | 1157 |
| 19 | [Tractor Beam](https://adventofcode.com/2019/day/19) | [Source](src/year2019/day19.rs) | 688 |
| 20 | [Donut Maze](https://adventofcode.com/2019/day/20) | [Source](src/year2019/day20.rs) | 189 |
| 21 | [Springdroid Adventure](https://adventofcode.com/2019/day/21) | [Source](src/year2019/day21.rs) | 1785 |
| 22 | [Slam Shuffle](https://adventofcode.com/2019/day/22) | [Source](src/year2019/day22.rs) | 12 |
| 23 | [Category Six](https://adventofcode.com/2019/day/23) | [Source](src/year2019/day23.rs) | 670 |
| 24 | [Planet of Discord](https://adventofcode.com/2019/day/24) | [Source](src/year2019/day24.rs) | 232 |
| 25 | [Cryostasis](https://adventofcode.com/2019/day/25) | [Source](src/year2019/day25.rs) | 2047 |

## 2018

![pie-2018]

| Day | Problem | Solution | Benchmark (μs) |
| --- | --- | --- | --: |
| 1 | [Chronal Calibration](https://adventofcode.com/2018/day/1) | [Source](src/year2018/day01.rs) | 16 |
| 2 | [Inventory Management System](https://adventofcode.com/2018/day/2) | [Source](src/year2018/day02.rs) | 78 |
| 3 | [No Matter How You Slice It](https://adventofcode.com/2018/day/3) | [Source](src/year2018/day03.rs) | 55 |
| 4 | [Repose Record](https://adventofcode.com/2018/day/4) | [Source](src/year2018/day04.rs) | 46 |
| 5 | [Alchemical Reduction](https://adventofcode.com/2018/day/5) | [Source](src/year2018/day05.rs) | 390 |
| 6 | [Chronal Coordinates](https://adventofcode.com/2018/day/6) | [Source](src/year2018/day06.rs) | 41 |
| 7 | [The Sum of Its Parts](https://adventofcode.com/2018/day/7) | [Source](src/year2018/day07.rs) | 8 |
| 8 | [Memory Maneuver](https://adventofcode.com/2018/day/8) | [Source](src/year2018/day08.rs) | 24 |
| 9 | [Marble Mania](https://adventofcode.com/2018/day/9) | [Source](src/year2018/day09.rs) | 909 |
| 10 | [The Stars Align](https://adventofcode.com/2018/day/10) | [Source](src/year2018/day10.rs) | 11 |
| 11 | [Chronal Charge](https://adventofcode.com/2018/day/11) | [Source](src/year2018/day11.rs) | 1156 |
| 12 | [Subterranean Sustainability](https://adventofcode.com/2018/day/12) | [Source](src/year2018/day12.rs) | 77 |
| 13 | [Mine Cart Madness](https://adventofcode.com/2018/day/13) | [Source](src/year2018/day13.rs) | 382 |
| 14 | [Chocolate Charts](https://adventofcode.com/2018/day/14) | [Source](src/year2018/day14.rs) | 24000 |
| 15 | [Beverage Bandits](https://adventofcode.com/2018/day/15) | [Source](src/year2018/day15.rs) | 583 |
| 16 | [Chronal Classification](https://adventofcode.com/2018/day/16) | [Source](src/year2018/day16.rs) | 37 |
| 17 | [Reservoir Research ](https://adventofcode.com/2018/day/17) | [Source](src/year2018/day17.rs) | 151 |
| 18 | [Settlers of The North Pole](https://adventofcode.com/2018/day/18) | [Source](src/year2018/day18.rs) | 384 |
| 19 | [Go With The Flow](https://adventofcode.com/2018/day/19) | [Source](src/year2018/day19.rs) | 1 |
| 20 | [A Regular Map](https://adventofcode.com/2018/day/20) | [Source](src/year2018/day20.rs) | 36 |
| 21 | [Chronal Conversion](https://adventofcode.com/2018/day/21) | [Source](src/year2018/day21.rs) | 66 |
| 22 | [Mode Maze](https://adventofcode.com/2018/day/22) | [Source](src/year2018/day22.rs) | 3396 |
| 23 | [Experimental Emergency Teleportation](https://adventofcode.com/2018/day/23) | [Source](src/year2018/day23.rs) | 506 |
| 24 | [Immune System Simulator 20XX](https://adventofcode.com/2018/day/24) | [Source](src/year2018/day24.rs) | 2056 |
| 25 | [Four-Dimensional Adventure](https://adventofcode.com/2018/day/25) | [Source](src/year2018/day25.rs) | 323 |

## 2017

![pie-2017]

| Day | Problem | Solution | Benchmark (μs) |
| --- | --- | --- | --: |
| 1 | [Inverse Captcha](https://adventofcode.com/2017/day/1) | [Source](src/year2017/day01.rs) | 1 |
| 2 | [Corruption Checksum](https://adventofcode.com/2017/day/2) | [Source](src/year2017/day02.rs) | 2 |
| 3 | [Spiral Memory](https://adventofcode.com/2017/day/3) | [Source](src/year2017/day03.rs) | 2 |
| 4 | [High-Entropy Passphrases](https://adventofcode.com/2017/day/4) | [Source](src/year2017/day04.rs) | 98 |
| 5 | [A Maze of Twisty Trampolines, All Alike](https://adventofcode.com/2017/day/5) | [Source](src/year2017/day05.rs) | 22000 |
| 6 | [Memory Reallocation](https://adventofcode.com/2017/day/6) | [Source](src/year2017/day06.rs) | 81 |
| 7 | [Recursive Circus](https://adventofcode.com/2017/day/7) | [Source](src/year2017/day07.rs) | 93 |
| 8 | [I Heard You Like Registers](https://adventofcode.com/2017/day/8) | [Source](src/year2017/day08.rs) | 47 |
| 9 | [Stream Processing](https://adventofcode.com/2017/day/9) | [Source](src/year2017/day09.rs) | 23 |
| 10 | [Knot Hash](https://adventofcode.com/2017/day/10) | [Source](src/year2017/day10.rs) | 66 |
| 11 | [Hex Ed](https://adventofcode.com/2017/day/11) | [Source](src/year2017/day11.rs) | 18 |
| 12 | [Digital Plumber](https://adventofcode.com/2017/day/12) | [Source](src/year2017/day12.rs) | 61 |
| 13 | [Packet Scanners](https://adventofcode.com/2017/day/13) | [Source](src/year2017/day13.rs) | 1 |
| 14 | [Disk Defragmentation](https://adventofcode.com/2017/day/14) | [Source](src/year2017/day14.rs) | 438 |
| 15 | [Dueling Generators](https://adventofcode.com/2017/day/15) | [Source](src/year2017/day15.rs) | 26000 |
| 16 | [Permutation Promenade](https://adventofcode.com/2017/day/16) | [Source](src/year2017/day16.rs) | 68 |
| 17 | [Spinlock](https://adventofcode.com/2017/day/17) | [Source](src/year2017/day17.rs) | 85 |
| 18 | [Duet](https://adventofcode.com/2017/day/18) | [Source](src/year2017/day18.rs) | 7 |
| 19 | [A Series of Tubes](https://adventofcode.com/2017/day/19) | [Source](src/year2017/day19.rs) | 19 |
| 20 | [Particle Swarm](https://adventofcode.com/2017/day/20) | [Source](src/year2017/day20.rs) | 245 |
| 21 | [Fractal Art](https://adventofcode.com/2017/day/21) | [Source](src/year2017/day21.rs) | 5 |
| 22 | [Sporifica Virus](https://adventofcode.com/2017/day/22) | [Source](src/year2017/day22.rs) | 36000 |
| 23 | [Coprocessor Conflagration](https://adventofcode.com/2017/day/23) | [Source](src/year2017/day23.rs) | 55 |
| 24 | [Electromagnetic Moat](https://adventofcode.com/2017/day/24) | [Source](src/year2017/day24.rs) | 275 |
| 25 | [The Halting Problem](https://adventofcode.com/2017/day/25) | [Source](src/year2017/day25.rs) | 3698 |

## 2016

![pie-2016]

| Day | Problem | Solution | Benchmark (μs) |
| --- | --- | --- | --: |
| 1 | [No Time for a Taxicab](https://adventofcode.com/2016/day/1) | [Source](src/year2016/day01.rs) | 3 |
| 2 | [Bathroom Security](https://adventofcode.com/2016/day/2) | [Source](src/year2016/day02.rs) | 29 |
| 3 | [Squares With Three Sides](https://adventofcode.com/2016/day/3) | [Source](src/year2016/day03.rs) | 24 |
| 4 | [Security Through Obscurity](https://adventofcode.com/2016/day/4) | [Source](src/year2016/day04.rs) | 79 |
| 5 | [How About a Nice Game of Chess?](https://adventofcode.com/2016/day/5) | [Source](src/year2016/day05.rs) | 37000 |
| 6 | [Signals and Noise](https://adventofcode.com/2016/day/6) | [Source](src/year2016/day06.rs) | 4 |
| 7 | [Internet Protocol Version 7](https://adventofcode.com/2016/day/7) | [Source](src/year2016/day07.rs) | 364 |
| 8 | [Two-Factor Authentication](https://adventofcode.com/2016/day/8) | [Source](src/year2016/day08.rs) | 9 |
| 9 | [Explosives in Cyberspace](https://adventofcode.com/2016/day/9) | [Source](src/year2016/day09.rs) | 6 |
| 10 | [Balance Bots](https://adventofcode.com/2016/day/10) | [Source](src/year2016/day10.rs) | 16 |
| 11 | [Radioisotope Thermoelectric Generators](https://adventofcode.com/2016/day/11) | [Source](src/year2016/day11.rs) | 719 |
| 12 | [Leonardo's Monorail](https://adventofcode.com/2016/day/12) | [Source](src/year2016/day12.rs) | 1 |
| 13 | [A Maze of Twisty Little Cubicles](https://adventofcode.com/2016/day/13) | [Source](src/year2016/day13.rs) | 3 |
| 14 | [One-Time Pad](https://adventofcode.com/2016/day/14) | [Source](src/year2016/day14.rs) | 77000 |
| 15 | [Timing is Everything](https://adventofcode.com/2016/day/15) | [Source](src/year2016/day15.rs) | 1 |
| 16 | [Dragon Checksum](https://adventofcode.com/2016/day/16) | [Source](src/year2016/day16.rs) | 1 |
| 17 | [Two Steps Forward](https://adventofcode.com/2016/day/17) | [Source](src/year2016/day17.rs) | 3858 |
| 18 | [Like a Rogue](https://adventofcode.com/2016/day/18) | [Source](src/year2016/day18.rs) | 728 |
| 19 | [An Elephant Named Joseph](https://adventofcode.com/2016/day/19) | [Source](src/year2016/day19.rs) | 1 |
| 20 | [Firewall Rules](https://adventofcode.com/2016/day/20) | [Source](src/year2016/day20.rs) | 21 |
| 21 | [Scrambled Letters and Hash](https://adventofcode.com/2016/day/21) | [Source](src/year2016/day21.rs) | 10 |
| 22 | [Grid Computing](https://adventofcode.com/2016/day/22) | [Source](src/year2016/day22.rs) | 28 |
| 23 | [Safe Cracking](https://adventofcode.com/2016/day/23) | [Source](src/year2016/day23.rs) | 1 |
| 24 | [Air Duct Spelunking](https://adventofcode.com/2016/day/24) | [Source](src/year2016/day24.rs) | 337 |
| 25 | [Clock Signal](https://adventofcode.com/2016/day/25) | [Source](src/year2016/day25.rs) | 1 |

## 2015

![pie-2015]

| Day | Problem | Solution | Benchmark (μs) |
| --- | --- | --- | --: |
| 1 | [Not Quite Lisp](https://adventofcode.com/2015/day/1) | [Source](src/year2015/day01.rs) | 2 |
| 2 | [I Was Told There Would Be No Math](https://adventofcode.com/2015/day/2) | [Source](src/year2015/day02.rs) | 8 |
| 3 | [Perfectly Spherical Houses in a Vacuum](https://adventofcode.com/2015/day/3) | [Source](src/year2015/day03.rs) | 95 |
| 4 | [The Ideal Stocking Stuffer](https://adventofcode.com/2015/day/4) | [Source](src/year2015/day04.rs) | 14000 |
| 5 | [Doesn't He Have Intern-Elves For This?](https://adventofcode.com/2015/day/5) | [Source](src/year2015/day05.rs) | 38 |
| 6 | [Probably a Fire Hazard](https://adventofcode.com/2015/day/6) | [Source](src/year2015/day06.rs) | 6572 |
| 7 | [Some Assembly Required](https://adventofcode.com/2015/day/7) | [Source](src/year2015/day07.rs) | 27 |
| 8 | [Matchsticks](https://adventofcode.com/2015/day/8) | [Source](src/year2015/day08.rs) | 12 |
| 9 | [All in a Single Night](https://adventofcode.com/2015/day/9) | [Source](src/year2015/day09.rs) | 34 |
| 10 | [Elves Look, Elves Say](https://adventofcode.com/2015/day/10) | [Source](src/year2015/day10.rs) | 15 |
| 11 | [Corporate Policy](https://adventofcode.com/2015/day/11) | [Source](src/year2015/day11.rs) | 1 |
| 12 | [JSAbacusFramework.io](https://adventofcode.com/2015/day/12) | [Source](src/year2015/day12.rs) | 83 |
| 13 | [Knights of the Dinner Table](https://adventofcode.com/2015/day/13) | [Source](src/year2015/day13.rs) | 37 |
| 14 | [Reindeer Olympics](https://adventofcode.com/2015/day/14) | [Source](src/year2015/day14.rs) | 28 |
| 15 | [Science for Hungry People](https://adventofcode.com/2015/day/15) | [Source](src/year2015/day15.rs) | 53 |
| 16 | [Aunt Sue](https://adventofcode.com/2015/day/16) | [Source](src/year2015/day16.rs) | 20 |
| 17 | [No Such Thing as Too Much](https://adventofcode.com/2015/day/17) | [Source](src/year2015/day17.rs) | 45 |
| 18 | [Like a GIF For Your Yard](https://adventofcode.com/2015/day/18) | [Source](src/year2015/day18.rs) | 154 |
| 19 | [Medicine for Rudolph](https://adventofcode.com/2015/day/19) | [Source](src/year2015/day19.rs) | 187 |
| 20 | [Infinite Elves and Infinite Houses](https://adventofcode.com/2015/day/20) | [Source](src/year2015/day20.rs) | 1667 |
| 21 | [RPG Simulator 20XX](https://adventofcode.com/2015/day/21) | [Source](src/year2015/day21.rs) | 2 |
| 22 | [Wizard Simulator 20XX](https://adventofcode.com/2015/day/22) | [Source](src/year2015/day22.rs) | 235 |
| 23 | [Opening the Turing Lock](https://adventofcode.com/2015/day/23) | [Source](src/year2015/day23.rs) | 6 |
| 24 | [It Hangs in the Balance](https://adventofcode.com/2015/day/24) | [Source](src/year2015/day24.rs) | 380 |
| 25 | [Let It Snow](https://adventofcode.com/2015/day/25) | [Source](src/year2015/day25.rs) | 1 |

[checks-badge]: https://img.shields.io/github/actions/workflow/status/maneatingape/advent-of-code-rust/checks.yml?label=checks
[checks-link]: https://github.com/maneatingape/advent-of-code-rust/actions/workflows/checks.yml
[docs-badge]: https://img.shields.io/github/actions/workflow/status/maneatingape/advent-of-code-rust/docs.yml?color=blue&label=docs
[docs-link]: https://maneatingape.github.io/advent-of-code-rust/aoc/
[Advent of Code]: https://adventofcode.com
[apple-link]: https://en.wikipedia.org/wiki/Apple_M2
[intel-link]: https://ark.intel.com/content/www/us/en/ark/products/50067/intel-core-i72720qm-processor-6m-cache-up-to-3-30-ghz.html
[pie-all]: docs/pie-all.svg
[pie-2024]: docs/pie-2024.svg
[pie-2023]: docs/pie-2023.svg
[pie-2022]: docs/pie-2022.svg
[pie-2021]: docs/pie-2021.svg
[pie-2020]: docs/pie-2020.svg
[pie-2019]: docs/pie-2019.svg
[pie-2018]: docs/pie-2018.svg
[pie-2017]: docs/pie-2017.svg
[pie-2016]: docs/pie-2016.svg
[pie-2015]: docs/pie-2015.svg
