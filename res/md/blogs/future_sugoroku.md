---
title: "Future Sugoroku - Reaching a high probability solver"
date: "2026-10-06"
tags: ["cs", "computer-science"]
wordNum: 0
readTime: 0
---
# Problem state

Given a maze of size N find the exit of the maze. The maze has 10 people in it,
represented by $X_i$. Each $X_i$ has 15 points
stored representing their life points. The starting point of the maze for all
$X_i$ is in the center of the maze. Each player can move around the maze
with restrictions.

a) Random unbiased numbers (R) are generated per tile of matrix, where 
0 <= R <= cardinality of X. The amount of numbers generated
depend on how many adjacent tiles there are (at most 4).
$X_i$ can move in groups of size up to R to another door and only one of $X_i$
has to pay the cost $C_xy$ of this move. 

b) If only one $X_i$ sits in the room they can only move
if at least 2 of it's adjacent neighbours along the same axis have
at least $X_j$ in them.

c) For all $i, HP(X_i)$ > 0.

d) Not moving inquires a cost from one of the $X_i$ in the room
which is the same as inquired $C_xy$ to enter the door.

e) Additionally, the same people who left the room the previous
round, cannot go back in round(i) + 1.

Find the exit with all players being alive.
## Restrictions

1) $i \in \{0, 1, \dots, 9\}$
2) $C_xy$ >= 1 & $C_xy$ <= 10
3) Cardinality of N is not limited

