# nccl

**non-crap config language**

It's as easy as five cents. Also not crap, which is kind of the point.

* key/value bindings
* no data types
* inherit stuff from other bindings

## Demo

In rust:

```rust
// TODO
let config = nccl::parse_file("config.nccl");
let ports = config["server"]["port"].keys_as::<i32>();
assert_eq!(ports, vec!["80", "443"]);
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

strings
    are bare words unless
    they have a colon
    "in which case:"
        just use quotes
    "this is still valid"
    this """too"""

lists
    juan
    deaux
    key
        value
    3
    false

schema
    must be a top-level key
        looks like normal
    default value
        in fact is normal!
    more things
        other default
    no default

inherit from: schema
    uses colon to inherit
    default value
        override default
    more things
        override again
    no default
        liar

indentation?
    four spaces
    no tabs
    sorry haters
        the one true indent style
```

