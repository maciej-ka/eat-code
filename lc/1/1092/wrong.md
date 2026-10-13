Why wrong?
==========
Based on description misread
that A and B have to be exact substrings
of a result supersequence



### Ideas
#### Two cases to try
str1 + str2
str2 + str1
select shorter of them

#### a + b
we need to compare end of a
with beginning of b

*dubious*
start comparison by no overlap at all
and then gradually increase overlap
*correct long solution "snaps" suddenly*

*dubious*
reverse b
*reverse seems to obfuscate for "cab", "abac"*

*dubious*
and calculate longest common postfix
*subparts can be different*

#### inclusion case
cab
acaba
solution:
acaba



### Attempt 1
A: cab
B: abac

look for apperances of A end in B
cab
a**b**ac

walk backwards
until the first difference or end of word
remember length

c**ab**
**ab**ac

for solution to be valid
B start has to be reached
"a" has to be found



### Attempt 2
A: cabab
B: abac

look for apperances of A end in B
start with last of them
cabab
aba**b**ac

jump to start of B
and walk forwards

c**a**bab
aba**b**ac

break on the first difference



