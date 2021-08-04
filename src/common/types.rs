pub type CalcErr = (String, Option<usize>);
pub type CalcResult<T> = Result<T, CalcErr>;