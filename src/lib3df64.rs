/*!
nphysics  [![Build Status](https://travis-ci.org/sebcrozet/nphysics.png?branch=master)](https://travis-ci.org/sebcrozet/nphysics)
========
**nphysics** is a 2 and 3-dimensional physics engine for games and animations. It uses
[ncollide](https://github.com/sebcrozet/ncollide) for collision detection, and
[nalgebra](https://github.com/sebcrozet/nalgebra) for vector/matrix math.

Its most distinctive feature is its genericity wrt the simulation
dimension. That means you can use it for both 2-dimensional physics and
3-dimensional physics. Higher dimensions could be possible, but **nphysics**
has not be written/tested with those in thought.

Examples are available on the `examples` directory.
There is also a short (outdated) [demonstration video](http://youtu.be/CANjXZ5rocI).

An on-line version of this documentation is available [here](http://www.rust-ci.org/sebcrozet/nphysics/doc/nphysics3df64/index.html).

## Why another physics engine?
There are a lot of physics engine out there.
However having a physics engine written in rust is much more fun than
writing bindings and has several advantages:

- it shows that rust is suitable for soft real-time applications
- it shows how well rust behaves with highly generic code
- it shows that there is no need to write two separate engine for 2d and 3d:
  genericity wrt the dimension is possible (modulo low level arithmetic
  specializations for each dimension).
- in a not-that-near future, C++ will die of ugliness. Then, people will
  search for a physics engine and **nphysics** will be there, proudly
  exhibiting its _rusty_ sexyness.

## Compilation
You will need the last rust compiler from the master branch.
If you encounter problems, make sure you have the last version before creating an issue.

The simplest way to build **nphysics** and all its dependencies is to do a
recursive clone:


    git clone --recursive git://github.com/sebcrozet/nphysics.git
    cd nphysics
    make deps
    make

To build the examples to the `bin` folder:


    make examples_deps
    make examples


Use `./your_favorite_example_here --help` to see all the cool stuffs you can do.

## Features
- static and dynamic rigid bodies
- common convex primitives: cone, box, ball, cylinder
- concave geometries build from convex primitives (aka. compound geometries)
- stable stacking
- island based sleeping (objects deactivation)
- ray casting
- swept sphere based continuous collision detection
- ball-in-socket joint
- fixed joint

## What is missing?
**nphysics** is a very young library and needs to learn a lot of things to
become a grown up. Many missing features are because of missing features on
**ncollide**. Features missing from **nphysics** itself include:

- kinematic bodies
- efficient signaling system
- more joints, joint limits, joint motors and breakable joints.
- soft-bodies (see https://github.com/natal/roft for a draft)
- parallel pipeline
- GPU-based pipeline

## Dependencies
All dependencies are automatically cloned with a recursive clone.
The libraries needed to compile the physics engine are:

* [ncollide](https://github.com/sebcrozet/ncollide): the collision detection library.
* [nalgebra](https://github.com/sebcrozet/nalgebra): the linear algebra library.

The libraries needed to compile the examples are:

* [kiss3d](https://github.com/sebcrozet/kiss3d): the 3d graphics engine.
* [rust-sfml](https://github.com/JeremyLetang/rust-sfml): the 2d graphics engine.
*/

#[crate_id = "nphysics3df64#0.1"];
#[crate_type = "lib"];
#[deny(non_camel_case_types)];
#[deny(unnecessary_parens)];
#[deny(non_uppercase_statics)];
#[deny(unnecessary_qualification)];
#[deny(missing_doc)];
#[deny(unused_result)];
#[deny(unnecessary_typecast)];
#[warn(non_camel_case_types)];
#[feature(managed_boxes)];
#[feature(globs)];
#[doc(html_root_url = "http://www.rust-ci.org/sebcrozet/nphysics/doc")];

extern crate std;
extern crate rand;
extern crate serialize;
extern crate collections;
extern crate nalgebra;
extern crate ncollide = "ncollide3df64";
#[cfg(test)]
extern crate test;

pub mod aliases;

pub mod integration;
pub mod detection;
pub mod resolution;

pub mod world;

pub mod object;

pub mod utils;
