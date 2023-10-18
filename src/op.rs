pub trait Operation: std::fmt::Debug {
    fn apply(&self, args: &[bool]) -> bool;
}

#[derive(Debug)]
pub struct Negate;
impl Operation for Negate {
    #[inline]
    fn apply(&self, args: &[bool]) -> bool {
        assert_eq!(args.len(), 1);
        !args[0]
    }
}
#[derive(Debug)]
pub struct And;
impl Operation for And {
    #[inline]
    fn apply(&self, args: &[bool]) -> bool {
        args.iter().all(|&b| b)
    }
}
#[derive(Debug)]
pub struct Or;
impl Operation for Or {
    #[inline]
    fn apply(&self, args: &[bool]) -> bool {
        args.iter().any(|&b| b)
    }
}
#[derive(Debug)]
pub struct Implies;
impl Operation for Implies {
    #[inline]
    fn apply(&self, args: &[bool]) -> bool {
        assert!(args.len() == 2);
        !args[0] || args[1]
    }
}