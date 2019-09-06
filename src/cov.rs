
use crate::*;
use crate::chol::*;

use std::fmt;

use std::convert::From;

pub type NA = [Number];
pub type NA3 = [Number; 3];
pub type NA4 = [Number; 4];
pub type NA5 = [Number; 5];
pub type NA6 = [Number; 6];
pub type NA9 = [Number; 9];
pub type NA10 = [Number; 10];
pub type NA12 = [Number; 12];
pub type NA15 = [Number; 15];
pub type NA16 = [Number; 16];
pub type NA25 = [Number; 25];

enum Dim { Dim3, Dim4, Dim5 }

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Vecn<T> { pub v: T }

pub type Vec3 = Vecn<NA3>;
pub type Vec4 = Vecn<NA4>;
pub type Vec5 = Vecn<NA5>;

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vec3:{}", pretty_matrix(3,1,&self.v))
    }
}
impl fmt::Display for Vec4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vec4:{}", pretty_matrix(4,1,&self.v))
    }
}
impl fmt::Display for Vec5 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vec5:{}", pretty_matrix(5,1,&self.v))
    }
}
impl From<NA3> for Vec3 {
    fn from(v: NA3) -> Self {
        Vec3 { v }
    }
}
impl From<NA4> for Vec4 {
    fn from(v: NA4) -> Self {
        Vec4 { v }
    }
}
impl From<NA5> for Vec5 {
    fn from(v: NA5) -> Self {
        Vec5 { v }
    }
}
impl From<Vec<Number>> for Vec3 {
    fn from(v: Vec<Number>) -> Self {
        let a: NA3 =if v.len() >= 3 { [ v[0], v[1], v[2], ] } else { [0.0;3] };
        Vec3 { v: a }
    }
}
impl From<Vec<Number>> for Vec4 {
    fn from(v: Vec<Number>) -> Self {
        let a: NA4 =if v.len() >= 4 { [ v[0], v[1], v[2], v[3], ] } else { [0.0;4] };
        Vec4 { v: a }
    }
}
impl From<Vec<Number>> for Vec5 {
    fn from(v: Vec<Number>) -> Self {
        let a: NA5 =if v.len() >= 5 { [ v[0], v[1], v[2], v[3],  v[4], ] } else { [0.0;5] };
        Vec5 { v: a }
    }
}
// pub type Vecxx = struct Vecx3 { pub v: NA3 }

// pub enum Dim { Dim3, Dim4, Dim5 }
enum WebEvent {
    // An `enum` may either be `unit-like`,
    PageLoad,
    PageUnload,
    // like tuple structs,
    KeyPress(char),
    Paste(String),
    // or like structures.
    Click { x: i64, y: i64 },
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Cov<T> { pub v: T }
pub type Cov3 = Cov<NA6>;
pub type Cov4 = Cov<NA10>;
pub type Cov5 = Cov<NA15>;

impl Cov3 {
    // SymMat
    pub fn diag(&self) -> NA3 {
        [self.v[0], self.v[3], self.v[5], ]
    }
    pub fn det(&self) -> Number {
        // [a,b,c,d,e,f] = self.v;
        let a = self.v[0];
        let b = self.v[1];
        let c = self.v[2];
        let d = self.v[3];
        let e = self.v[4];
        let f = self.v[5];
        a*d*f - a*e*e - b*b*f + 2.0*b*c*e - c*c*d
    }

    pub fn choldc(&self) -> Jac33 {
        let xx: &mut NA9 = &mut [0.0; 9];
        xx[..6].copy_from_slice(&self.v);
        do_choldc(&mut xx[..], 3);
        Jac33 { v: *xx }
    }

    pub fn cholinv(&self) -> Cov3 {
        let xx: &mut NA6 = &mut [0.0; 6];
        xx.copy_from_slice(&self.v);
        do_cholinv(&mut xx[..], 3);
        Cov3 { v: *xx }
    }
    pub fn scale_diag(&self, s: f64) -> Cov3 {
        Cov {v: [self.v[0]*s, self.v[1], self.v[2], self.v[3]*s, self.v[4], self.v[5]*s, ]}
    }
}

impl fmt::Display for Cov3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let w = 3;
        let idx = |i0: usize, j0: usize| {
            if i0 <= j0 { j0 + i0*w - (i0*(i0+1))/2 }  else { i0 + j0*w - (j0*(j0+1))/2 }
        };
        let mut v: NA9 = [0.0; 9];
        for i in 0usize..3 {
            for j in 0usize..3 {
                v[j*w+i] = self.v[idx(i,j)];
            }
        }
        write!(f, "Cov3:{}", pretty_matrix(3,3,&v))
    }
}

