# Raytracer

Raycast engine written in Rust.

<img alt="Description" src="https://github.com/PavelVavruska/raytracer/blob/master/raytracer_peek_20240611_2.gif">

11.6.2024 - three mirror balls, dot product of two 3D vectors for reflextion, better balls angle

5.6.2024 - two mirror balls, higher resolution, saving rays on the heap

4.6.2024 - fixed bouncing ray of the mirror ball, more colors, moving ball

29.5.2024  - fixed perspective, travel of the ray ends on the wall

9.10.2022 - features one time reflection from mirror balls

6.10.2022 - 2D raytracer (casting rays from the player)

## How To Build It ?

Download Rust compiler from [here](https://www.rust-lang.org/en-US/), change the working directory to the root of this project, then execute the following command:

```
> cargo build
```

The executable binary will appear in `target/debug`, called `raytracer`.

## Controls

- move arrow keys/ASWD

## TODOs

- 3D raytrace/cast

## Bugs

- Yes.
