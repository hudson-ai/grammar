use std::fmt;

enum Symbol<'a> {
    Terminal(char),
    Nonterminal(&'a Nonterminal),
}

struct Nonterminal {
    name: String,
}

struct Production<'a> {
    nonterminal: &'a Nonterminal,
    symbols: Vec<Symbol<'a>>,
}

struct EarleyItem<'a> {
    pos: usize,
    start: usize,
    production: Production<'a>,
}

impl fmt::Display for Production<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ->", self.nonterminal.name)?; // TODO: implement nonterminal fmt::Display?
        for symbol in self.symbols.iter() {
            write!(f, " {}", symbol)?;
        }
        write!(f, "")
    }
}

impl fmt::Display for Symbol<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Symbol::Terminal(ch) => write!(f, "{}", ch),
            Symbol::Nonterminal(Nonterminal { name, .. }) => write!(f, "{}", name),
        }
    }
}

impl fmt::Display for EarleyItem<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let prod = &self.production;
        write!(f, "{} ->", prod.nonterminal.name)?;
        for (i, symbol) in prod.symbols.iter().enumerate() {
            if i == self.pos {
                write!(f, " •")?;
            };
            write!(f, " {}", symbol)?;
        }
        if self.pos == prod.symbols.len() {
            write!(f, " •")?;
        }
        write!(f, " ({})", self.start)
    }
}

pub fn main() {
    let number = Nonterminal {
        name: "Number".to_string(),
    };
    let prod = Production {
        nonterminal: &number,
        symbols: vec![Symbol::Nonterminal(&number), Symbol::Terminal('9')],
    };
    println!("{}", prod);
    let ear: EarleyItem = EarleyItem {
        pos: 1,
        start: 1,
        production: prod,
    };
    println!("{}", ear);
}
