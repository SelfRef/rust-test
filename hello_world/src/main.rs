use std::mem;

fn main() {
	let a: u8 = 123;
	println!("a = {}", a);

	let mut b: i8 = 0;
	println!("b = {} before", b);
	b = 42;
	println!("b = {} after", b);
	
	let mut c = 123456789;
	println!("c = {}, size {} bytes", c, mem::size_of_val(&c));
	c = -1;
	println!("c = {}", c);
	
	let z: isize = 123;
	let size_of_z = mem::size_of_val(&z);
	println!("z = {}, size {} bytes, {}-bit OS", z, size_of_z, size_of_z*8);

	let d: char = 'x';
	println!("d = {}, size {} bytes", d, mem::size_of_val(&d));

	let e: f32 = 2.5;
	println!("e = {}, size {} bytes", e, mem::size_of_val(&e));

	let g: bool = false;
	println!("g = {}, size {} bytes", g, mem::size_of_val(&g));
}
