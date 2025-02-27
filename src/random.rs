use rand::{Rng, distributions::Uniform};

/// Generates a random vector of integers within a specified range.
/// # Arguments
/// * `floor` - The minimum value (inclusive) for the generated integers.
/// * `ceil` - The maximum value (exclusive) for the generated integers.
/// * `n` - The number of integers to generate.
/// # Returns
/// A `Vec<usize>` containing `n` random integers in the range `[floor, ceil)`.

pub fn gen_random_vector(floor: usize, ceil: usize, n: usize) -> Vec<usize> {
    let range = Uniform::new(floor, ceil);
    rand::thread_rng().sample_iter(&range).take(n).collect()
}
