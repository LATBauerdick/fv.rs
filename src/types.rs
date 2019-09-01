
use crate::cov::*;

use std::fmt;

use std::convert::From;

pub type Number = f64;

#[derive(Debug, Clone)]
pub struct Prong<'a> {
                    pub n_prong: usize,
                    pub fit_vertex: XMeas,
                    pub fit_momenta: Vec<QMeas>,
                    pub fit_chi2s: Vec<Chi2>,
                    pub measurements: &'a VHMeas,
                }

type Chi2 = Number;

#[derive(Debug, Clone)]
pub struct XMeas(pub Vec3, pub Cov3);
impl XMeas {
    pub fn blowup(&self, scale: f64) -> XMeas {
        XMeas(self.0.clone(), self.1.scale_diag(scale))
    }
}
impl fmt::Display for XMeas {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let XMeas(v, cv) = self;
// -- return a string showing vertex position vector with errors

        let vv         = v.v;
        let x          = vv[0];
        let y          = vv[1];
        let z          = vv[2];
        let s2v: Vec<Number> = cv.diag().to_vec().into_iter().map(|x| x.sqrt()).collect();
        let dx         = s2v[0];
        let dy         = s2v[1];
        let dz         = s2v[2];
        fn f(x: &Number, dx: &Number) -> String {
            format!("{:7.2} +-{:7.2} ", *x, *dx)
        }
        write!(fmt, "(r,z) =({:7.2},{:7.2}), x y z = {}{}{}", f64::sqrt(x*x + y*y), z, f(&x, &dx), f(&y, &dy), f(&z, &dz))
    }
}

#[derive(Debug, Clone)]
pub struct HMeas(pub Vec5, pub Cov5, pub Number);
impl HMeas {

}
impl fmt::Display for HMeas {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let HMeas(h, ch, _w) = self;

        let hs = h.v;
        let sh: Vec<Number> = ch.diag().to_vec().into_iter().map(|x| x.sqrt()).collect();
        let s00 = format!("{:10.5} +-{:10.5}", hs[0], sh[0]);
        let s01 = format!("{:8.3} +-{:8.3}", hs[1], sh[1]);
        let s02 = format!("{:8.3} +-{:8.3}", hs[2], sh[2]);
        let s03 = format!("{:8.3} +-{:8.3}", hs[3], sh[3]);
        let s04 = format!("{:8.3} +-{:8.3}", hs[4], sh[4]);

        write!(fmt, "Helix ->{}{}{}{}{}", s00, s01, s02, s03, s04)
    }
}
#[derive(Debug, Clone)]
pub struct QMeas(pub Vec3, pub Cov3, pub Number);
impl QMeas {

}
impl From<&HMeas> for QMeas {
    fn from(hm: &HMeas) -> Self {
        let h = &hm.0;
        let ch = &hm.1;
        let w = &hm.2;
        let cq:Cov3 = ch.into();
        let q: Vec3 = [h.v[0],h.v[1],h.v[2]].into();
        QMeas(q,cq,*w)
    }
}
static MPI: f64 = 0.1395675_f64;
use std::f64::consts::PI;
impl fmt::Display for QMeas {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let QMeas(q, cq, w2pt) = self;
        fn f(s: &String, (x, dx): (&Number, &Number)) -> String {
            format!("{}{:8.3} +-{:8.3} ", s, *x, *dx)
        }
        let m           = MPI;
        let wp          = w2pt;
        let w           = q.v[0];
        let tl          = q.v[1];
        let psi0        = q.v[2];
        let pt          = wp / w.abs();
        let pz          = pt*tl;
        let psi         = psi0*180.0/PI;
        let e           = f64::sqrt(pt*pt  + pz*pz + m*m);
        let jj   = Jac34 { v : [ -wp/w/w, -wp/w/w*tl, 0.0, -(pz*pz + pt*pt)/w/e
                                , 0.0, wp/w, 0.0, pt*pt*tl/e
                                , 0.0, 0.0, 1.0, 0.0] };
        let cqp        = jj * cq.clone();
        let pp         = [pt, pz, psi, e];
        let dp: Vec<Number> = cqp.diag().to_vec().into_iter().map(|x| x.sqrt()).collect();
        let dpp        = [dp[0], dp[1], dp[2]*180.0/PI, dp[3]];
        let sp         = pp[..].iter().zip(&dpp).fold("".to_string(), |s, x|{ f(&s, x) });
        write!(fmt, "pt,pz,fi,E -> {}GeV", sp)
    }
}
#[derive(Debug, Clone)]
pub struct VHMeas {
    pub vertex:  XMeas,
    pub helices: Vec<HMeas>,
}

