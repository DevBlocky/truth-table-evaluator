mod op;

#[derive(Debug)]
struct VariableMap {
    vstates: Vec<(char, bool)>,
}
impl VariableMap {
    fn from_binary(variables: &[char], n: usize) -> VariableMap {
        let mut vstates = Vec::new();
        for (i, var) in variables.iter().enumerate() {
            let set = (n & (1 << i)) != 0;
            vstates.push((*var, set))
        }
        VariableMap { vstates }
    }

    fn all(variables: &[char]) -> Vec<VariableMap> {
        (0..(1 << variables.len()))
            .map(|n| VariableMap::from_binary(variables, n))
            .collect()
    }

    fn check(&self, p: char) -> bool {
        for (var, state) in &self.vstates {
            if *var == p {
                return *state;
            }
        }
        false
    }
}

#[derive(Debug)]
enum Expression {
    Constant(bool),
    Variable(char),

    Expr {
        args: Vec<Expression>,
        op: Box<dyn op::Operation>,
    },
}
impl Expression {
    fn eval(&self, vmap: &VariableMap) -> bool {
        match self {
            Expression::Constant(b) => *b,
            Expression::Variable(p) => vmap.check(*p),

            Expression::Expr { args, op } => {
                let args = args.iter().map(|expr| expr.eval(vmap)).collect::<Vec<_>>();
                op.apply(&args)
            }
        }
    }
    fn search_variables(&self) -> Vec<char> {
        match self {
            Expression::Constant(_) => Vec::new(),
            Expression::Variable(c) => vec![*c],

            Expression::Expr { args, .. } => {
                let mut vars = Vec::new();
                for arg in args {
                    vars.extend(arg.search_variables());
                }
                vars.sort_unstable();
                vars.dedup();
                vars
            }
        }
    }
}
impl From<char> for Expression {
    fn from(c: char) -> Expression {
        Expression::Variable(c)
    }
}
impl From<bool> for Expression {
    fn from(b: bool) -> Expression {
        Expression::Constant(b)
    }
}
impl From<&str> for Expression {
    fn from(s: &str) -> Expression {
        let mut chars = s.chars();
        let c = chars.next().unwrap();
        match c {
            '!' => Expression::Expr {
                args: vec![chars.next().unwrap().into()],
                op: Box::new(op::Negate),
            },
            c => c.into(),
        }
    }
}

macro_rules! impl_helper {
    {$op:expr, $fn_name:ident, $($arg:ident)+} => {
        fn $fn_name<$($arg),+>($($arg: $arg),+) -> Expression
        where
            $($arg: Into<Expression>),+
        {
            Expression::Expr {
                args: vec![ $($arg.into()),+ ],
                op: Box::new($op)
            }
        }
    }
}
impl_helper! {op::Negate, not, a}
impl_helper! {op::Or, or, a b}
impl_helper! {op::Or, or3, a b c}
impl_helper! {op::Or, or4, a b c d}
impl_helper! {op::And, and, a b}
impl_helper! {op::And, and3, a b c}
impl_helper! {op::And, and5, a b c d e}
impl_helper! {op::Implies, implies, a b}

fn main() {
    // define expressions
    let exprs = vec![
        or(
            implies('t', and(not('q'), 't')),
            and3('q', not('r'), or('p', and3(not('p'), 's', 't'))),
        ),
        or3(
            or(not('t'), not('q')),
            and3(not('r'), 'q', 'p'),
            and5(not('r'), 'q', not('p'), 's', 't'),
        ),
        and3(
            or3("!t", "!q", "q"),
            or3("!t", "!q", "!r"),
            or4("!t", "!q", "p", and3("!p", "s", "t")),
        ),
        and(
            or3("!t", "!q", "!r"),
            or4("!t", "!q", "p", "s"),
        )
    ];

    // setup table
    let vars = exprs[0].search_variables();
    for v in &vars {
        print!("{} ", v);
    }
    for i in 0..exprs.len() {
        print!(" e{}", i);
    }
    println!();

    // create variable maps for all possible combinations of variables
    let var_maps = VariableMap::all(&vars);

    for map in &var_maps {
        for v in &vars {
            print!("{} ", map.check(*v) as i32);
        }

        // record all evaluations for the expressions
        let mut evaluations = vec![];
        for expr in &exprs {
            let eval = expr.eval(map);
            evaluations.push(eval);
            print!("  {}", eval as i32);
        }

        // if all evaluations are not the same, print an arrow to highlight inconsistency
        evaluations.dedup();
        if evaluations.len() == 1 {
            println!();
        } else {
            println!(" <----");
        }
    }
}
