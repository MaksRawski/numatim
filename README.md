# NUMATIM - Number Verbatim

Returns a given number in words. To choose a language use the `-l` option.
Supported languages can be found in `dicts`. Default language is English.
> Throughout this README I will be referring to those files in `dicts` as dicts. (duh)
By no means am I a linguist so there is a high chance that 
there are languages which are impossible to implement in terms of those dictionaries.
If that's the case, feel free to open an issue.

## TODO:
* store dictionaries in ~/.config/numatim probably using `confy`
* clean up this README before pushing
* add toki pona

## References used for creating dicts

* english dict - https://www.davesavage.com/names-of-very-big-numbers1.html - slightly modified
* polish dict - https://highlab.pl/nazwy-duzych-liczb/

## Structure of dictionaries

Dictionaries should have two main sections:
1. `numbers`
2. `powers`

Take a look at [english dict](for a real life example).
For special cases eg. how something should be varied it must be put inside an array.
More detail can be found in particular sections.

### `numbers`

* Digits (1-9)
* Tens (10-90) in many languages "teens" have different structure. To specify them one should use following structure:
`[teen, rest]`
in english seperation between `rest` and digit is `-` which should be noted as eg. `fifty-`
Example: 2 - `["twelve", "twenty-"]`
* Hundreds (100-900)

### `powers`

First element is for the delimiters for multiples of powers `[1, 2-4, 5-9]`, for now it can't be omitted.
Put all your powers of thousand here. Amount of them is truly arbitrary.
If your `power` is varied it should have multiples of that power like so: `[1, 2-4, 5-9]`
`2-4` is also for "teens"

