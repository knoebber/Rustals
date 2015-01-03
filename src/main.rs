

//extern crate num;
extern crate image;


use std::io::File;
use image::GenericImage;
use std::rand::{task_rng,Rng}; 
use std::rand::random;
use image::ImageBuffer;
use image::Luma;
use std::os;
//use std::num::Float; <- need for f64.sqrt()
use std::num::FloatMath;
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
fn draw_rect(pixels:&mut[u8], x:int, y:int, w:int, h:int)
{
		for i in range(x,x+w)
		{
			for j in range(y,y+w)
			{
				if j==y || j==y+h-1 || i==x || i==x+w-1
				{
					let index = get_index_i(i,j);
					if index < WIDTH*HEIGHT {
						pixels[index]=255;
					}
				}
			}
	}
}


/*
Draws line from x0 y0 to x1 y1 
with Bresenham's line algorithm
Can go off screen without error. 
I think my lines look like shit because of casts..
*/
fn draw_line(pixels:&mut[u8],x0:int,y0:int,x1:int,y1:int)
{
	let mut delta_x:f64 = (x1-x0) as f64;
	let mut delta_y:f64 = (y1-y0) as f64;
	if delta_x < 0.0 {delta_x *= -1.0;}
	if delta_y < 0.0 {delta_y *=-1.0;}
	let mut x = x0 as f64;
	let mut y = y0 as f64;
	let mut sx: f64;
	let mut sy: f64;
	let mut err:f64;
	let mut index:uint;
	if  x0 > x1 {sx = -1.0; }
	else { sx= 1.0; }

	if y0 > y1 {sy = -1.0; }
	else { sy= 1.0; }

	if delta_x > delta_y
	{
		err = (delta_x as f64)/2.0;
		while x as int != x1
		{
			index = get_index_i(x as int,y as int);
			if index<WIDTH*HEIGHT
			{
				pixels[index]=255;
			}
			err -= delta_y;
			if err < 0.0
			{
				y += sy;
				err+= delta_x;
			}
			x+= sx;
		}
	}
	else
	{
		err = delta_y  / 2.0;
		while y as int != y1 
		{
			index = get_index_i(x as int,y as int);
			if index<WIDTH*HEIGHT
			{
			 pixels[index]=255;
			}

			err-=delta_x as f64;
			if err < 0.0
			{
				x += sx;
				err += delta_y as f64;
			}
			y+= sy;
		}
		index = get_index_i(x as int,y as int);
		if index<WIDTH*HEIGHT
		{
			pixels[index]=255;
		}
	}
	
}
/*
draws a polar line. 
treats x0 y0 as the origin. 
takes a degree for simplicity. 
returns the endpoint.
*/