impl From<NA6> for Cov3 {
    fn from(v: NA6) -> Self {
        Cov3 { v }
    }
}
impl From<Vec<Number>> for Cov3 {
    fn from(v: Vec<Number>) -> Self {
        let a: NA6 =
            if v.len() == 6 { [ v[0], v[1], v[2], v[3], v[4], v[5], ] }
            else if v.len() >= 9 { [ v[0], v[1], v[2], v[4], v[5], v[8], ] }
            else { [0.0;6] }; // error
        Cov3 { v: a }
    }
}

impl Cov4 {
    pub fn diag(&self) -> NA4 {
        [self.v[0], self.v[4], self.v[7], self.v[9], ]
    }
}
impl fmt::Display for Cov4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let w = 4;
        let idx = |i0: usize, j0: usize| {
            if i0 <= j0 { j0 + i0*w - (i0*(i0+1))/2 }  else { i0 + j0*w - (j0*(j0+1))/2 }
        };
        let mut v: NA16 = [0.0; 16];
        for i in 0usize..4 {
            for j in 0usize..4 {
                v[j*w+i] = self.v[idx(i,j)];
            }
        }
        write!(f, "Cov4:{}", pretty_matrix(4,4,&v))
    }
}

impl From<NA10> for Cov4 {
    fn from(v: NA10) -> Self {
        Cov4 { v }
    }
}
impl From<Vec<Number>> for Cov4 {
    fn from(v: Vec<Number>) -> Self {
        let a: NA10 =
            if v.len() == 10 { [ v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7], v[8], v[9], ] }
            else if v.len() >= 16 { [ v[0], v[1], v[2], v[3], v[5], v[6], v[7], v[10], v[11], v[15], ] }
            else { [0.0;10] }; // error
        Cov4 { v: a }
    }
}
use std::ops::Add;
impl Add<&Cov4> for Cov4 {
    type Output = Cov4;
    fn add(mut self, other: &Cov4) -> Cov4 {
        for i in 0..10 { self.v[i] += other.v[i]; }
        self
    }
}
impl Cov5 {
    pub fn diag(&self) -> NA5 {
        [self.v[0], self.v[5], self.v[9], self.v[12], self.v[14], ]
    }
}
impl From<&Cov5> for Cov3 { // we make a Cov3 from a Cov5 by just dropping the last R indices...
    fn from(cv: &Cov5) -> Self {
        let v = &cv.v;
        let a: NA6 = { [ v[0], v[1], v[2], v[5], v[6], v[9], ] };
        Cov3 { v: a }
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct Jac<T> { pub v: T }
pub type Jac33 = Jac<NA9>;
pub type Jac34 = Jac<NA12>;
pub type Jac35 = Jac<NA15>;
pub type Jac53 = Jac<NA15>;
pub type Jac55 = Jac<NA25>;

impl Jac33 {
    pub fn tr(&self) -> Jac33 {
        let w = 3;
        let ixa = |i0: usize, j0: usize| i0*w+j0;

        let xx: &mut NA9 = &mut [0.0; 9];
        for i0 in 0..w {
            for j0 in 0..w {
                xx[ixa(i0, j0)] = self.v[ixa(j0, i0)];
            }
        }
        Jac33 { v: *xx }
    }
}
impl From<NA9> for Jac33 {
    fn from(v: NA9) -> Self {
        Jac33 { v }
    }
}

impl fmt::Display for Jac33 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Jac33:{}", pretty_matrix(3,3,&self.v))
    }
}

