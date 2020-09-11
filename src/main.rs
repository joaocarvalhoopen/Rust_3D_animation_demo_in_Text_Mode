//
// Rust 3D animation demo
//
// Author: Joao Nuno Carvalho
// Date:   2020.09.11
// 
// Description: This program is a 3D demo of a rotating Rust logo,
//              in text mode (inside a terminal). The program is 
//              written in Rust.
//
// Instruction on how to generate the same kind of animation for other
// logo or image in PNG:
//
// 1 - Obtain a square or squarish kind of logo.
// 2 - Edit it só you have two colors, back and white, max out the
//     contrast, and save it as a PNG.    
// 3 - Change the filename of the PNG at the start of the program code.
// 4 - Run the program by doing:
//        cargo run --release  
//     (inside the directory of the program.)    
// 5 - At the end of the program there are two functions two help
//     you test if the reading of the PNG file was correct.
//
// Important note: This code uses parts in the render from my
//                 port to Rust, from the donut Javascript made
//                 by Andy Sloane 2011.
// 
// See the following excellent explanation from Andy Sloane of the
// mathematics behind it.
// Have a donut - obfuscated c donut
// https://www.a1k0n.net/2006/09/15/obfuscated-c-donut.html
//
// My port of the render to Rust is in:
//
// joaocarvalhoopen / 3D_Text_donut_demo_in_Rust
// https://github.com/joaocarvalhoopen/3D_Text_donut_demo_in_Rust/
//
//
// License: For my part of the port is MIT Open License but the
//          original render code in Javascript doesn't have a
//          license written, that I could find.
// 


use std::{thread, time};
use slice_fill::SliceExt;
use std::process;

use image;
use image::GenericImageView;
use std::path::Path;

const PNG_FILENAME: &str = "rust_2.png";

const R1: f32 = 1.0;
const R2: f32 = 2.0; 
const K2: f32 = 5.0;

const SCREEN_WIDTH:  f32 = 80.0;
const SCREEN_HEIGHT: f32 = 50.0;

// Calculate K1 based on screen size: the maximum x-distance occurs
// roughly at the edge of the torus, which is at x=R1+R2, z=0.  we
// want that to be displaced 3/8ths of the width of the screen, which
// is 3/4th of the way from the center to the side of the screen.
// screen_width*3/8 = K1*(R1+R2)/(K2+0)
// screen_width*K2*3/(8*(R1+R2)) = K1

const K1: f32 = SCREEN_WIDTH * K2 * 6.0 / (7.0 * (R1 + R2));

/// Load the image using `png`
fn load_image_gen_points(png_filename: &str) -> Vec<(f32, f32)> 
{
    // Use the open function to load an image from a Path.
    // ```open``` returns a dynamic image.
    let ResultImg = image::open(&Path::new(& png_filename));
    let img = match ResultImg {
        Ok(img) => img,
        _       => {
                        println!("Error: While reading PNG file");
                        process::exit(1);
                    }, 
    };

    let (width, height) = img.dimensions();
    
    // The dimensions method returns the images width and height
    //println!("dimensions {:?}", img.dimensions());

    // The color method returns the image's ColorType
    // println!("{:?}", img.color());
    
    let mut list_points: Vec<(f32,f32)> = vec![];

    // println!("Img width: {},  {}", width, height );
    let mut max_x: f32 = 0.0;
    let mut max_y: f32 = 0.0;
    
    for y in (0..height).step_by(2) {
        for x in (0..width).step_by(2) {
            let pixel = img.get_pixel(x, y);
            // println!(" x: {}  y: {}  Pixel: {:?}", x, y, pixel);
            if pixel == image::Rgba([0, 0, 0, 255]) {
                // println!("{:?}", pixel);
                let new_x = ((x as f32 - (width  as f32 / 2.0)) * 2.0) / (width as f32);
                let new_y = ((y as f32 - (height as f32 / 2.0)) * 2.0) / (height as f32);
                if new_x > max_x {
                    max_x = new_x;
                }
                if new_y > max_y {
                    max_y = new_y;
                }
                list_points.push( (new_x, new_y) );
            }
         }    
    }

    // Points in the limit
    list_points.push( (-1.0, -1.0) );
    list_points.push( (-1.0,  1.0) );
    list_points.push( ( 1.0, -1.0) );
    list_points.push( ( 1.0,  1.0) );
    list_points.push( ( 0.0,   0.0) );

    list_points
}

fn lin_pos(x: usize, y: usize) -> usize {
    return y * (SCREEN_WIDTH as usize) + x; 
}

