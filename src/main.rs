use std::fmt;

#[derive(Debug)]
enum Symbol {
    Terminal(char),
    Nonterminal,
}

struct EarleyItem {
    pos: usize,
    start: usize,
    lhs: Symbol,
    rhs: Vec<Symbol>,
}

impl fmt::Display for EarleyItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} ->", self.lhs)?;
        for (i, item) in self.rhs.iter().enumerate() {
            if i == self.pos {
                write!(f, " •")?;
            };
            write!(f, " {:?}", item)?;
        }
        if self.pos == self.rhs.len() {
            write!(f, " •")?;
        }
        write!(f, " ({})", self.start)
    }
}

fn main() {
    let ear = EarleyItem {
        pos: 1,
        start: 1,
        lhs: Symbol::Nonterminal,
        rhs: vec![Symbol::Terminal('a'), Symbol::Terminal('b')],
    };
    println!("{}", ear)
}
