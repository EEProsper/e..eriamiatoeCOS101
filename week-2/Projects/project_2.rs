fn main() {
	let t : f64 = 2.0 * 450_000.00;
	let m : f64 = 1.0 * 1_500_000.00;
	let h : f64 = 3.0 * 750_000.00;
	let d : f64 = 3.0 * 2_850_000.00;
	let a : f64 = 1.0 * 250_000.00;

	//sum and average
	let sum = t + m + h + d + a;
	println!("The total number of sales of P.M. Okeke and Sons Ltd is {}",sum );
	let av = sum / 5.0;
	println!("The average number of sales of P.M. Okeke and Sons Ltd is {}",av );

}