fn draw_polar_line(pixels:&mut[u8],x0:int,y0:int,r:int,degree:int) -> (int,int)
{
	let theta:f64 = (3.14159265359 / 180.0) * degree as f64;
	//println!("theta: {}",theta);
	let x1:f64 = (r as f64)*(theta.cos())+x0 as f64;
	let y1:f64 = (r as f64)*(theta.sin())+y0 as f64;
	draw_line(pixels,x0,y0,x1 as int,y1 as int);
	(x1 as int, y1 as int)
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

fn on_screen(x:int,y:int) -> bool
{
	(x>=0 && x <= WIDTH as int && y>=0 && y <= HEIGHT as int)
}


fn square_fractal(pixels:&mut[u8],x:int, y:int, side:int)
{

	draw_rect(pixels,x,y,side,side);
	if side > 5
	{
		square_fractal(pixels,x+ side/2,y,side/2);
		square_fractal(pixels,x- side/2,y,side/2);
		square_fractal(pixels,x,y+side/2,side/2);
		square_fractal(pixels,x,y-side/2,side/2);
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
fn squares(pixels:&mut[u8],x:int, y:int,length:int, theta:int)
{
	let first = draw_polar_line(pixels,x,y,length,theta);
	let second = draw_polar_line(pixels,x,y,length,theta - 90);
	if length > 15
	{
		squares(pixels,first.0,first.1,length - 5,theta);
		squares(pixels,second.0,second.1,length - 5,theta);
	}
}

fn curved_tree(pixels:&mut[u8],x:int, y:int,length:int, theta:int)
{
	let endpoints = draw_polar_line(pixels,x,y,length,theta);
	if length > 2 && on_screen(x,y)
	{
		curved_tree(pixels,endpoints.0,endpoints.1,length/2,theta);
		curved_tree(pixels,endpoints.0,endpoints.1,length*15/16,theta-30);
	}
}

fn tree(pixels:&mut[u8],x:int, y:int,length:int, theta:int)
{
	//println!("length: {}",length);
	//println!("theta: {}",theta);
	//println!("x: {}",x);
	//println!("y: {}",y);
	let endpoints = draw_polar_line(pixels,x,y,length,theta);
	if length > 20 && on_screen(x,y)
	{
		tree(pixels,endpoints.0,endpoints.1,3*length/4,theta); //take out this for just a double tree.
		tree(pixels,endpoints.0,endpoints.1,3*length/4,theta+20);
		tree(pixels,endpoints.0,endpoints.1,3*length/4,theta-20);
	}
}

fn rand_tree(pixels:&mut[u8],x:int, y:int,length:int, theta:int)
{
	let endpoints = draw_polar_line(pixels,x,y,length,theta);
	if length > 50 && on_screen(x,y)
	{
		//println!("ayy");
		let mut new_theta:int = theta+task_rng().gen_range(-30,30);
		//rand_tree(pixels,endpoints.0,endpoints.1,2*length/3,new_theta + 15);
		rand_tree(pixels,endpoints.0,endpoints.1,2*length/3,new_theta - 30);
		rand_tree(pixels,endpoints.0,endpoints.1,2*length/3,new_theta + 30);
		let mut branchs = 0;
		while random()    //1/2 chance to make a new branch
		{
			branchs += 1;
			if branchs % 2== 0
			{
				new_theta -= branchs * 30;
			}
			else
			{
				new_theta += branchs * 30;
			}
			rand_tree(pixels,endpoints.0,endpoints.1,2*length/3,new_theta);
		} 
	}
}

fn snow_flake(pixels: &mut[u8],x:int,y:int,length:int,theta:int,depth:int)
{	
		let end = draw_polar_line(pixels,x,y,length,theta);
		//println!("depth: {}",depth);
		if depth > 0 && on_screen(x,y)
		{
		snow_flake(pixels,x,y,length/3,theta,depth-1);
		snow_flake(pixels,end.0,end.1,length,theta-60,depth-1);
		snow_flake(pixels,end.0,end.1,length/3,theta+120,depth-1);
		//snow_flake(pixels,end.0,end.1,length,theta-60,depth-1);
		}

}

fn exp(pixels: &mut[u8],x:int,y:int,radius:int,depth:int)
{
	draw_rect(pixels,x - radius,y - radius,radius*2,radius*2);
	draw_circle(pixels,x,y,radius);
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
	  	square_fractal(pixels,500,500,500);
	  }
	  else if args[1] == "square" 
	  {
	  	//draw_polar_line(pixels,500,500,100,90);
	  	squares(pixels,0,0,80,150);
	  }

	 	else if args[1] == "ctree" 
	  {
	  	//draw_polar_line(pixels,500,500,100,90);
	  	curved_tree(pixels,700,400,150,240);
	  }
	  else if args[1] == "tree" 
	  {
	  	//draw_polar_line(pixels,500,1000,500,270);
	  	tree(pixels,500,1000,240,270);
	  }
	  else if args[1] == "rtree" 
	  {
	  	rand_tree(pixels,500,1000,250,270);
	  }

	  else if args[1] == "snow" 
	  {
	  	snow_flake(pixels,500,500,300,270,15);
	  }
	  else if args[1] == "exp"
		{
			exp(pixels,500,500,400,10);
		}
	  else
	  {
	  	panic!("the first argument be: rec || circ || ctree  || tree || square ")
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