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

pub fn mod_exp(base :u64, exponent :u64, q:u64) -> u64 {
	if exponent == 0{
		return 1
	}

	let reduced_base = base % q;
	let mut current_state = reduced_base;

	for _i in 0..exponent-1 {
		current_state = mod_mul(current_state, reduced_base, q);
	}
	current_state
}
