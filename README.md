# ASCIImg

## Generate ASCII art from images (jpg, png, gif)

# Build

Build release (debug can be very slow) with cargo using:

`$ cargo build --release`

# Usage

After building:

`$ asciimg [path-to-image] [width] [height]`

With cargo:

`$ cargo run --release [path-to-image] [width] [height]`


# Example (rust mascot **Ferris**)
Result from: `$ cargo run ferris.png 70 25`
```
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@$@@@@@==@@@@=@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@==@@@-===@+====@-===@@@==@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@=========================@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@===============================-===@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@===================================@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@===========================================-@@@@@@@@@@@@@
@@@@@@@@@@@@@===========================================@@@@@@@@@@@@@@
@@@@@@@@@===================================================-@@@@@@@@@
@@@@@@@@@@=================================================-@@@@@@@@@@
@@@@@@+-=====================================================-=@@@@@@@
@@@@@@$===================@@@   =======`@@   ==================@@@@@@@
@@@@@@@-================= @@@    ===== @@@`  `=================@@@@@@@
@@@@@====================        =====        ==================-@@@@@
@@@@======================      =======      ====================-@@@@
@@@@=======``-=========================================````======+@@@@
@@@@@======````-===``============================``===`````=====-@@@@@
@@@@@@======@```#====@@@-=====```````````=====@@@-===@#```-====+@@@@@@
@@@@@@@@====-@@``@===============@@@@@=============-@@``@@====@@@@@@@@
@@@@@@@@@#===@@@@@@@@=============-@@=============@@@@@@@@===@@@@@@@@@
@@@@@@@@@@@===@@@@@@@=======--=====@=======-=====@@@@@@@@==#@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@=====-@@@@@@@@@@@@@======@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@@======@@@@@@@@@$===-@@@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
```