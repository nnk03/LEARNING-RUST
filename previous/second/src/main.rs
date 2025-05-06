pub struct B {
    pub val_b : u32
}

pub struct A {
    pub val_a : u32,
    pub sb : B
}

impl Clone for B {
    fn clone(&self) -> B {
		B {
			val_b : self.val_b
		}
	}
}

// impl Copy for B {}


impl Clone for A {
	fn clone(&self) -> A {
		A {
			val_a : self.val_a,
			sb : self.sb.clone()
			// the below throws an error since move occurs, if copy trait is not implemented
			// sb : self.sb
		}

	}

}

// impl Copy for A {}



fn main() {
	let xb : B = B {val_b : 42};
	let xa : A = A {val_a : 40, sb : xb};

	println!("value in xa is {}, {}", xa.val_a, xa.sb.val_b);

	// error since xa.sb has taken ownership from xb, if copy trait is not implemented
	// println!("value in xb is {}", xb.val_b);

	let mut yb : B = B {val_b : 43};
	let mut ya : A = A {val_a : 44, sb : yb.clone()};
	println!("value in ya is {} {}", ya.val_a, ya.sb.val_b);

	let zb : &mut B = &mut yb;
	let za : &mut A = &mut ya;
	zb.val_b = 55;
	za.val_a = 54;
	za.sb.val_b = 56;
	println!("value in zb is {}", zb.val_b);
	println!("value in za is {} {}", za.val_a, za.sb.val_b);

	println!("value in yb is {}", yb.val_b);
	println!("value in ya is {} {}", ya.val_a, ya.sb.val_b);

	// not showing error
	// let cb : &mut B = &mut yb;





    // println!("Hello, world!");

}






