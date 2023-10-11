use std::collections::HashMap;

mod op;

#[derive(Debug)]
struct VariableMap {
    map: HashMap<char, bool>,
}
impl VariableMap {
    fn from_binary(variables: &[char], n: usize) -> VariableMap {
        let mut map = HashMap::new();
        for (i, var) in variables.iter().enumerate() {
            let set = (n & (1 << i)) != 0;
            map.insert(*var, set);
        }
        VariableMap { map }
    }

    fn all(variables: &[char]) -> Vec<VariableMap> {
        (0..(1 << variables.len()))
            .map(|n| VariableMap::from_binary(variables, n))
            .collect()
    }

    fn check(&self, p: char) -> bool {
        assert!(self.map.contains_key(&p));
        self.map[&p]
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
                vars.dedup();
                vars
            }
        }
    }
}

/// HELPER SHIT ///
impl Into<Expression> for bool {
    fn into(self) -> Expression {
        Expression::Constant(self)
    }
}
impl Into<Expression> for char {
    fn into(self) -> Expression {
        Expression::Variable(self)
    }
}

fn not<A1: Into<Expression>>(a: A1) -> Expression {
    Expression::Expr {
        args: vec![a.into()],
        op: Box::new(op::Negate),
    }
}
fn and<A1: Into<Expression>, A2: Into<Expression>>(a: A1, b: A2) -> Expression {
    Expression::Expr {
        args: vec![a.into(), b.into()],
        op: Box::new(op::And),
    }
}
fn or<A1: Into<Expression>, A2: Into<Expression>>(a: A1, b: A2) -> Expression {
    Expression::Expr {
        args: vec![a.into(), b.into()],
        op: Box::new(op::Or),
    }
}
fn implies<A1: Into<Expression>, A2: Into<Expression>>(a: A1, b: A2) -> Expression {
    Expression::Expr {
        args: vec![a.into(), b.into()],
        op: Box::new(op::Implies),
    }
}

fn main() {
    // define expressions
    let exprs = vec![
        implies(and(implies('p', 'r'), 'q'), not('t')),
        not(and(or(not('p'), 'r'), and('q', 't'))),
        or(and('p', not('r')), or(not('q'), not('t'))),
        and(or('p', or(not('q'), not('t'))), or(not('r'), or(not('q'), not('t')))),
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
