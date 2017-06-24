# nccl

**non-crap config language**

It's as easy as five cents. Also not crap, which is kind of the point.

* key/value bindings
* no data types
* flexible indentation (eat it, python!)
* inherit from existing keys

## Demo

In rust:

```rust
let config = nccl::parse_file("config.nccl").unwrap();
let ports = config["server"]["port"].keys_as::<i32>().unwrap();
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
    "2017-03-21T23:44:42+04"
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

