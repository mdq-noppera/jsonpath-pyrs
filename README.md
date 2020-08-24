# jsonpath_pyrs
## Python Wrapper for Rust crate jsonpath_lib.

Uses rust crates serde and jsonpath_lib to provide a Python api with working JSON path filters.

# Install:
1. From source:
- install [rust](https://rustup.rs/)
- switch to nightly compiler: `rustup override set nightly`
- `pip install bdist_wheel`
- build wheel `python setup.py bdist_wheel`
- pip install created .whl file

2. From release:
- download from releases version for your os and python version
- pip install .whl file

## Currently compiled for:
Linux 64 Bit - Python 3.7

## Features:
- retrieve list of Python Strings, Integers, Floats, Booleans and Lists of those values (1 level max)

## NOT implemented:
- retrieving whole json objects with expression

## Usage:
### reading from json string
``` 
import jsonpath_pyrs as jp

json_str = '{"books": [{"name": "lord of the rings", "author": "J.R.R. Tolkien"}, {"name": "Harry Potter and the Philosopher\'s Stone", "author": "J.K. Rowling"}]}'
expression = "$.books[?(@.author == 'J.R.R. Tolkien')].name"

value = jp.get_value_from_json_str(json_str, expression)


print(type(value))
print(value)
```
### Output
```
<class 'str'>
lord of the rings
```
### Reading from json file
Values of 'example.json'
```
 
{
  "firstName": "John",
  "lastName": "Smith",
  "isAlive": true,
  "age": 27,
  "address": {
    "streetAddress": "21 2nd Street",
    "city": "New York",
    "state": "NY",
    "postalCode": "10021-3100"
  },
  "phoneNumbers": [
    {
      "type": "home",
      "number": "212 555-1234"
    },
    {
      "type": "office",
      "number": "646 555-4567"
    }
  ],
  "children": [],
  "spouse": null
}
```
Python code
```
import jsonpath_pyrs as jp

filename = "example.json"
expression = "$.phoneNumbers[*].number"

value = jp.read_values_from_file(filename, expression)


print(type(value))
print(value)
```
Output
```
<class 'list'>
['212 555-1234', '646 555-4567']
```
