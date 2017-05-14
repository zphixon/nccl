# nccl

**non-crap config language**

It's as easy as five cents. Also not crap, which is kind of the point.

* key/value bindings
* only strings
* inherit stuff from other bindings

## demo

```
# one major syntactical feature

key
    value

# supports comments
# must be on a line their own
# has no data types

bools
    t
    f
    yes
    no
    maybe?

ints
    5280
    299_792_458
    14.3
    23

dates
    2017-03-21
    20170321T234442+0400
    2017-03-21T23:44:42+04
    tomorrow

strings
    are bare words unless
    they have a colon
    "in which case: just use quotes"
    """"""" this is still valid"
    this too"""

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
    no default value

inherit from: schema
    uses colon
    more things
        override default
    no default value
        you lie!

indentation?
    yes. four spaces.
    no tabs

server
    domain
        example.com
    port
        80
    root
        /var/www/html


```

