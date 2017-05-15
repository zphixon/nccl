# nccl

**non-crap config language**

It's as easy as five cents. Also not crap, which is kind of the point.

* key/value bindings
* no data types
* inherit stuff from other bindings
* every key must have a value

## demo

```
# one major syntactical feature

key
    value

bool one
    t

bool too
    false

ints
    5280
        thirteen
    1738
        fetty wap

dates
    2017-03-21
        all sorts
    20170321T234442+0400
        gotta parse 'em yourself
    2017-03-21T23:44:42+04
        in the name of
    tomorrow
        simplicity

strings
    are bare words
        unless
    they have
        a colon
    "in which case:"
        just use quotes
    "this is"
        "still valid"
    this
        """too"""

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

inherit from: schema
    uses colon
        to inherit
    more things
        override default
    default value
        override again

indentation?
    four spaces
        sorry haters
    no tabs
        this is the real indent style

server
    domain
        example.com
    port
        80
    root
        /var/www/html


```

