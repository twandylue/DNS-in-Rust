# Useful Command

## [Convert Hex to Binary](https://unix.stackexchange.com/questions/279505/convert-hexadecimal-to-binary)

### xxd

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

## Convert Hex to Decimal in Python

```python
>>> print(int('0xFF', base=16)) 
255
```
