

const size: usize= 10

struct 	Poly {
	coeff: [f64;size],
	degree: u32,
}

impl Poly {
	fn eval ( &self , x: f64 ) -> f64 
	{
		let mut ans = 0
		let mut temp1 = 0
		let mut temp2 = 0
		for i in 0..self.degree
		{ 
			temp1 = x.powi(i)
			temp2 = self.coeff[i] * temp1
			ans = ans + temp1
		}
	}	

}

fn main()
{
	let p1 = poly { coeff: [2,5,1,3,0,1,0,0,0,0] , degree: 6 }
	println! { "<{}>", p1.eval(2.2) }
}