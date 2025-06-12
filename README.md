# Inventing a Better Compression Algorithm for a Specific Problem

**Domain-specific knowledge can lead to dramatically better compression results compared to general-purpose algorithms like `brotli`.**

## What domain are we entwined with for today?

A game of snake. We are only interested that it can be represented as a sequence of coordinates on infinite integer space, where the first coordinate is the head of a snake.

Each coordinate is a pair of numbers representing a position:

- The first number is the X position (horizontal)
- The second number is the Y position (vertical)

```python
(2, 1)  # means X = 2, Y = 1
```

### Let's take this example to demonstrate typical enconding

Consider this snake on a grid:

```txt
    0 1 2 3 4 5 6
  0 . . . . . . .
  1 . . H ● ● . .
  2 . . . . ● . .
  3 . . . . . . .
  4 . . . . . . .
```

This snake is represented as:

```python
[(2,1), (3,1), (4,1), (4,2)]
```

where H is the head and ● represents body segments.
