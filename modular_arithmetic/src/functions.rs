use std::{u64, i64};
use std::cmp::{max, min};
use arithmetic::mod_abs;


//classical euclidean algorithm implemented non-recusively
pub fn gcd(a: u64, b: u64) -> u64 {
	let mut bigger = max(a,b);
	let mut smaller = min(a,b);
	let mut remainder = bigger % smaller;

	while remainder != 0 {
		bigger = smaller;
		smaller = remainder;
		remainder = bigger % smaller
	}
	smaller
}

#[test]
fn verify_gcd(){
	assert_eq!(gcd(1,2), 1);
	assert_eq!(gcd(1,4), 1);
	assert_eq!(gcd(2,4), 2);
	assert_eq!(gcd(9, 81), 9);
	assert_eq!(gcd(11,17), 1);
	assert_eq!(gcd(300,400), 100);
}

//returns the odd part of n and the exponent of the even power
pub fn split_odd_even(n: u64) -> (u64, u64) {
	let mut even_power = 0;
	let mut n_shift = n;

	loop {
		if n_shift & 1 == 0 {
			even_power += 1;
			n_shift = n_shift.rotate_right(1)
		} else {
			return (n_shift, even_power)
		}
	}
}


//transforms n into its odd part and returns the exponent of its even part
pub fn mut_even_power(n: &mut u64) -> u64 {
	let mut even_power = 0;

	loop {
		if *n & 1 == 0 {
			even_power += 1;
			*n = n.rotate_right(1);
		} else {
			return even_power;
		}
	}
}

//Jacobi symbol of 2 to the power of n
fn jacobi_even_power(pow: u64, d:u64) -> i8 {
	if d % 8 == 1 || d % 8 == 7 || pow % 2 == 0 {
		1
	} else {
		-1
	}
}

fn jacobi_neg(d: u64) -> i8 {
	if d % 4 == 1 {
		1
	} else {
		-1
	}
}

fn jacobi_flip(n: u64, d: u64) -> bool {
	if n % 4 == 1 || d % 4 == 1 {
		false
	} else {
		true
	}
}


//jacobi symbol only defined for odd d
pub fn jacobi_symbol(n: i64, d: u64) -> i8 {
	if d == 1 {
		return 1
	}

	let mut num = mod_abs(n, d);
	let mut den = d;

	let mut jacobi: i8 = 1;

	loop{
		num = num % den;

		//can this step be done only once at the beginning?
		if gcd(num, den) != 1 {
			return 0
		}

		let num_even_pow = mut_even_power(&mut num);
		jacobi *= jacobi_even_power(num_even_pow, d);

		if num == 1 {
			return jacobi
		}

		if num == d-1 {
			return jacobi * jacobi_neg(den)
		}

		if jacobi_flip(num, den) {
			jacobi *= -1;
		}

		std::mem::swap(&mut num, &mut den);
	}
}

#[test]
fn verify_jacobi_symbol() {
	assert_eq!(jacobi_symbol(1,1),1);
	assert_eq!(jacobi_symbol(25, 11), 1);
	assert_eq!(jacobi_symbol(18, 37), -1);
	assert_eq!(jacobi_symbol(24, 43), 1);
	assert_eq!(jacobi_symbol(20, 45), 0);
}