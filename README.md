# Fieldsim

<p align="center">
<img src="images/field3.png" width="28%"> <img src="images/field4.png" width="26.1%"> <img src="images/field5.png" width="35.9%">
</p>

A graphical simulator for electric bodies in a 2D universe, made for plotting isopotentials and field lines.

## Functionality

Currently there are 5 electric bodies available:
1. Point charge
2. Dipole
3. Global field
4. Circular charge (circle of constant charge density)
5. Line charge (line segment of constant linear charge density)

The potential and electric field these generate are of slightly different form to the usual expressions in 3D. In order for Gauss's Law to hold for 2D closed contours, the electric field of a point charge must drop off as 1/r rather than 1/r². Likewise the potential must go as ln(r) rather than 1/r. Everything else follows as expected.

The program plots isopotentials and field lines using a simple adaptive RK4 algorithm, which seems to be quite accurate most of the time. Isopotentials are laid down first, and field lines are then plotted by dividing the isopotentials into segments of equal electric flux, according to [1]. This results in the density of field lines correctly corresponding to the strength of the field.

The program also has rudimentary simulation capabilities, allowing bodies to interact with eachother and move about.

All graphics are plotted using [nannou](https://nannou.cc/) and the GUI is done with nannou\_egui, an interface to [egui](https://github.com/emilk/egui).

## Interface

As it stands, the user interface is incomplete. There is a small GUI with 4 buttons:
* Add isopotential - Press this and then click anywhere to start an isopotential at that point
* Draw field lines - Draw the field lines from all placed isopotentials
* Clear lines - Delete all isopotentials and field lines
* Add - Add the selected type of body
Additionally the simulation is toggled with Space.

<p align="center">
<img src="images/menu.png" width="20%"> <img src="images/add_menu.png">
</p>

## Building

There is no trickery afoot here. Simply clone the repo and build/run using Rust's [Cargo](https://doc.rust-lang.org/cargo/index.html).

## References

[1] E. J. Horowitz; A general field‐line plotting algorithm. _Computers in Physics and IEEE Computational Science & Engineering_ 1 July 1990
