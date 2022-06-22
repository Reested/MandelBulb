<p align="center">
  <a href="" rel="noopener">
 <img width=200px height=200px src="http://celarek.at/wp/wp-content/uploads/2014/05/mandelbulb_big.jpg" alt="Project logo"></a>
</p>

<h3 align="center">MandelBulb</h3>

<div align="center">

[![GitHub issues](https://img.shields.io/github/issues/Reested/MandleBulb)](https://github.com/Reested/MandleBulb/issues)
[![GitHub stars](https://img.shields.io/github/stars/Reested/MandleBulb)](https://github.com/Reested/MandleBulb/stargazers)
[![GitHub license](https://img.shields.io/github/license/Reested/MandleBulb)](https://github.com/Reested/MandleBulb)

</div>

---

<p align="center"> MandelBulb is a program that is not only built for solving fractal equations and testing single core performance but also for showing the use of C and Assembly code in Rust.
    <br> 
</p>

## 📝 Table of Contents
- [About](#about)
- [Getting Started](#getting_started)
- [Built Using](#built_using)
- [Authors](#authors)
- [Acknowledgments](#acknowledgement)
- [Screenshots](#screenshots)

## 🧐 About <a name = "about"></a>
A Mandelbulb is a 3-Dimensional Fractal whereas a Mandlebrot is a 2-Dimensional Fractal. A complex number can be defined as `a + bi` where `a` and `b` are real numbers and `i^2 = -1`. Hypercomplex numbers consist of 3 numbers representing the x, y, and z of a point in a Cartesian system. 

A Mandlebrot is defined as `z -> z^n + c`, where `z` and `c` are triplex or hypercomplex numbers. A Mandlebulb, however, incorporates triplex numbers instead of complex numbers.

How do we find the n^th value of a Mandlebulb? White and Nylander already figured this out but we must use polar or spherical coordinates instad of cartesian. To find the n^th power of a vector **v** = (x,y,z) is 
```
v^n := r^n(sin(nθ) * cos(nΦ), sin(nθ) * sin(nΦ), cos(nθ))
```
where 
```
r = sqrt(x^2 + y^2 + z^2)
Φ = arctan(y/x) = arg(x + yi)
θ = arctan(sqrt(x^2 + y^2)/z) = arccos(z/r)
```

## 🏁 Getting Started <a name = "getting_started"></a>
You will need to make sure you have [Rust](https://www.rust-lang.org/tools/install), [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html), and [CMake](https://cmake.org/install/) installed on you system.

### Step 1 - Clone Repo
Clone the repository to your local system. Make sure you have git installed.
```
git clone https://github.com/Reested/MandelBulb.git
```

### Step 2 - Run Cargo
Change directory into the repository you just installed.
```
cd MandelBulb
```

Build and Run with Cargo
```
cargo run
```

## ⛏️ Built Using <a name = "built_using"></a>
- [Kiss3D](https://github.com/sebcrozet/kiss3d) - 3D Graphics Engine
- [PB](https://github.com/a8m/pb) - Progress Bar for Rust
- [CMake](https://cmake.org) - C Compiler Toolset
- [NAlgebra](https://github.com/dimforge/nalgebra) - Linear Algebra Library

## ✍️ Authors <a name = "authors"></a>
- [Evon Dionne](https://github.com/Reested/) - Initial work

## 🎉 Acknowledgements <a name = "acknowledgement"></a>
- [Jesus Najera](https://www.cantorsparadise.com/mandelbulb-three-dimensional-fractals-d74c0317b76d)
- [Daniel White](https://www.skytopia.com/project/fractal/2mandelbulb.html#epilogue)
- [Daniel Shiffman](http://thecodingtrain.com/)
- [WIKI](https://en.wikipedia.org/wiki/Mandelbulb)

## 📸 Screenshots <a name = "screenshots"></a>
![Inside MandelBulb](/res/mandelbulb_inside.png?raw=true "Inide of a MandelBulb")
![Inside Video MandelBulb](/res/inside_of_mandelbulb.mov?raw=true "Inide video of a MandelBulb")