/** Enumeration to characterize the distribution of values in a given column */

#[derive(Copy, Clone)]
pub(crate) enum Dist {
    Uniform,
    Zipf50,
    Zipf75,
    Zipf95,
    Zipf99,
    Serial,
    Cyclic,
}