use crate::DomainError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Result<T> {
    Ok(T),
    Err(Vec<DomainError>),
}

impl<T> Result<T> {
    pub fn ok(value: T) -> Self {
        Self::Ok(value)
    }

    pub fn err(message: impl Into<String>) -> Self {
        Self::Err(vec![DomainError::new(message)])
    }

    pub fn fail(errors: Vec<DomainError>) -> Self {
        Self::Err(errors)
    }

    pub fn is_ok(&self) -> bool {
        matches!(self, Self::Ok(_))
    }

    pub fn is_err(&self) -> bool {
        matches!(self, Self::Err(_))
    }

    pub fn unwrap(self) -> T {
        match self {
            Self::Ok(value) => value,
            Self::Err(errors) => panic!("called Result::unwrap on Err: {errors:?}"),
        }
    }

    pub fn errors(&self) -> &[DomainError] {
        match self {
            Self::Ok(_) => &[],
            Self::Err(errors) => errors.as_slice(),
        }
    }
}

impl<T> Result<T> {
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Result<U> {
        match self {
            Result::Ok(v) => Result::Ok(f(v)),
            Result::Err(errors) => Result::Err(errors),
        }
    }
}

pub fn combine_errors(results: &[Result<()>]) -> Vec<DomainError> {
    results
        .iter()
        .flat_map(|r| match r {
            Result::Err(errors) => errors.clone(),
            Result::Ok(_) => Vec::new(),
        })
        .collect()
}

pub fn combine2<T1, T2>(r1: Result<T1>, r2: Result<T2>) -> Result<(T1, T2)> {
    let mut errors = Vec::new();
    let v1 = match r1 {
        Result::Ok(v) => Some(v),
        Result::Err(e) => {
            errors.extend(e);
            None
        }
    };
    let v2 = match r2 {
        Result::Ok(v) => Some(v),
        Result::Err(e) => {
            errors.extend(e);
            None
        }
    };
    if !errors.is_empty() {
        return Result::Err(errors);
    }
    Result::Ok((v1.unwrap(), v2.unwrap()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_ok() {
        let r: Result<i32> = Result::ok(2);
        let mapped = r.map(|n| n + 1);
        assert!(matches!(mapped, Result::Ok(3)));
    }

    #[test]
    fn combine_aggregates_errors() {
        let r1: Result<()> = Result::err("e1");
        let r2: Result<()> = Result::err("e2");
        let errors = combine_errors(&[r1, r2]);
        assert_eq!(errors.len(), 2);
    }

    #[test]
    fn err_fail_and_queries() {
        let err = Result::<i32>::err("bad");
        assert!(err.is_err());
        assert!(!err.is_ok());
        assert_eq!(err.errors().len(), 1);

        let fail = Result::<i32>::fail(vec![DomainError::new("a"), DomainError::new("b")]);
        assert_eq!(fail.errors().len(), 2);
    }

    #[test]
    fn ok_has_no_errors() {
        let ok = Result::ok(42);
        assert!(ok.is_ok());
        assert!(!ok.is_err());
        assert!(ok.errors().is_empty());
    }

    #[test]
    fn map_propagates_err() {
        let err: Result<i32> = Result::err("fail");
        assert!(err.map(|n| n + 1).is_err());
    }

    #[test]
    fn combine2_ok() {
        let combined = combine2(Result::ok(1), Result::ok(2));
        assert_eq!(combined, Result::Ok((1, 2)));
    }

    #[test]
    fn combine2_merges_errors() {
        let combined = combine2(Result::<i32>::err("e1"), Result::<i32>::err("e2"));
        assert_eq!(combined.errors().len(), 2);
    }

    #[test]
    fn combine_errors_ignores_ok() {
        let errors = combine_errors(&[Result::ok(()), Result::err("only")]);
        assert_eq!(errors.len(), 1);
    }
}
