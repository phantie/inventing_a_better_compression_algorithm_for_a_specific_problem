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

This snake is represented as where H is the head and ● represents body segments:

```python
[(2,1), (3,1), (4,1), (4,2)]
```

But what how much memory it takes?

#### Sailing to the land of Rusty Crabs

We need to decide on the *first*[^1] representation of this sequence in Rust.

```python
[(2,1), (3,1), (4,1), (4,2)]
```

Take a top-down approach at decomposing this representation:

1. it's a sequence
2. a sequence of pairs
3. a pair of `x` and `y` positions
4. `x` position
5. `y` position

From here we take a bottom-top approach:

##### `x` position and `y` position

- no reason for them to take differing amounts of memory
- it must be in the space of both negative and positive numbers, so we have few *built-in* choices
  - `isize`: pointer-sized signed integer (32-bit on 32-bit systems, 64-bit on 64-bit systems)
    - type not to be used in this context
  - `i8`: 8-bit signed integer (-128 to 127)
    - likely too small
  - `i16`: 16-bit signed integer (-32,768 to 32,767)
    - might suffice, but risky if requirements change
  - `i32`: 32-bit signed integer (-2,147,483,648 to 2,147,483,647)
    - sweet spot
  - `i64`: 64-bit signed integer (-9,223,372,036,854,775,808 to 9,223,372,036,854,775,807)
    - clearly too large

*So choose `i32` for both `x` and `y` positions*

##### a pair of `x` and `y` positions

A pair is product type [^2] and may be represented:

- as tuple

  ```rust
  (x, y)
  ```

- as struct

  ```rust
  pub struct Pos {
      pub x: i32,
      pub y: i32,
  }
  ```

What do we choose?

- from the perspective of memory consumption
  - both take the same amount of memory
    - for this matter, memory layout [^3] for `struct` does not matter: `#[repr(C)]` or default Rust alignment take the same space

- from the perspective of usability
  - tuple

    ```rust
    let pos = (0, 0);
    let x = pos.0; // access by index in the tuple
    let x = pos.1; // access by index in the tuple
    ```

  - struct

    ```rust
    let pos = Pos { x: 0, y: 0}
    let x = pos.x; // access by attribute name
    let y = pos.y; // access by attribute name
    ```

*`struct` wins from the perspective of usability.*

[^1]: Because: firstly, yet we don't even consider optimizations - we need to get stuff done; and secondly it's just *one of the views* on the same *entity* - so we could transform it later for more convenience

[^2]: [https://en.wikipedia.org/wiki/Product_type](https://en.wikipedia.org/wiki/Product_type)

[^3]: [https://doc.rust-lang.org/reference/type-layout.html](https://doc.rust-lang.org/reference/type-layout.html)
