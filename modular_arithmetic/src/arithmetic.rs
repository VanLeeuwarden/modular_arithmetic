use std::u64;
use std::i64;


pub fn mod_add(a:u64, b:u64, q:u64) -> u64 {
	let a0 = a % q;
	let b0 = b % q;
	let sum = a0.checked_add(b0);
	match sum {
		Some(x) => x % q,
		None => slow_add(a0,b0,q)
	}
}

pub fn mod_abs(a: i64, q: u64) -> u64 {
	let mut abs_a = a;
	while abs_a < 0 {
		abs_a += q as i64;
	}
	abs_a as u64
}

pub fn mod_sub(a: i64, b: i64, modulus: u64) -> u64 {
	mod_add(mod_abs(a,modulus), mod_abs(b, modulus), modulus)
}


//assumes a,b < q
fn slow_add(a:u64, b:u64, q:u64) -> u64 {
	if a > b {
		let neg_b = q -b;
		a - neg_b
	} else {
		let neg_a = q - a;
		b - neg_a
	}
}


pub fn mod_mul(a:u64, b:u64, q:u64) -> u64 {
	let a0 = a % q;
	let b0 = b % q;
	let prod = a0.checked_mul(b0);
	match prod {
		Some(x) => x % q,
		None => slow_mul(a0, b0, q)
	}
}

fn slow_mul(a:u64, b:u64, q:u64) -> u64 {
	let mut total = 0;
	for _ii in 0..b {
		total = mod_add(total, a, q);
	}
	total 
}

pub fn mod_exp(base:u64, exponent:u64, modulus:u64) -> u64 {
	let mut value = 1;
	let mut digits = exponent;
	let mut b = base;
	while digits != 0 {
		if digits % 2 != 0 {
			value = mod_mul(value, base, modulus);
			digits -= 1;
		}
		digits /= 2;
		b = mod_mul(b, b, modulus);
	}
	return value;
}
#[test]
fn verify_mod_exp(){
	assert_eq!(mod_exp(0, 100, 6), 0);
	assert_eq!(mod_exp(1, 3, 20), 1);
	assert_eq!(mod_exp(3, 11, 6), 3);
	assert_eq!(mod_exp(4, 5, 7), 2);
	assert_eq!(mod_exp(18446744073709551615, 30, 10), 5);
}