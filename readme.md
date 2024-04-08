# Warhammer 40k Shooting Simulator

## Overview

This project is a shooting simulator for the popular tabletop game Warhammer 40k. It aims to simulate attacks between shooting units and targets to calculate average outcomes based on the game's shooting mechanics. The simulator takes into account various factors such as the number of shots, ballistic skill, strength, toughness, armour save, and armour penetration to determine the outcomes of shooting attacks. This software is intended to evolve into an API that allows Warhammer 40k players to run simulated attacks and better understand potential outcomes before making strategic decisions in their games.

## How It Works

The simulator executes a loop that simulates a large number of shooting attacks (100,000 iterations in the current setup) to generate statistically significant results. For each iteration, it follows these steps:

1. **Hit Phase:** Determines the number of successful hits based on the shooting unit's ballistic skill.
2. **Wound Phase:** Calculates the number of successful wounds from the hits, considering the strength of the attack against the target's toughness.
3. **Save Phase:** Determines the number of failed saves by the target, taking into account the target's armour save and the armour penetration of the attack.

After completing the simulations, the program calculates and displays the average number of successful hits, wounds, and failed saves across all iterations.

## Functions

- `main()`: Orchestrates the simulation loop and prints out the average results of hits, wounds, and failed saves.
- `d6()`: Simulates a roll of a six-sided dice, returning a random integer between 1 and 6.
- `hit(shots: i32, bs: i32) -> i32`: Calculates the number of successful hits based on the number of shots and the ballistic skill.
- `wound(hits: &i32, strength: i32, toughness: i32) -> i32`: Determines the number of wounds inflicted from the successful hits.
- `save(wounds: &i32, armour_save: i32, armour_penetration: i32) -> i32`: Calculates the number of wounds that are not saved by the target.
- `average(list: &[i32]) -> Option<i32>`: Computes the average value from a list of integers.

## Dependencies

To run this simulator, you will need the following:

- Rust programming language
- `rand` crate for generating random numbers

## Usage

1. Clone the repository to your local machine.
2. Ensure you have Rust and Cargo installed.
3. Build the project using `cargo build`.
4. Run the simulation using `cargo run`.

## Future Enhancements

- Transform the simulator into a fully functional API.
- Allow users to specify parameters such as unit types, weapon profiles, and target profiles.
- Implement more detailed game mechanics like cover and special abilities.

## Contributing

Contributions to extend functionality, improve simulation accuracy, or refactor the codebase are welcome. Please feel free to submit pull requests or open issues to discuss potential improvements.
