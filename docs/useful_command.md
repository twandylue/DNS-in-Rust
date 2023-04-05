# Useful Command

## [Convert Hex to Binary](https://unix.stackexchange.com/questions/279505/convert-hexadecimal-to-binary)

### Bash

```console
$ echo '01 20' | xxd -p -r | xxd -b -g 0 -c 8 | cut -c11-74
0000000100100000
```

### Python

```python
>>> format(0xFF, '0>16b')
'0000000011111111'
```

```python
>>> f'{0xFF:0>16b}'
'0000000011111111'
```

<details>
  <summary>Breaking down the expression:</summary>

```plain
- f'...': creates an f-string, allowing expressions to be evaluated within curly braces {}.
- 0xFF: the integer value 255 in hexadecimal notation.
- :0>16b: formats the value as a binary string with a width of 16 digits, padding the left side with 0's if necessary.
- :: start of the format specifier.
- 0: specifies to pad with 0's.
- >: specifies to right-align the value (by default, it is left-aligned).
- 16: specifies the width of the field.
- b: specifies to format as a binary string.
```

</details>

## Convert Hex to Decimal

```python
>>> print(int('0xFF', base=16)) 
255
```

## Display binary data

### hexdump

```console
$ hexdump <binary-file>
...
```

### xxd

```console
$ xxd <binary-file>
...
```

> [Combine xxd with Vim](https://vim.fandom.com/wiki/Hex_dump)
