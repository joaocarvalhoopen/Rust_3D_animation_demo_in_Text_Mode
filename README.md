# Rust 3D animation demo in Text Mode
This is a simple and generic 3D demo, rotating logo or PNG image in text mode. 

## Description
This program is a 3D demo of a rotating Rust logo or generic PNG image, in text mode (inside a terminal). The program is written in the Rust programming language.

## Instruction on how to generate the same kind of animation for other logo or image in PNG

```

1 - Obtain a square or squarish kind of logo.

2 - Edit it só you have two colors, back and white, max out the contrast, and save it as a PNG.    

3 - Change the filename of the PNG at the start of the program code.

4 - Run the program by doing:

cargo run --release  

    (inside the directory of the program.)    

5 - At the end of the program there are two functions two help you test if the reading of the PNG file was correct.

```

## Important note - It uses part of the render engine from Andy Sloane 
This code uses parts in the render from my port to Rust, from the donut Javascript made by Andy Sloane 2011. <br>
<br>
See the following excellent explanation from Andy Sloane of the mathematics behind it. <br>
<br>
**Have a donut - obfuscated c donut** <br>
[https://www.a1k0n.net/2006/09/15/obfuscated-c-donut.html](https://www.a1k0n.net/2006/09/15/obfuscated-c-donut.html) <br>
<br>
**My port of the render to Rust is in**: <br>
joaocarvalhoopen / 3D_Text_donut_demo_in_Rust <br>
[https://github.com/joaocarvalhoopen/3D_Text_donut_demo_in_Rust/](https://github.com/joaocarvalhoopen/3D_Text_donut_demo_in_Rust/)

## License
For my part of the port is MIT Open License but the original render code in Javascript doesn't have a license written, that I could find.

## Have fun!
Best regards, <br>
Joao Nuno Carvalho 