impl Jac55 {
    pub fn tr(&self) -> Jac55 {
        let w = 5;
        let ixa = |i0: usize, j0: usize| i0*w+j0;

        let xx: &mut NA25 = &mut [0.0; 25];
        for i0 in 0..w {
            for j0 in 0..w {
                xx[ixa(i0, j0)] = self.v[ixa(j0, i0)];
            }
        }
        Jac55 { v: *xx }
    }
}
impl fmt::Display for Jac55 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Jac55:{}", pretty_matrix(5,5,&self.v))
    }
}
impl From<NA25> for Jac55 {
    fn from(v: NA25) -> Self {
        Jac55 { v }
    }
}
impl From<Vec<Number>> for Jac55 {
    fn from(v: Vec<Number>) -> Self {
        let a: NA25 =
            if v.len() >= 25 { [ v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7], v[8], v[9], v[10], v[11], v[12], v[13], v[14], v[15], v[16], v[17], v[18], v[19],v[20], v[21], v[22], v[23], v[24], ] }
            else { [0.0;25] }; // error
        Jac55 { v: a }
    }
}

impl Jac34 {

}
impl From<NA12> for Jac34 {
    fn from(v: NA12) -> Self {
        Jac34 { v }
    }
}

impl fmt::Display for Jac34 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Jac34:{}", pretty_matrix(3,4,&self.v))
    }
}

impl Jac53 {
    pub fn tr(&self) -> Jac35 {
        let ixa = |i0: usize, j0: usize| i0*3+j0;
        let ixb = |i0: usize, j0: usize| i0*5+j0;

        let xx: &mut NA15 = &mut [0.0; 15];
        for i0 in 0..5 {
            for j0 in 0..3 {
                xx[ixa(i0, j0)] = self.v[ixb(j0, i0)];
            }
        }
        Jac35 { v: *xx }
    }
}
impl From<NA15> for Jac53 {
    fn from(v: NA15) -> Self {
        Jac53 { v }
    }
}

impl fmt::Display for Jac53 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Jac53:{}", pretty_matrix(5,3,&self.v))
    }
}

// fn new_na9_from<F: Iterator<Item=Number>>(src: F) -> NA9 {
//     let mut result: NA9 = [0.0; 9];
//     for (rref, val) in result.iter_mut().zip(src) {
//         *rref = val;
//     }
//     result
// }
// fn cmap<F: Fn(Number, Number) -> Number>(a: NA9, b: NA9, f: F) -> NA9 {
//     new_na9_from(a.iter().zip(&b).map(|(x, y)| f(*x, *y)))
// }

impl Add for Cov3 {
    type Output = Cov3;
    fn add(mut self, other: Cov3) -> Cov3 {
        for i in 0..6 { self.v[i] += other.v[i]; }
        self
    }
}
impl Add for Jac33 {
    type Output = Jac33;
    fn add(mut self, other: Jac33) -> Jac33 {
        for i in 0..9 { self.v[i] += other.v[i]; }
        // Jac33 {
        //     // v: cmap(self.v, other.v, |x, y| x + y),
        //     // v: self.v.iter().zip(&other.v).map(|(s, o)| s+o).collect()
        // }
        self
    }
}
impl Add for Jac55 {
    type Output = Jac55;
    fn add(mut self, other: Jac55) -> Jac55 {
        for i in 0..25 { self.v[i] += other.v[i]; }
        self
    }
}
impl Add<&Vec4> for Vec4 {
    type Output = Vec4;
    fn add(mut self, other: &Vec4) -> Vec4 {
        for i in 0..4 { self.v[i] += other.v[i]; }
        self
    }
}
use std::ops::Mul;
impl Mul for Vec3 {
    type Output = Number;
    fn mul(self, other: Vec3) -> Number {
        let nb = 3;
        let mut s = 0.0;
        for k in 0..nb {
            s += self.v[k] * other.v[k];
        }
        s
    }
}
impl Mul for Jac33 {
    type Output = Jac33;
    fn mul(mut self, other: Jac33) -> Jac33 {
        //self.v.len() is 9;
        let nb = 3;
        let na = self.v.len() / nb;
// indV w i0 j0 = i0*w+j0 -- w=nj width of niXnj matrix, i0=0..ni-1, j0=0..nj-1
        let ixa = |i0, j0| i0*nb+j0;
        let ixb = |i0, j0| i0*na+j0;
        let tmp = self.v.clone();                ;
        for i in 0..na {
        for j in 0..na {
        let mut s = 0.0;
        for k in 0..nb {
            s += tmp[ixa(i,k)] * other.v[ixb(k,j)];
        }
        self.v[ixa(i,j)] = s;
        }}
        self
    }
}

