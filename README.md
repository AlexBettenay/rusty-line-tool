# Line Tool in Rust

## Overview

This program demonstrates **object-oriented programming (OOP)** concepts in Rust, including the use of structs, methods, and multithreading. The tool offers functionalities to:

1. Calculate the equation of a line given two points.
2. Find the intersection of two lines using equations.
3. Find the intersection of two lines using two points for each line.

Additionally, it showcases Rust features such as:
- **Error handling** with `Result` and `Option`.
- **Threading** using `std::thread`.
- **Synchronization** with `Arc` and `Mutex`.
- **User input handling** via the `std::io` module.

## Features

1. **Calculate Line Equation**
   - Input two points, and the program calculates the slope and y-intercept of the line.

2. **Intersection from Line Equations**
   - Input equations in the format `y = mx + c`, and the program computes the intersection point.

3. **Intersection from Coordinates**
   - Input coordinates for two lines, and the program determines the intersection point using multithreading.

4. **Interactive Menu**
   - A user-friendly menu guides you through the operations.
