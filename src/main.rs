extern crate rand;

use rand::{thread_rng, sample, Rng};

fn main() {
    let gene_set = " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!.,'";
    let target = "I thought it was going to Laguardia.  It's going to JFK.";
    let best = get_best(get_fitness, display, target, gene_set, target.len());
    println!("{}", best);
}

// Returns the target string created trhough iterations.  When a more fit string is
// created, it is printed
fn get_best(get_fitness: fn(&String,&str) -> usize,
    display: fn(&String, &str),
    target: &str,
    gene_set: &str,
    length: usize) -> String {
        let mut best_parent = generate_parent(gene_set, length);
        let mut best_fitness = get_fitness(&best_parent, target);

// Randomly mutate and update the parent when the child is more fit than the parent.
// Print when string is recreated
        while best_fitness < length {
            let child = mutate_parent(&best_parent, gene_set);
            let fitness = get_fitness(&child, target);
            if fitness > best_fitness {
                best_fitness = fitness;
                best_parent = child;
                display(&best_parent, target);
            }
        }
        best_parent
    }

// Generates a string the length of the target.  All chars are randomly selected from the
// list of valid chars
fn generate_parent(gene_set: &str, length: usize) -> String {
    let mut rng = thread_rng();
    let sample = sample(&mut rng, gene_set.chars(), length);
    sample.into_iter().collect()
}

// Returns the number of correct characters in the string
fn get_fitness(candidate: &String, target: &str) -> usize {
    let different_count = target.chars()
        .zip(candidate.chars())
        .filter(|&(a, b)| a != b)
        .count();

    target.len() - different_count
}

// Randomly changes one of the characters in the parent to another valid char
fn mutate_parent(parent: &String, gene_set: &str) -> String {
    let mut rng = thread_rng();
    let gene_index = rng.gen::<usize>() % gene_set.len();
    let parent_index = rng.gen::<usize>() % parent.len();
    let mut candidate = String::with_capacity(parent.len());

    if parent_index > 0 {
        candidate.push_str(&parent[..parent_index]);
    }
    candidate.push_str(&gene_set[gene_index..(1+gene_index)]);
    if parent_index+1 < parent.len() {
        candidate.push_str(&parent[parent_index+1..]);
    }
    candidate
}

// Prints to screen
fn display(candidate: &String, target: &str) {
    println!("{}\t{}", candidate, get_fitness(&candidate, target));
}