impl Mul<Cov3> for Jac34 {
    type Output = Cov4;
    fn mul(self, other: Cov3) -> Cov4 {
        // let l = other.v.len();
        let n = 3; // match l 6->3, 10->4, 15->5
        let m = self.v.len() / n; // mxn * nxn * nxm -> mxm
        let mut vint = self.v.clone(); // mxn * nxn -> mxn, v has same size as self.v
        let w = n; //= indVs n
        let ixa = |i0: usize, j0: usize| {
            if i0 <= j0 { j0 + i0*w - (i0*(i0+1))/2 }  else { i0 + j0*w - (j0*(j0+1))/2 }
        };
        let ixb = |i0, j0| i0*m+j0; // = indV m
        for i in 0..n {
        for j in 0..m {
        let mut s = 0.0;
        for k in 0..n {
            s += other.v[ixa(i,k)] * self.v[ixb(k,j)];
        }
        vint[ixb(i,j)] = s;
        }}
        let mut vp = [0.0_f64; 10];  // match l 6->3, 10->4, 15->5
        let ixi = |i0, j0| i0*m+j0; // = indV m
        let w = m; //= indVs m
        let ixc = |i0: usize, j0: usize| {
            if i0 <= j0 { j0 + i0*w - (i0*(i0+1))/2 }  else { i0 + j0*w - (j0*(j0+1))/2 }
        };
        for i in 0..m {
        for j in i..m {
        let mut s = 0.0;
        for k in 0..n {
            s += self.v[ixi(k,i)] * vint[ixi(k,j)];
        }
        vp[ixc(i,j)] = s;
        }}
        Cov4{v: vp}
    }
}
impl Mul<Jac55> for Jac55 {
    type Output = Jac55;
    fn mul(mut self, other: Jac55) -> Jac55 {
        //self.v.len() is 9;
        let nb = 5;
        let na = self.v.len() / nb;
// indV w i0 j0 = i0*w+j0 -- w=nj width of niXnj matrix, i0=0..ni-1, j0=0..nj-1
        let ixa = |i0, j0| i0*nb+j0;
        let ixb = |i0, j0| i0*na+j0;
        let tmp = self.v.clone();                ;
        for i in 0..na {
        for j in 0..na {
        let mut s = 0.0;
        for k in 0..nb {
            s += tmp[ixa(i,k)] * other.v[ixb(k,j)];
        }
        self.v[ixa(i,j)] = s;
        }}
        self
    }
}
impl Mul<Cov5> for Jac55 {
    type Output = Cov5;
    fn mul(self, other: Cov5) -> Cov5 {
        let n = 5;
        let w = n;
        let ixa = |i0: usize, j0: usize| i0*w+j0;
        let ixb = |i0: usize, j0: usize| {
            if i0 <= j0 { j0 + i0*w - (i0*(i0+1))/2 }  else { i0 + j0*w - (j0*(j0+1))/2 }
        };

        let mut vr = [0.0; 25];
        for i in 0..n {
        for j in 0..n {
        let mut s = 0.0;
        for k in 0..n {
            s += self.v[ixa(i,k)] * other.v[ixb(k,j)];
        }
        vr[ixa(i,j)] = s;
        }}
        vr[..].into()
    }
}

impl Mul for Cov3 {
    type Output = Jac33;
    fn mul(self, other: Cov3) -> Jac33 {
        //self.v.len() is 9;
        let n = 3;
        let w = n;
        let ixa = |i0: usize, j0: usize| {
            if i0 <= j0 { j0 + i0*w - (i0*(i0+1))/2 }  else { i0 + j0*w - (j0*(j0+1))/2 }
        };
        let ixr = |i0: usize, j0: usize| i0*w+j0;

        let mut res = Jac33 { v: [0.0; 9] };
        for i in 0..n {
        for j in 0..n {
        let mut s = 0.0;
        for k in 0..n {
            s += self.v[ixa(i,k)] * other.v[ixa(k,j)];
        }
        res.v[ixr(i,j)] = s;
        }}
        res
    }
}

