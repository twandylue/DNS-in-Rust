# Useful Command in Linux

## [Convert Hex to bits](https://unix.stackexchange.com/questions/279505/convert-hexadecimal-to-binary)

```console
$ echo '01 20' | xxd -p -r | xxd -b -g 0 -c 8 | cut -c11-74
0000000100100000                                                
```
