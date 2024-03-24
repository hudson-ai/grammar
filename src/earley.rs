use indexmap::IndexSet;
use std::fmt;

#[derive(PartialEq, Eq, Hash, Clone)]
enum Symbol {
    Terminal(Vec<char>),
    Nonterminal { name: String },
}
impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Symbol::Terminal(chars) => match &chars[..] {
                [char] => write!(f, "'{}'", char),
                _ => write!(f, "[{}]", String::from_iter(chars)),
            },
            Symbol::Nonterminal { name, .. } => write!(f, "{}", name),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Production {
    nonterminal: Symbol, //TODO: type-narrow??
    symbols: Vec<Symbol>,
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

#[derive(Clone)]
struct Grammar {
    productions: Vec<Production>,
}
impl fmt::Display for Grammar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for prod in self.productions.iter() {
            writeln!(f, "{}", prod)?;
        }
        write!(f, "")
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct EarleyItem {
    production: Production,
    pos: usize,
    start: usize,
}
impl EarleyItem {
    fn next_symbol(&self) -> Option<&Symbol> {
        self.production.symbols.get(self.pos)
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

#[derive(Clone)]
struct StateSet(IndexSet<EarleyItem>);
impl StateSet {
    fn new() -> StateSet {
        StateSet(IndexSet::<EarleyItem>::new())
    }
}
impl FromIterator<EarleyItem> for StateSet {
    fn from_iter<T: IntoIterator<Item = EarleyItem>>(iter: T) -> Self {
        StateSet(IndexSet::<EarleyItem>::from_iter(iter))
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

struct EarleyParser {
    grammar: Grammar,
    pos: usize,
    state_sets: Vec<StateSet>,
    chars: Vec<char>,
}
impl EarleyParser {
    fn consume_str(&mut self, input: String) {
        for ch in input.chars() {
            self.consume_char(ch);
        }
    }

    fn consume_char(&mut self, ch: char) {
        self.chars.push(ch);
        while let Some(next_state_set) = self.inner_loop(self.pos) {
            self.state_sets.push(next_state_set);
            self.pos += 1;
        }
    }
    fn inner_loop(&mut self, state_set_pos: usize) -> Option<StateSet> {
        // Remove state set and replace later
        if state_set_pos != self.state_sets.len() - 1 {
            todo!("Ok, time to figure out the logic for rewinding...")
        }
        let mut curr_state_set = self.state_sets.remove(state_set_pos);
        let mut next_state_set = StateSet::new();
        let mut i = 0_usize;
        while i < curr_state_set.0.len() {
            let item = curr_state_set.0[i].clone();
            match item.next_symbol() {
                Some(nonterminal @ Symbol::Nonterminal { .. }) => {
                    // predict
                    // The symbol at the right of the fat dot is non-terminal.
                    // We add the the corresponding rules to the current state set.
                    for production in self.grammar.productions.iter() {
                        if &production.nonterminal == nonterminal {
                            curr_state_set.0.insert(EarleyItem {
                                production: production.clone(),
                                start: state_set_pos,
                                pos: 0,
                            });
                        }
                    }
                }
                Some(Symbol::Terminal(chars)) => {
                    //scan
                    // The symbol at the right of the fat dot is terminal. We check if the input matches this symbol.
                    // If it does, we add this item (advanced one step) to the next state set.
                    if let Some(ch) = self.chars.get(state_set_pos) {
                        if chars.contains(ch) {
                            next_state_set.0.insert(EarleyItem {
                                production: item.production.clone(),
                                start: item.start,
                                pos: item.pos + 1,
                            });
                        }
                    }
                }
                None => {
                    // completion
                    // There is nothing at the right of the fat dot. This means we have a successful partial parse.
                    // We look for the parent items, and add them (advanced one step) to this state set.
                    for parent in self.state_sets[item.start].0.iter() {
                        match parent.next_symbol() {
                            Some(nonterminal) if nonterminal == &item.production.nonterminal => {
                                curr_state_set.0.insert(EarleyItem {
                                    production: parent.production.clone(),
                                    start: parent.start,
                                    pos: parent.pos + 1,
                                });
                            }
                            _ => {}
                        }
                    }
                }
            }
            i += 1;
        }
        self.state_sets.push(curr_state_set);

        if !next_state_set.0.is_empty() {
            Some(next_state_set)
        } else {
            None
        }
    }
}
impl From<Grammar> for EarleyParser {
    fn from(grammar: Grammar) -> Self {
        let start = 0_usize;
        let pos = 0_usize;
        let state_set: StateSet = grammar
            .productions
            .iter()
            .map(|production| EarleyItem {
                production: production.clone(),
                start,
                pos,
            })
            .collect();
        EarleyParser {
            grammar,
            pos,
            state_sets: vec![state_set],
            chars: vec![],
        }
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
    let mut parser = EarleyParser::from(grammar.clone());
    parser.consume_str("1+(2*3-4)".to_string());
    print!("{parser}")
}
