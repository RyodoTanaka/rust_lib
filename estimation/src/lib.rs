#[macro_use]
extern crate cpython;
use cpython::{Python, PyResult};


fn estimate(_py: Python, node: Vec<f32>, collision_list: Vec<Vec<f32>>,
            beta: f32, threshold: f32) -> PyResult<bool> {

    let th_pow = threshold.powf(2.0);
    if collision_list.len()!=0 {
        let mut total_prob = 0.0;
        for col in collision_list {
            let mut prob = 0.0;
            if node == col {return Ok(true)}
            else {                
                for i in 0..node.len() { prob += (node[i]-col[i]).powf(2.0) }
                prob = th_pow-prob*beta;
                if prob < 0.0 {prob = 0.0}
            }
            total_prob += prob;
            if total_prob>th_pow {return Ok(true)}
        }
    }
    
    Ok(false)
}


py_module_initializer!(rustestimate, initrustestimate, PyInit_rustestimate, |py, m | {
    try!(m.add(py, "estimate", py_fn!(py, estimate(node: Vec<f32>,
                                                   collision_list: Vec<Vec<f32>>,
                                                   beta: f32, threshold: f32))));
   
    Ok(())
});
