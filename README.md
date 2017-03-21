# nccl

**non-crap config language**

It's easy. I mean real easy:

```
key
    value

# supports comments
# must be on a line their own
# has some data types

bools
    t
    f

ints # by default are 64-bit signed ints
    5280
    299_792_458
    14.3
    16f
    23u
    # and are always 64-bit

dates # subset of ISO 8601
    # date
    2017-03-21
    # short date + time + timezone
    20170321T234442+0400
    # long date + time + timezone
    2017-03-21T23:44:42+04
    # and that's it

strings
    are bare words unless they match
    bools or ints
    "in which case just use quotes"
    """"""" this is still valid"
    this too"""

lists
    juan
    deaux
    key
        value
    3
    f

schema:
    ends with colon
        default value
    more things
        other default
    no default value

inherit from: schema
    more things
        override default
    no default value
        you lie!

tabs too
	I suppose...
	can't mix them though

really any
 number of spaces
  so long as it
 is consistent

invalid
   this is terrible!
   	 what is even going on here
  wat
```

