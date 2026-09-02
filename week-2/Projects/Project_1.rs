fn main() {
	let p : f64 = 520_000_000.0;
	let r : f64 =  10.0;
	let t : f64 =  5.0;

	//compound interest
	let a = p * ( 1.0 + (r / 100.0) ).powf(t);
	println!("The Ibeju Lekki Local Government Chairman is to pay back an amount of {}",a );
	let i = a - p;
	println!("The interest is {}",i );
}