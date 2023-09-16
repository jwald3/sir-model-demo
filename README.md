# SIR Model Simulation in Rust
This project is a simple simulation of the SIR (Susceptible, Infected, Recovered) model for infectious diseases, written in Rust.

## Overview
The SIR model is a simple mathematical model that's often used to understand the dynamics of infectious diseases in populations. In this project, we simulate the progression of a disease through a population using the SIR model.

There are three main components in the SIR model:

* Susceptible: Individuals who are vulnerable to being infected.
* Infected: Individuals who have the disease and can spread it to susceptible individuals.
* Recovered: Individuals who have recovered from the disease and are now immune.
  
The simulation runs daily, and at each day:

1. Susceptible individuals have a chance (beta) to become infected.
2. Infected individuals have a chance (gamma) to recover.
   
## Structure
* SirParameters struct: This holds the parameters beta and gamma which represent the base level rate of infection and recovery, respectively.

* PopulationGroup enum: Represents the three possible states an individual can be in: Susceptible, Infected, or Recovered.

* next_state function: Given the current state of an individual and a random number, it determines the next state of the individual.

* simulate_day function: Simulates a day for the entire population by determining the next state for each individual.

* count_groups function: Counts the number of individuals in each state.

## Usage
1. Ensure you have Rust and Cargo installed.
2. Clone the repository.
3. Navigate to the project directory and run:
```
cargo run
```
4. This will simulate the progression of the disease for a year and print the number of Susceptible, Infected, and Recovered individuals for each day.

## Dependencies
This project depends on the rand crate for generating random numbers. Ensure it is added to your Cargo.toml file.

## Contributions
Feel free to fork this repository and make improvements or adaptations!
