
# nccl [![Rust](https://github.com/zphixon/nccl/actions/workflows/rust.yml/badge.svg)](https://github.com/zphixon/nccl/actions/workflows/rust.yml)

**non-crap config language**

It's as easy as five cents. Also not crap, which is kind of the point.

* key/value bindings
* flexible indentation (eat it, python!)
* merging configurations together

[Crates.io](https://crates.io/crates/nccl) - [Docs](https://docs.rs/crate/nccl)

## Demo

*(more comprehensive examples in the docs)*

### Simple

In rust:

```rust
fn main() {
    let source = std::fs::read_to_string("examples/config.nccl").unwrap();
    let config = nccl::parse_config(&source).unwrap();
    let ports = config["server"]["port"]
        .values()
        .map(|port| port.parse::<u16>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(ports, vec![80, 443]);
}
```

config.nccl:

```
server
    domain
        example.com
        www.example.com
    port
        80
        443
    root
        /var/www/html
```

Internally, your configuration is a tree. There is no real distinction between
keys and values, everything is a node. 

### Inheritance

Nccl lets you define your own configuration to inherit from. If a node is
present in both, it will be merged.

inherit.nccl:

```
hello
    world
        panama
    friends
        doggos

sandwich
    meat
        bologne
        ham
    cheese
        provolone
        cheddar
```

inherit2.nccl:

```
hello
    world
        alaska
        neighbor
    friends
        John
        Alex

sandwich
    meat
        turkey
    cheese
        muenster
```

Result from `parse_config_with`:

```text
hello
    world
        panama
        alaska
        neighbor
    friends
        doggos
        John
        Alex
sandwich
    meat
        bologne
        ham
        turkey
    cheese
        provolone
        cheddar
        muenster
```

## Example config

```
# one major syntactical feature:

key
    value

# comments too

bool one
    t

bool too
    false

ints
    5280
    thirteen
    1738

dates
    2017-03-21
    20170321T234442+0400
    2017-03-21T23:44:42+04
    tomorrow

# this uses 3 spaces for the whole key
strings
   are bare words
   unless you want newlines
   in which case:
      "just\nuse quotes"
   "this is still valid"
   this """too"""

# this uses tabs for the whole key
lists
	juan
	deaux
	key
		value
	3
	false

indentation?
    must use the same for top-level values
    eg 2 or 4 spaces for one key
    or tabs for one key
```