fn render_frame(a: & f32, b: & f32,
    z_buffer: & mut Vec<f32>, output: & mut Vec<char>,
    point_list: & Vec<(f32, f32)> ) {

    let lux = ['.', ',', '-', '~', ':', ';', '=', '!', '*', '#', '$', '@'];

    // Pre-compute sines and cosines of A and B
    let cos_a: f32 = a.cos();
    let sin_a: f32 = a.sin();
    let cos_b: f32 = b.cos();
    let sin_b: f32 = b.sin();

    output.fill(' ');   // Space
    z_buffer.fill(0.0);

    // Theta goes around the cross-sectional circle of a torus
    let theta: f32 = 0.0;
    // Pre-compute sines and cosines of theta
    let cos_theta: f32 = theta.cos();
    let sin_theta: f32 = theta.sin();

    // Phi goes around the center of revolution of a torus
    let phi: f32 = 0.0;

    for (point_img_x, point_img_y) in point_list {
        // Pre-compute sines and cosines of phi
        let cos_phi: f32 = phi.cos();
        let sin_phi: f32 = phi.sin();

        // The x,y coordinate of each point of the image, before revolving (factored
        // out of the above equations)
        let point_x: f32 = point_img_x.clone();
        let point_y: f32 = - point_img_y.clone();

        // Final 3D (x,y,z) coordinate after rotations, directly from
        // our math above
        let x: f32 = point_x * (cos_b * cos_phi + sin_a * sin_b * sin_phi)
            - point_y * cos_a * sin_b; 
        let y: f32 = point_x * (sin_b * cos_phi - sin_a * cos_b * sin_phi)
            + point_y * cos_a * cos_b;

        // let z: f32 = K2 + cos_A * circle_x * sin_phi + circle_y * sin_A;

        let z: f32 = K2 + cos_a * point_x * sin_phi + point_y * sin_a 
             + (point_x * point_x + point_y * point_y) * -1.1;

        let ooz: f32 = 1.0 / z;  // "one over z"
    
        // x and y projection.  note that y is negated here, because y
        // goes up in 3D space but down on 2D displays.
        let xp: usize = (SCREEN_WIDTH / 2.0 + K1 * ooz * x) as usize;
        let yp: usize = (SCREEN_HEIGHT / 2.0 - K1 * ooz * y) as usize;
    
        // calculate luminance.  ugly, but correct.
        let lum: f32 = cos_phi * cos_theta * sin_b - cos_a * cos_theta * sin_phi -
            sin_a * sin_theta + cos_b * (cos_a * sin_theta - cos_theta * sin_a * sin_phi);
        // Lum ranges from -sqrt(2) to +sqrt(2).  If it's < 0, the surface
        // is pointing away from us, so we won't bother trying to plot it.
        if     xp < SCREEN_WIDTH as usize 
            && yp < SCREEN_HEIGHT as usize
            && ooz > z_buffer[lin_pos(xp, yp)] {
            
            z_buffer[lin_pos(xp, yp)] = ooz;
            let luminance_index: usize = (lum.abs() * 8.0) as usize;
            // Luminance_index is now in the range 0..11 (8*sqrt(2) = 11.3)
            // now we lookup the character corresponding to the
            // luminance and plot it in our output:
            output[lin_pos(xp, yp)] = lux[luminance_index];
        }        
    }

    // Now, dump output[] to the screen.
    // bring cursor to "home" location, in just about any currently-used
    // terminal emulation mode
    print!("\x1b[H");
    for j in 0..SCREEN_HEIGHT as usize {
        for i in 0..SCREEN_WIDTH as usize {
            print!("{}", output[lin_pos(i,j)]);
        }
        print!("\n");
    }
}

/// Fill the output Vec with the Rust 2D PNG
fn fill_output_buffer_from_point_list(output: & mut Vec<char>, point_list: & Vec<(f32, f32)>) {
   
    let mut max_x: i32 = -1;
    let mut max_y: i32 = -1;
    for (x, y) in point_list {
        println!("val de input x: {}, y: {}", x, y);
        if     (*x >= -1.0 && *x <= 1.0) 
           &&  (*y >= -1.0 && *y <= 1.0) { 

            let new_x = ( ( ( x + 1.0 ) / 2.0 ) * (SCREEN_WIDTH - 1.0)) as i32;            
            let new_y = ( ( ( y + 1.0 ) / 2.0) * (SCREEN_HEIGHT - 1.0 )) as i32;

            if new_x > max_x {
                max_x = new_x;
            }
            if new_y > max_y {
                max_y = new_y;
            }
            
            println!(" val output x: {}, y: {}", new_x, new_y);
            output[lin_pos(new_x as usize, new_y as usize)] = '*';
        }
    }    

    println!("MAX x: {}, y: {}", max_x, max_y);
}

fn print_output_buffer(output: & mut Vec<char>) {
    print!("\x1b[H");
    for j in 0..SCREEN_HEIGHT as usize {
        for i in 0..SCREEN_WIDTH as usize {
            print!("{}", output[lin_pos(i,j)]);
        }
        print!("\n");
    }
}

#[warn(unstable_name_collisions)]
fn main() {

    let point_list = load_image_gen_points(PNG_FILENAME);
    
    let mut a: f32 = 0.0_f32;
    let mut b: f32 = 0.0_f32;

    let mut z_buffer: Vec<f32> = vec![0.0; (SCREEN_WIDTH * SCREEN_HEIGHT) as usize];
    let mut output:  Vec<char> = vec![' '; (SCREEN_WIDTH * SCREEN_HEIGHT) as usize];

    println!("\x1b[2J");
    let mut frame: i32 = 0;
    loop {
        render_frame(& a, & b, & mut z_buffer, & mut output, & point_list);
        println!("Rust_3D_animation frame: {} a: {} b: {}", frame, a, b);
        a += 0.04;
        b += 0.02;
        thread::sleep(time::Duration::from_millis(30)); // 10
        frame += 1;
    }

    // DEBUG
    // Fill the output Vec with the Rust 2D PNG
    
    // fill_output_buffer_from_point_list(& mut output, & point_list);

    // print_output_buffer(& mut output);
}
