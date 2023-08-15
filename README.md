# capillary

Library for HashMap-like storage of key-value pairs, but allowing for
step-by-step (partial) search of value.

Internally it uses an implementation of a tree. This makes it possible to search
for a value by providing partial steps.

## Use case

One use case, that served as inspiration to create this crate, is replacement
inside of a string.

For example, we might want to replace all occurances of keys with their values.
Naive and inefficient way is to iterate over keys and call `string.replace`
for every key.

The idea behind `capillary` is to iterate over string once and be able to
replace all occurances in a linear manner. Any character can lead either to a
valid, or invalid state. That way, part of the key can be used to go towards
some (potential) values. As soon as one unique value is reached, it can be
returned.
