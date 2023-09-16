struct SirParameters {
    beta: f64,      // base level rate of infection
    gamma: f64,     // base level rate of recovery
}

enum PopulationGroup {
    Susceptible,
    Infected,
    Recovered,
}

fn next_state(group: &PopulationGroup, params: &SirParameters, rand: f64) -> PopulationGroup {
    match group {
        PopulationGroup::Susceptible => {
            if rand < params.beta {
                PopulationGroup::Infected
            } else {
                PopulationGroup::Susceptible
            }
        }
        PopulationGroup::Infected => {
            if rand < params.gamma {
                PopulationGroup::Recovered
            } else {
                PopulationGroup::Infected
            }
        }
        _ => PopulationGroup::Recovered,
    }
}


fn simulate_day(population: &mut Vec<PopulationGroup>, params: &SirParameters) {
    for person in population.iter_mut() {
        let rand: f64 = rand::random();
        *person = next_state(&person, &params, rand);
    }
}

fn count_groups(population: &[PopulationGroup]) -> (usize, usize, usize) {
    let mut susceptible = 0;
    let mut infected = 0;
    let mut recovered = 0;

    for person in population.iter() {
        match person {
            PopulationGroup::Susceptible => susceptible += 1,
            PopulationGroup::Infected => infected += 1,
            PopulationGroup::Recovered => recovered += 1,
        }
    }

    (susceptible, infected, recovered)
}

fn main() {
    let params = SirParameters {
        beta: 0.05,
        gamma: 0.01,
    };

    let mut population: Vec<PopulationGroup> = vec![];

    for _ in 0..990 {
        population.push(PopulationGroup::Susceptible);
    }

    // infected population of 10 people
    for _ in 0..10 {
        population.push(PopulationGroup::Infected);
    }

    for day in 1..=365 {
        simulate_day(&mut population, &params);

        let (s, i, r) = count_groups(&population);

        println!("Day {}: S={},  I={}, R={}", day, s, i, r);
    }
}
