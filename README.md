# bulb-parser

Parser and interpreter for [Bulb Script](https://docs.fireflyzero.com/dev/bulb/), a scripting language for making simple adventure games for [Firefly Zero](https://fireflyzero.com/).

This is a zero-dependency environment-agnostic Rust crate. The interpreter wrapper for running Bulb games on Firefly Zero lives in the [firefly-bulb](https://github.com/firefly-zero/firefly-bulb) repo.

## Installation

```bash
cargo add bulb-parser
```

## Usage

Parse the file and get pretty and efficient grouped structs for all sections:

```rust
let script = "...";
let sections = bulb_parser::parse(script).unwrap();
```

We `unwrap` here but you probably want to nicely handle the error and show it to the user.

Make a state manager (aka interpreter) and run some actions:

```rust
let mut state = bulb_parser::State::new(sections);
state.enqueue("action_id");
loop {
    let Some(action) = state.pop() else {
        break;
    };
    state.apply(action);
}
```

State takes ownership of Sections but you can still access it as `State.sections` attribute.

Actions are poped explicitly so that you can get into the loop and run additional logic on some specific actions. Namely, handle `Action::Say` to display text messages.

## Stability

We plan to keep backward compatibility for the scripting language but the structs representing them may change significantly. If you're making an interpreter for Bulb, be ready to adjust your code for new features with almost every release. On the positive side, the Rust type checker is a good guide!

## License

MIT License. Make some cool Bulb interpreters and editors, open-source or not. Happy hacking!
