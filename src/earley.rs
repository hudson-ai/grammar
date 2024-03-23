use indexmap::IndexSet;
use std::fmt;

#[derive(PartialEq, Eq, Hash, Clone)]
enum Symbol {
    Terminal(Vec<char>),
    Nonterminal { name: String },
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Production {
    nonterminal: Symbol, //TODO: type-narrow??
    symbols: Vec<Symbol>,
}

#[derive(Clone)]
struct Grammar {
    productions: Vec<Production>,
}

#[derive(PartialEq, Eq, Hash)]
struct EarleyItem {
    production: Production,
    pos: usize,
    start: usize,
}

struct StateSet(IndexSet<EarleyItem>);
impl FromIterator<EarleyItem> for StateSet {
    fn from_iter<T: IntoIterator<Item = EarleyItem>>(iter: T) -> Self {
        StateSet(IndexSet::<EarleyItem>::from_iter(iter))
    }
}

struct EarleyParser {
    grammar: Grammar,
    pos: usize,
    state_sets: Vec<StateSet>,
}

impl EarleyItem {
    fn from_production(production: Production, start: usize) -> Self {
        EarleyItem {
            production,
            pos: 0,
            start,
        }
    }

    fn next_symbol(&self) -> Option<&Symbol> {
        self.production.symbols.get(self.pos)
    }
}

impl fmt::Display for StateSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for item in &self.0 {
            writeln!(f, "{}", item)?;
        }
        write!(f, "")
    }
}

impl fmt::Display for EarleyParser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, state_set) in self.state_sets.iter().enumerate() {
            writeln!(f, "=== {} ===", i)?;
            writeln!(f, "{}", state_set)?;
        }
        write!(f, "")
    }
}

impl From<Grammar> for EarleyParser {
    fn from(grammar: Grammar) -> Self {
        let start = 0_usize;
        let pos = 0_usize;
        let state_set: StateSet = grammar
            .productions
            .iter()
            .map(|production| EarleyItem::from_production(production.clone(), start))
            .collect();
        EarleyParser {
            grammar,
            pos,
            state_sets: vec![state_set],
        }
    }
}

impl fmt::Display for Grammar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for prod in self.productions.iter() {
            writeln!(f, "{}", prod)?;
        }
        write!(f, "")
    }
}

impl fmt::Display for Production {
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
                match &chars[..] {
                    [char] => write!(f, "'{}'", char),
                    _ => write!(f, "[{}]", String::from_iter(chars))
                }
            }
            Symbol::Nonterminal { name, .. } => write!(f, "{}", name),
        }
    }
}

impl fmt::Display for EarleyItem {
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
                nonterminal: sum.clone(),
                symbols: vec![sum.clone(), pm.clone(), product.clone()],
            },
            Production {
                nonterminal: sum.clone(),
                symbols: vec![product.clone()],
            },
            Production {
                nonterminal: product.clone(),
                symbols: vec![product.clone(), md.clone(), factor.clone()],
            },
            Production {
                nonterminal: product.clone(),
                symbols: vec![factor.clone()],
            },
            Production {
                nonterminal: factor.clone(),
                symbols: vec![lparen.clone(), sum.clone(), rparen.clone()],
            },
            Production {
                nonterminal: factor.clone(),
                symbols: vec![number.clone()],
            },
            Production {
                nonterminal: number.clone(),
                symbols: vec![digit.clone(), number.clone()],
            },
            Production {
                nonterminal: number.clone(),
                symbols: vec![digit.clone()],
            },
        ],
    };
    // print!("{}", grammar);
    let parser = EarleyParser::from(grammar.clone());
    // let s = parser.state_sets.first().unwrap();
    print!("{}", parser)
}
