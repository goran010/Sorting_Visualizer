use rand::{distributions::Uniform, Rng};

/// Generates a random vector of integers within a specified range.
/// 
/// # Arguments
/// * `floor` - The minimum value (inclusive) for the generated integers.
/// * `ceil` - The maximum value (exclusive) for the generated integers.
/// * `n` - The number of integers to generate.
/// 
/// # Returns
/// A `Vec<usize>` containing `n` random integers in the range `[floor, ceil)`.
/// 
/// # Examples
/// ```
/// let random_vector = gen_random_vector(1, 10, 5);
/// println!("{:?}", random_vector); // Outputs a vector with 5 random numbers between 1 and 9.
/// ```
pub fn gen_random_vector(floor: usize, ceil: usize, n: usize) -> Vec<usize> {
    let range = Uniform::new(floor, ceil);
    rand::thread_rng().sample_iter(&range).take(n).collect()
}