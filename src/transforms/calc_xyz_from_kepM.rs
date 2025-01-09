use crate::transforms::calc_conic_anomaly_from_mean_anomaly;
use crate::transforms::calc_true_anomaly_from_conic_anomaly;
use crate::errors::OrbitError;

use nalgebra::Vector3;

#[allow(non_snake_case)]
pub fn calc_xyz_from_kepM(a: f64, e: f64, inc: f64, arg: f64, node: f64, M: f64, mu: f64) -> Result<(Vector3<f64>, Vector3<f64>), OrbitError>  {

    let E = calc_conic_anomaly_from_mean_anomaly(e, M)?;

    let ox;
    let oy;
    let vox;
    let voy;

    if e < 1.0 {        
    
        let cosE = E.cos();
        let omece = 1.0 - e * cosE;
        let f = calc_true_anomaly_from_conic_anomaly(e, E)?;
        let r = a * omece;
        let c = (mu * a).sqrt() / r;
    
        ox = r * f.cos();
        oy = r * f.sin();
        vox = - c * E.sin();
        voy = c * (1.0 - e * e).sqrt() * cosE;
    
    } else {
        
        let f = calc_true_anomaly_from_conic_anomaly(e, E)?;
        let r = a * (1.0 - e*e) / (1.0 + e * f.cos());
        let c = (-mu * a).sqrt() / r;
    
        ox = r * f.cos();
        oy = r * f.sin();
        vox = - c * E.sinh();
        voy = c * (e*e - 1.0).sqrt() * E.cosh();
    
    }

    let sa = arg.sin();
    let ca = arg.cos();

    let si = inc.sin();
    let ci = inc.cos();
    
    let sn = node.sin();
    let cn = node.cos();

    let c1 = ca * cn - sa * sn * ci;
    let c2 = sa * cn + ca * sn * ci;
    let c3 = ca * sn + sa * cn * ci;
    let c4 = ca * cn * ci - sa * sn;
    let c5 = sa * si;
    let c6 = ca * si;

    let position = Vector3::new(ox * c1 - oy * c2, ox * c3 + oy * c4, ox * c5 + oy * c6);
    let velocity = Vector3::new(vox * c1 - voy * c2, vox * c3 + voy * c4, vox * c5 + voy * c6);

    Ok((position, velocity))
}