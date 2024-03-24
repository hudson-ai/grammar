Curious thought.

Can we build an "index" from a grammar and a set of tokens?

Idea enumerate all Earley items, and for each do the following:
1. Initialize Earley Parser with first state set containing only the single Earley item.
1. For each token (string), consume the string with the parser.
    - If the token is rejected (specifically, leading to an invalid state -- partial parses are OK), drop it and continue.
1. Collect the generated state sets in an index.

On later parses, we can just add (union of) the states we get by iterating over the Earley items in the last state set with the token being consumed.

This index might be huge. Can we compress it?