impl Mul for Cov5 {
    type Output = Jac55;
    fn mul(self, other: Cov5) -> Jac55 {
        //self.v.len() is 9;
        let n = 5;
        let w = n;
        let ixa = |i0: usize, j0: usize| {
            if i0 <= j0 { j0 + i0*w - (i0*(i0+1))/2 }  else { i0 + j0*w - (j0*(j0+1))/2 }
        };
        let ixr = |i0: usize, j0: usize| i0*w+j0;

        let mut res = Jac55 { v: [0.0; 25] };
        for i in 0..n {
        for j in 0..n {
        let mut s = 0.0;
        for k in 0..n {
            s += self.v[ixa(i,k)] * other.v[ixa(k,j)];
        }
        res.v[ixr(i,j)] = s;
        }}
        res
    }
}
impl From<&[Number]> for Cov5 {
    fn from(v: &[Number]) -> Self {
        let a: NA15 =
            if v.len() == 15 { [ v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7], v[8], v[9], v[10], v[11], v[12], v[13], v[14], ] }
            else if v.len() >= 25 { [ v[0], v[1], v[2], v[3], v[4], v[6], v[7], v[8], v[9], v[12], v[13], v[14], v[18], v[19], v[24],] }
            else { [0.0;15] }; // error
        Cov5 { v: a }
    }
}
impl From<Vec<Number>> for Cov5 {
    fn from(v: Vec<Number>) -> Self {
        let a: NA15 =
            if v.len() == 15 { [ v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7], v[8], v[9], v[10], v[11], v[12], v[13], v[14], ] }
            else if v.len() >= 25 { [ v[0], v[1], v[2], v[3], v[4], v[6], v[7], v[8], v[9], v[12], v[13], v[14], v[18], v[19], v[24],] }
            else { [0.0;15] }; // error
        Cov5 { v: a }
    }
}

impl Cov5 {
    // SymMat
    pub fn det(&self) -> Number {
        // [a,b,c,d,e,f,g,h,i,j,k,l,m,n,o] = self.0;
        let (a,b,c,d,e,f,g,h,i,j,k,l,m,n,o) = (self.v[0],self.v[1],self.v[2],self.v[3],self.v[4],self.v[5],self.v[6],self.v[7],self.v[8],self.v[9],self.v[10],self.v[11],self.v[12],self.v[13],self.v[14],);
        a*f*j*m*o - a*f*j*n*n - a*f*k*k*o + 2.0*a*f*k*l*n - a*f*l*l*m
            - a*g*g*m*o + a*g*g*n*n + 2.0*a*g*h*k*o - 2.0*a*g*h*l*n - 2.0*a*g*i*k*n
            + 2.0*a*g*i*l*m - a*h*h*j*o + a*h*h*l*l + 2.0*a*h*i*j*n - 2.0*a*h*i*k*l
            - a*i*i*j*m + a*i*i*k*k - b*b*j*m*o + b*b*j*n*n + b*b*k*k*o
            - 2.0*b*b*k*l*n + b*b*l*l*m + 2.0*b*c*g*m*o - 2.0*b*c*g*n*n - 2.0*b*c*h*k*o
            + 2.0*b*c*h*l*n + 2.0*b*c*i*k*n - 2.0*b*c*i*l*m - 2.0*b*d*g*k*o
            + 2.0*b*d*g*l*n + 2.0*b*d*h*j*o - 2.0*b*d*h*l*l - 2.0*b*d*i*j*n
            + 2.0*b*d*i*k*l + 2.0*b*e*g*k*n - 2.0*b*e*g*l*m - 2.0*b*e*h*j*n
            + 2.0*b*e*h*k*l + 2.0*b*e*i*j*m - 2.0*b*e*i*k*k - c*c*f*m*o + c*c*f*n*n
            + c*c*h*h*o - 2.0*c*c*h*i*n + c*c*i*i*m + 2.0*c*d*f*k*o - 2.0*c*d*f*l*n
            - 2.0*c*d*g*h*o + 2.0*c*d*g*i*n + 2.0*c*d*h*i*l - 2.0*c*d*i*i*k
            - 2.0*c*e*f*k*n + 2.0*c*e*f*l*m + 2.0*c*e*g*h*n - 2.0*c*e*g*i*m
            - 2.0*c*e*h*h*l + 2.0*c*e*h*i*k - d*d*f*j*o + d*d*f*l*l + d*d*g*g*o
            - 2.0*d*d*g*i*l + d*d*i*i*j + 2.0*d*e*f*j*n - 2.0*d*e*f*k*l - 2.0*d*e*g*g*n
            + 2.0*d*e*g*h*l + 2.0*d*e*g*i*k - 2.0*d*e*h*i*j - e*e*f*j*m + e*e*f*k*k
            + e*e*g*g*m - 2.0*e*e*g*h*k + e*e*h*h*j
    }

