
# nccl [![Freaking travis](https://travis-ci.org/cheezgi/nccl.svg?branch=master)](https://travis-ci.org/cheezgi/nccl)

**non-crap config language**

It's as easy as five cents. Also not crap, which is kind of the point.

* key/value bindings
* flexible indentation (eat it, python!)
* inheritance from existing keys

[Crates.io](https://crates.io/crates/nccl) - [Docs](https://docs.rs/crate/nccl)

## Demo

### Simple

In rust:

```rust
let config = nccl::parse_file("config.nccl").unwrap();
let ports = config["server"]["port"].keys_as::<i64>().unwrap();
assert_eq!(ports, vec![80, 443]);
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

`nccl` stores your configuration internally as a tree. Leaf nodes are referred
to as "values," and branch nodes are referred to as "keys." So in this example,
`root` is a key, and `/var/www/html` is its value.

### Inheritance

Nccl lets you define your own configuration to inherit from. Just use
`nccl::parse_file_with` with the result from the configuration you would like
to inherit from.

Note, if a key is present in both the parent configuration and the child
configuration, the key will not be duplicated. Values that are present in both
configurations with the same path will be duplicated.

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

In rust:

```rust
let schemas = nccl::parse_file("examples/inherit.nccl").unwrap();
let user = nccl::parse_file_with("examples/inherit2.nccl", schemas).unwrap();
assert_eq!(user["sandwich"]["meat"].keys().len(), 3);
assert_eq!(user["hello"]["world"].keys().len(), 3);
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

