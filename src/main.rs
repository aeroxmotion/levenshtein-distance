#![feature(test)]

extern crate test;

const A: &'static str = "kitten";
const B: &'static str = "sitting";

fn head(x: &[u8]) -> u8 {
	x[0]
}

fn tail(x: &[u8]) -> &[u8] {
	&x[1..]
}

fn _lev_recursive(a: &[u8], b: &[u8]) -> i32 {
	if b.is_empty() {
		return a.len() as i32;
	}

	if a.is_empty() {
		return b.len() as i32;
	}

	if head(a) == head(b) {
		_lev_recursive(tail(a), tail(b))
	} else {
		1 + _lev_recursive(tail(a), b)
			.min(_lev_recursive(a, tail(b)))
			.min(_lev_recursive(tail(a), tail(b)))
	}
}

fn lev_recursive(a: String, b: String) -> i32 {
	_lev_recursive(a.as_bytes(), b.as_bytes())
}

fn lev_iterative(s: String, t: String) -> i32 {
	let s_chars = s.as_bytes();
	let t_chars = t.as_bytes();

	let m = s_chars.len();
	let n = t_chars.len();

	let mut d = vec![vec![0; n + 1]; m + 1];

	for i in 1..=m {
		d[i][0] = i as i32;
	}

	for j in 1..=n {
		d[0][j] = j as i32;
	}

	for j in 1..=n {
		for i in 1..=m {
			let cost = (s_chars[i - 1] != t_chars[j - 1]) as i32;

			d[i][j] = (d[i - 1][j] + 1)
				.min(d[i][j - 1] + 1)
				.min(d[i - 1][j - 1] + cost);
		}
	}

	d[m][n]
}

fn _lev_iiterative(s: &[u8], t: &[u8]) -> i32 {
	let m = s.len();
	let n = t.len();

	let mut v0 = vec![0; n + 1];
	let mut v1 = vec![0; n + 1];

	for i in 0..=n {
		v0[i] = i as i32;
	}

	for i in 0..m {
		v1[0] = i as i32 + 1;

		for j in 0..n {
			v1[j + 1] =
				// Deletion cost
				(v0[j + 1] + 1)
				// Insertion cost
				.min(v1[j] + 1)
				// Substitution cost
				.min(v0[j] + (s[i] != t[j]) as i32);
		}

		v0 = v1.clone();
	}

	v0[n]
}

fn lev_iiterative(a: String, b: String) -> i32 {
	_lev_iiterative(a.as_bytes(), b.as_bytes())
}

fn main() {
	println!("Lev recursive: {}", lev_recursive(A.into(), B.into()));
	println!("Lev iterative: {}", lev_iterative(A.into(), B.into()));
	println!("Lev iiterative: {}", lev_iiterative(A.into(), B.into()));
}

#[cfg(test)]
mod tests {
	use super::{lev_iiterative, lev_iterative, lev_recursive, A, B};
	use test::Bencher;

	#[test]
	fn recursive_lev_should_work() {
		assert_eq!(lev_recursive(A.into(), B.into()), 3);
	}

	#[bench]
	fn recursive_lev(b: &mut Bencher) {
		b.iter(|| lev_recursive(A.into(), B.into()))
	}

	#[bench]
	fn iterative_lev(b: &mut Bencher) {
		b.iter(|| lev_iterative(A.into(), B.into()))
	}

	#[test]
	fn iterative_lev_should_work() {
		assert_eq!(lev_iterative(A.into(), B.into()), 3);
	}

	#[bench]
	fn iiterative_lev(b: &mut Bencher) {
		b.iter(|| lev_iiterative(A.into(), B.into()))
	}

	#[test]
	fn iiterative_lev_should_work() {
		assert_eq!(lev_iiterative(A.into(), B.into()), 3);
	}
}
