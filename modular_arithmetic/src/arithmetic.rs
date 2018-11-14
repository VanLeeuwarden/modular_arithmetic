use std::u64;


pub fn mod_add(a:u64, b:u64, q:u64) -> u64 {
	let a0 = a % q;
	let b0 = b % q;
	let sum = a0.checked_add(b0);
	match sum {
		Some(x) => x % q,
		None => slow_add(a0,b0,q)
	}
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
