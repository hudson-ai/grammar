use std::fmt;

#[derive(Debug)]
enum Symbol {
    Terminal(char),
    Nonterminal,
}

struct Rule {
    lhs: Symbol,
    rhs: Vec<Symbol>,
}

struct EarleyItem {
    pos: usize,
    start: usize,
    rule: Rule,
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} ->", self.lhs)?;
        for item in self.rhs.iter() {
            write!(f, " {:?}", item)?;
        }
        write!(f, "")
    }
}

impl fmt::Display for EarleyItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let rule = &self.rule;
        write!(f, "{:?} ->", rule.lhs)?;
        for (i, item) in rule.rhs.iter().enumerate() {
            if i == self.pos {
                write!(f, " •")?;
            };
            write!(f, " {:?}", item)?;
        }
        if self.pos == rule.rhs.len() {
            write!(f, " •")?;
        }
        write!(f, " ({})", self.start)
    }
}

pub fn main() {
    let rule = Rule {
        lhs: Symbol::Nonterminal,
        rhs: vec![Symbol::Terminal('a'), Symbol::Terminal('b')],
    };
    println!("{}", rule);

    let ear = EarleyItem {
        pos: 1,
        start: 1,
        rule,
    };
    println!("{}", ear);
}