    pub fn choldc(&self) -> Jac55 {
        let xx: &mut NA25 = &mut [0.0; 25];
        xx[..15].copy_from_slice(&self.v);
        do_choldc(&mut xx[..], 5);
        Jac55 { v: *xx }
    }

    pub fn cholinv(&self) -> Cov5 {
        let xx: &mut NA15 = &mut [0.0; 15];
        xx.copy_from_slice(&self.v);
        do_cholinv(&mut xx[..], 5);
        Cov5 { v: *xx }
    }
}
impl fmt::Display for Cov5 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let w = 5;
        let idx = |i0: usize, j0: usize| {
            if i0 <= j0 { j0 + i0*w - (i0*(i0+1))/2 }  else { i0 + j0*w - (j0*(j0+1))/2 }
        };
        let mut v: NA25 = [0.0; 25 ];
        for i in 0usize..5 {
            for j in 0usize..5 {
                v[j*w+i] = self.v[idx(i,j)];
            }
        }
        write!(f, "Cov5:{}", pretty_matrix(5,5,&v))
    }
}


/// pretty print of matrix
pub fn pretty_matrix(r: usize, c: usize, v: &[Number]) -> String {
    let to3fix = |x: Number| format!("{:.3}", x);
    let fill_blanks = |k: usize, str: &str| format!("{:>1$}", str, k);
    let mx: usize = v.iter().map(|x| to3fix(*x).len()).max().unwrap();
    let idx = |i0, j0| j0*c+i0;
    let fmt = |x: Number| fill_blanks(mx, &to3fix(x));
    (0usize..r).map(|j| format!("({})",
                                (0usize..c).
                                map(|i| fmt(v[idx(i,j)])).
                                fold("".to_string(), |accum, s| accum + &s + " ")
                                )
                    ).fold("".to_string(), |a, s| a + "\n" + &s)
}

#[test]
fn test_cov() {
    // let Cov<Dim3> xc3 = Cov {v: Vec}
    //
    let _ch3 : Cov3 = [2.0, -1.0, 0.0, 2.0, -1.0, 2.0].into();
    let ch3 = Cov3::from([2.0, -1.0, 0.0, 2.0, -1.0, 2.0]);
    let ch5 = Cov5{ v: [2.0, -1.0, 0.0, 0.0, 0.0, 2.0, -1.0, 0.0, 0.0, 2.0, 0.0, 0.0, 2.0, 0.0, 2.0] };


    let res = format!(r"
chol: -----------------
A = L * L^T             {}
L                       {}
L * L^T                 {}
A^(-1) = L' * L'^T      {}
A * A^(-1)              {}
A = L * L^T             {}
L                       {}
L * L^T                 {}
A^(-1) = L' * L'^T      {}
A * A^(-1)              {}
det this                {}
",
    ch3,
    ch3.choldc(),
    ch3.choldc() * ch3.choldc().tr(),
    ch3.cholinv(),
    ch3.clone() * ch3.cholinv(),
    ch5,
    ch5.choldc(),
    ch5.choldc() * ch5.choldc().tr(),
    ch5.cholinv(),
    ch5.clone() * ch5.cholinv(),
    ch5.det(),
    );
    // print!("{}", res);

    // let v3 = Vec3 { v: [10.0,11.0,12.0] };
    // res.push_str(&format!("Vec *. Vec = {}\n", (v3.clone() * v3).to_string()));
    // println!("Vec * Vec = {}\n", v3.clone() * v3);
    // println!("Cov *. Cov = {}\n", (Cov3 {v: [1.0,2.0,3.0,4.0,5.0,6.0]}) * (Cov3 {v: [0.0,0.0,1.0,1.0,0.0,0.0]}));

    assert!(true, "test failed with '{}'", res);
}


