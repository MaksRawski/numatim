# NUMATIM - Number Verbatim

Returns a given number in words. To choose a language use the `-l` option.
Supported languages can be found in `dicts`. Default language is English.

This project is work in progress so if it's impossible to implement a dict for a 
particular language please open an issue.

## Installation

`cargo install numatim`

## TODO:

* add tests
* choose default language based on locale
* add reverse option

## Terminology 

* __dict__ - json dictionary used for defining how a number should be represented in a certain language
* __tenths__ - numbers in range 10-19 - I don't think there is a name for this range but this is how I'm going to refer to it throughout this README

## References used for creating dicts

* English - http://lcn2.github.io/mersenne-english-name/tenpower/tenpower.html
* Polish - https://highlab.pl/nazwy-duzych-liczb/

## Structure of dicts

Dicts should have two main sections:

1. [numbers](#numbers)
2. [powers](#powers)

If a certain number's or power's declination needs to be hardcoded it must be put inside an array.
How exactly that must be done can be found in particular sections.
Take a look at the [dicts folder](https://gitlab.com/maksrawski/numatim/~/tree/master/dicts/) for a real life examples.

### Numbers

Numbers must contain 3 arrays of strings:

* Digits (1-9)
* Tens (10-90) in many languages tenths have different structure. 
They can be defined here: `[tenth, rest]` eg. `["twelve", "twenty"]`
* Hundreds (100-900)

### Powers

Put all your powers of thousand here. Amount of them is truly arbitrary.

* First power must contain delimiters for the following multiples `[1, 2-4, 5-9]`, for now it can't be omitted.
* To hardcode delicnation of certain power it should be in this format `[1, 2-4, 5-9]` 
(2-4 will also be used for numbers in range 10-19).
