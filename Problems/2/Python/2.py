"""What I learned

Python does multi line comments with three double quotes. Most people
recommend doing single line quotes with #.

Phi is also not defined in this language:
    https://stackoverflow.com/a/25212208/5098999

** is used for exponentiation in Python

Python has a serious lack of quality documentation when compared to C#/PowerShell.

In Python you type things by wrapping them in int()

In Python, there is no way to do a for loop without a condition.
"""
import math

phi = (1 + math.sqrt(5)) / 2

sum = 0
i = 3
while True:
    value = int(((phi ** i) - (-phi ** -i)) / math.sqrt(5))
    i += 3
    if value > 4000000:
        break
    sum += value

print(sum)
