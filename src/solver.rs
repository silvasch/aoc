pub trait Solver<O> {
    fn solve(&self, input: &str) -> Result<O, String>;
}
