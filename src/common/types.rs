pub type CalcErr = (String, Option<u16>);
pub type CalcResult<T> = Result<T, CalcErr>;