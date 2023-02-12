# calculator

A simple calculator written in Rust.

## Usage

```bash
cargo run
```

For now, you can edit the `main.rs` file to change the expression.

## Features

- [ ] Basic arithmetic
  - [x] Basic operators (left value, operator, right value) (ex: `2 + 2` or `8 nPr 3`)
  - [x] Parentheses
  - [x] Order of operations (PEMDAS)
  - [ ] Implicit multiplication
  - [ ] Negative numbers
- [x] Functions (either before or after a value) (ex: `sqrt 4`, or `4!`)
- [x] Constants
  - [x] pi
  - [x] e
- [ ] CLI (might never happen)
  - [ ] Read from stdin
  - [ ] Read from file
  - [ ] Read from command line

## Algorithm

I made my own algorithm to parse the expression. It's not perfect, but it works.

Basically, after I tokenize (which is pretty easy), I parse the tokens into an
AST (Abstract Syntax Tree).

The parser is in `src/parser.rs`. It has essentially two interesting functions:
`find_and_replace` and `replace_paren`.

`find_and_replace` acts as such:
1. If num op num found or fun with after as false and num found
1.1. If next is an operator of strictly higher priority than op or fun or function with strictly higher priority and after is true
     call find_and_replace at second num's index
1.2. Else
     Replace the tokens with ParsedToken
2. If num and fun with after as true found
2.1 Replace the tokens with ParsedToken

num can be either a number, a lparen, or a ParsedToken. If it's an lparen, call replace_paren with position of lparen

`replace_paren` acts as such:
1. Make subvec from at+1 to the next rparen of same depth
2. Call find_and_replace on subvec until its length is 1
3. Replace everything from at to the next rparen with the ParsedToken which is the only element in the subvec

Pliz rate. I'm not sure if this is the best way to do it, but it works. I didn't
want to dig through a bunch of random articles on the internet to find a
pre-existing algorithm, so I made my own.

## License

[Apache-2.0](LICENSE.md)
