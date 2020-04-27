extern crate rand;
use std::str;
use rand::Rng;
use std::cmp::Ordering;
use itertools::Itertools;

const TARGET_STR : &str = "I WAS DISCOVERED BY A GENETIC ALGORITHM";
const RANDOM_STR_MAX_LEN : usize = 50;
const MUTATION_RATE : usize = 80;
const ALPHABET : &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const MUTATION_STEPS : usize = 2;
const PENALTY_FACTOR : u8 = 100;
const INITIAL_GENERATION_SIZE : u32 = 1000;
const GENERATIONS : u16 = 1000;
const ALLOWED_TO_BREED : u8 = 40;
const ALLOWED_SURVIVORS : u8 = 40;
use std::iter::FromIterator;

#[derive(Eq,Clone)]
struct Organism {
  genome: String,
  fitness: Option<u32>
}

impl Ord for Organism {
  fn cmp(&self, other: &Self) -> Ordering {
      self.fitness.cmp(&other.fitness)
  }
}

impl PartialOrd for Organism {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
      Some(self.cmp(other))
  }
}

impl PartialEq for Organism {
  fn eq(&self, other: &Self) -> bool {
      self.fitness == other.fitness
  }
}

impl Organism {

  fn random() -> Organism {

    let mut string_letters = Vec::new();
    let mut rng = rand::thread_rng();
    let len = rng.gen_range(5, 55);

    for _ in 0..len {
      string_letters.push(ALPHABET.chars().nth(rng.gen_range(0, ALPHABET.len())).unwrap());
    }

    return Organism { genome: String::from_iter(string_letters), fitness: None };
  }

  fn test_fitness(&mut self) {
    
    let smallest = if self.genome.len() < TARGET_STR.len() { self.genome.len() } else { TARGET_STR.len() };
    let mut fitness_score : i32 = 0;
    let DIFFERENCE_IN_GENOME_LENGTHS = ((self.genome.len() as i32 - TARGET_STR.len() as i32) as i32).checked_abs().unwrap();
    fitness_score += DIFFERENCE_IN_GENOME_LENGTHS * 9999;

    for x in (0..smallest) {
      fitness_score += (
        (self.genome.chars().nth(x).unwrap() as i16 
        - TARGET_STR.chars().nth(x).unwrap() as i16) 
        as i32).pow(2);
    }

    self.fitness = Some(fitness_score as u32);

  }

  fn breed_with(&self, other_organism : Organism) -> Organism {

    let doubled = self.genome.chars().zip(other_organism.genome.chars());
    let mut child = "".to_string();

    let mut rng = rand::thread_rng();

    for (i, j) in doubled {
      if rng.gen_range(0, 1) == 0 {
        child.push_str(&i.to_string());
      } else {
        child.push_str(&j.to_string());
      }
    }

/*    let cutpoint_1 = rng.gen_range(0, self.genome.len()-1);
    let cutpoint_2 = rng.gen_range(cutpoint_1, self.genome.len()-1);  
    let cutpoint_3 = rng.gen_range(0, other_organism.genome.len()-1);
    let cutpoint_4 = rng.gen_range(cutpoint_3, (other_organism.genome.len()-1));
  
    let mut child = "".to_string();
    child.push_str(&self.genome[0..cutpoint_1].to_string());
    child.push_str(&other_organism.genome[cutpoint_3..cutpoint_4].to_string());
    child.push_str(&self.genome[cutpoint_2..self.genome.len()].to_string()); */
    return Organism { genome: child, fitness: None }; 
  
  }

  fn lookup_gene(gene : char) -> usize {
    for (idx, i) in ALPHABET.chars().enumerate() {
      if i == gene {
        return idx;
      }
    }

    return 0;

  }

  fn mutate(&mut self) {

    let mut rng = rand::thread_rng();
    let genes = self.genome.clone();
    let mut new_genome = "".to_string();

    for gene in genes.chars() {
      if rng.gen_range(0, MUTATION_RATE) == 2 {
        let BASE_PAIR_CHANGE : i8 = rng.gen_range(-3, 3);
        let mutated_value = (gene as i8 + BASE_PAIR_CHANGE as i8) as u8;
        let mutated = (mutated_value as u8) as char;
        new_genome.push_str(&mutated.to_string());    
      } else {
        new_genome.push_str(&gene.to_string());
      }
    }

    self.genome = new_genome; 
  }
}

struct Generation {
  organisms: Vec<Organism>
}


impl Generation {

  fn evaluate_fitness(&mut self) {
    for candidate in &mut self.organisms {
      candidate.test_fitness();
    }
  }

  fn apply_mutations(&mut self) {
    for organism in &mut self.organisms {
      organism.mutate();
    }
  }

  fn spawn_random_pool<'a>(poolSize : usize) -> Generation {

    let mut generation = Generation { organisms: Vec::new() };
  
    for _ in 0..poolSize {
      generation.organisms.push(Organism::random())
    }
  
    return generation;
  }

  fn top(&self, percentage : usize) -> Vec<Organism> {
    let organisms : Vec<Organism> = self.organisms.clone().into_iter().sorted().collect();
    return organisms[0..percentage].to_vec();
  }
}

fn main() {

    let mut count = 0;
    let mut generation = Generation::spawn_random_pool(INITIAL_GENERATION_SIZE as usize);
    let mut rng = rand::thread_rng();
    let mut running = true;

    while (running == true) {

      generation.evaluate_fitness();

      /* get survivors of generation */
      let best = generation.top(50);
      
      let mut next_generation = Generation { organisms: best.clone() };

    /* add children */
    for i in 0..20 {
      let first_parent = best.clone().into_iter().nth(i).unwrap();
      let second_parent = best.clone().into_iter().nth((i+1) % 20).unwrap();
      let mut child = first_parent.breed_with(second_parent);
      next_generation.organisms.push(child);
    }

    /* add some fresh genes for good measure */

    for i in 0..10 {
      let new_organism = Organism::random();
      next_generation.organisms.push(new_organism);
    }
 
    for candidate in generation.top(2) {
      print!("[GENERATION {:>4}]    {:>45}  [ERR: {:>4}]\r\n", count, candidate.genome, candidate.fitness.unwrap());
    }

    next_generation.apply_mutations();

    generation = next_generation;
    count += 1;

  }

}
