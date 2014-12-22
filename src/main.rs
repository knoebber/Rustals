

extern crate num;
extern crate image;


use std::io::File;
use image::GenericImage;
//use std::rand::random;
use image::ImageBuffer;
use image::Luma;
use std::os;
const WIDTH: uint = 1000;
const HEIGHT: uint = 1000;





/*
encodes a circle into a 1d array
uses midpoint circle algorithm
note: will break if attempts to a draw a circle off the screen because 
it uses signed integers. 
*/
fn draw_circle(pixels:&mut[u8],x0:int,y0:int,radius:int)
{
	let mut f = 1 - radius;
	let mut ddf_x = 1i;
	let mut ddf_y = -2 * radius;
	let mut x = 0i;
	let mut y = radius;
	pixels[get_index_i(x0+x,y0+y)]=255;
	pixels[get_index_i(x0,y0-radius)]=255;
	pixels[get_index_i(x0+radius,y0)]=255;
	pixels[get_index_i(x0-radius,y0)]=255;
	while x < y 
	{
		if f >= 0
		{
			y = y - 1;
			ddf_y += 2;
			f += ddf_y;
		}
		x+=1;
		ddf_x +=2;
		f += ddf_x;

		pixels[get_index_i(x0+x,y0+y)]=255;
		pixels[get_index_i(x0 - x,y0+y)]=255;
		pixels[get_index_i(x0+x,y0-y)]=255;
		pixels[get_index_i(x0-x,y0-y)]=255;

		pixels[get_index_i(x0+y,y0+x)]=255;
		pixels[get_index_i(x0-y,y0+x)]=255;
		pixels[get_index_i(x0+y,y0-x)]=255;
		pixels[get_index_i(x0-y,y0-x)]=255;

	}


}
/*
	encodes a rectangle into a 1d array
*/
fn draw_rect(pixels:&mut[u8], x:uint, y:uint, w:uint, h:uint)
{
		for i in range(x,x+w)
		{
			for j in range(y,y+w)
			{
				if j==y || j==y+h-1 || i==x || i==x+w-1
				{
					let index = get_index(i,j);
					if index <= WIDTH*HEIGHT {
						pixels[index]=255;
					}
				}
			}
	}
}

/*
gets index for 1d array that uses 2d indexings
*/
fn get_index(x:uint, y:uint) -> uint
{

		x+(WIDTH*y)

}
fn get_index_i(x:int, y:int) -> uint
{

		(x+((WIDTH as int)*y)) as uint
}




fn rec_fractal(pixels:&mut[u8],x:uint, y:uint, radius:uint)
{

	draw_rect(pixels,x,y,radius * 2,radius * 2);
	if radius > 8	
	{
		rec_fractal(pixels,x+ (radius/4),y,(radius/4));
		rec_fractal(pixels,x- (radius/4),y,radius/4);
		rec_fractal(pixels,x,y+(radius/4),radius/4);
		rec_fractal(pixels,x,y-(radius/4),radius/4);
	}
}

fn circ_fractal(pixels:&mut[u8],x:int, y:int, radius:int)
{

	draw_circle(pixels,x,y,radius);
	if radius > 8
	{
		circ_fractal(pixels,x+ (radius/2),y,(radius/2));
		circ_fractal(pixels,x- (radius/2),y,radius/2);

		circ_fractal(pixels,x,y+(radius/2),radius/2);
		circ_fractal(pixels,x,y-(radius/2),radius/2);
	}
}
fn main() 
{
	//declare as slice so we can pass it around

	let pixels: &mut[u8] = &mut [0u8, ..WIDTH*HEIGHT];

	let mut file_name = "fractal.png".to_string();

  let args = os::args();
  if args.len()>1
  {
	  if args[1] == "circ"
	  {
	  	circ_fractal(pixels,500,500,400);
	  }
	  else if args[1] == "rec"
	  {
	  	rec_fractal(pixels,100,100,200)
	  }

	  else
	  {
	  	panic!("argument 1 must be: rec || circ ")
	  }
	}
	if args.len()>2
	{
		file_name = args[2].to_string();
	}
	
	//dope_fractal(pixels,250,250,500);
	//draw_circle(pixels,500,500,200);



	
	
	let mut imgbuf = ImageBuffer::new(WIDTH as u32, HEIGHT as u32);
	//put the pixels to the buffer
	for i in range(0,WIDTH)
	{
		for j in range(0,HEIGHT)
		{
			let index=get_index(i,j);
			let pixel = Luma([pixels[index]]);

			imgbuf.put_pixel(i as u32, j as u32, pixel);
		}
	}
	


	// Save the image 
	let fout = File::create(&Path::new(file_name)).unwrap();

	//We must indicate the image's color type and what format to save as.
	let _    = image::ImageLuma8(imgbuf).save(fout, image::PNG);
	}