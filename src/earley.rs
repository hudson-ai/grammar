use std::fmt;

enum Symbol {
    Terminal(Vec<char>),
    Nonterminal { name: String },
}

struct Production<'a> {
    nonterminal: &'a Symbol, //TODO: type-narrow??
    symbols: Vec<&'a Symbol>,
}

struct EarleyItem<'a> {
    pos: usize,
    start: usize,
    production: &'a Production<'a>,
}

struct Grammar<'a> {
    productions: Vec<Production<'a>>,
}

impl fmt::Display for Grammar<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for prod in self.productions.iter() {
            writeln!(f, "{}", prod)?;
        }
        write!(f, "")
    }
}

impl fmt::Display for Production<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ->", self.nonterminal)?;
        for symbol in self.symbols.iter() {
            write!(f, " {}", symbol)?;
        }
        write!(f, "")
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Symbol::Terminal(chars) => {
                if chars.len() > 1 {
                    write!(f, "[")?;
                } else {
                    write!(f, "'")?;
                }
                for ch in chars.iter() {
                    write!(f, "{}", ch)?;
                }
                if chars.len() > 1 {
                    write!(f, "]")
                } else {
                    write!(f, "'")
                }
            }
            Symbol::Nonterminal { name, .. } => write!(f, "{}", name),
        }
    }
}

impl fmt::Display for EarleyItem<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let prod = &self.production;
        write!(f, "{} ->", prod.nonterminal)?;
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
    let sum = Symbol::Nonterminal {
        name: "Sum".to_string(),
    };
    let product = Symbol::Nonterminal {
        name: "Product".to_string(),
    };
    let factor = Symbol::Nonterminal {
        name: "Factor".to_string(),
    };
    let number = Symbol::Nonterminal {
        name: "Number".to_string(),
    };
    let pm = Symbol::Terminal(vec!['+', '-']);
    let md = Symbol::Terminal(vec!['*', '/']);
    let lparen = Symbol::Terminal(vec!['(']);
    let rparen = Symbol::Terminal(vec![')']);
    let digit = Symbol::Terminal(('0'..='9').collect());
    let grammar = Grammar {
        productions: vec![
            Production {
                nonterminal: &sum,
                symbols: vec![&sum, &pm, &product],
            },
            Production {
                nonterminal: &sum,
                symbols: vec![&product],
            },
            Production {
                nonterminal: &product,
                symbols: vec![&product, &md, &factor],
            },
            Production {
                nonterminal: &product,
                symbols: vec![&factor],
            },
            Production {
                nonterminal: &factor,
                symbols: vec![&lparen, &sum, &rparen],
            },
            Production {
                nonterminal: &factor,
                symbols: vec![&number],
            },
            Production {
                nonterminal: &number,
                symbols: vec![&digit, &number],
            },
            Production {
                nonterminal: &number,
                symbols: vec![&digit],
            },
        ],
    };
    print!("{}", grammar);
}
