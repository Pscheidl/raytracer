# Raytracer

Realtime 3D raytracing engine written in Rust.

World consists of box, camera and multiple mirror balls that are moving.
<img alt="Description" src="https://github.com/PavelVavruska/raytracer/blob/master/raytracer_peek_20240706.gif">

## How To Build It ?

Download Rust compiler from [here](https://www.rust-lang.org/en-US/), change the working directory to the root of this project, then execute the following command:

```
> cargo build
```

The executable binary will appear in `target/debug`, called `raytracer`.

## Controls

- move camera keys: 

```
X axis: AD
Y axis: WS
Z axis: QE

X axis roll: RF
Y axis roll: TG
Z axis roll: CV
```
- low detail render enable key: L
- low detail rander disable key: H

## TODOs

- add new object types
- add new materials

## Bugs

- Yes.

## Older versions

<img alt="Description" src="https://github.com/PavelVavruska/raytracer/blob/master/raytracer_peek_20240701.gif">
<img alt="Description" src="https://github.com/PavelVavruska/raytracer/blob/master/raytracer_peek_20240629.gif">
<img alt="Description" src="https://github.com/PavelVavruska/raytracer/blob/master/raytracer_peek_20240623_2.gif">
<img alt="Description" src="https://github.com/PavelVavruska/raytracer/blob/master/raytracer_peek_20240623.gif">
<img alt="Description" src="https://github.com/PavelVavruska/raytracer/blob/master/raytracer_peek_20240620.gif">