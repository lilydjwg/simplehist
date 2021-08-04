A simple program to show a histogram on the terminal.

It reads integers from stdin and output a histogram, like this:

```
$ some_program | simplehist
# Number of samples = 43
# Min = 1
# Max = 78
#
# Mean = 24.69767441860465
# Standard deviation = 24.46215385725176
# Variance = 598.3969713358572
#
# Each ∎ is a count of 1
#
 1 ..  9 [  9 ]: ∎∎∎∎∎∎∎∎∎
 9 .. 17 [ 10 ]: ∎∎∎∎∎∎∎∎∎∎
17 .. 25 [ 13 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎
25 .. 33 [  1 ]: ∎
33 .. 41 [  0 ]: 
41 .. 49 [  1 ]: ∎
49 .. 57 [  1 ]: ∎
57 .. 65 [  0 ]: 
65 .. 73 [  4 ]: ∎∎∎∎
73 .. 81 [  4 ]: ∎∎∎∎
```
