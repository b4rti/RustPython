//! Random module.

use rand::distributions::{Distribution, Normal};

use crate::function::PyFuncArgs;
use crate::obj::objfloat;
use crate::pyobject::{PyContext, PyObjectRef, PyResult, TypeProtocol};
use crate::vm::VirtualMachine;

pub fn make_module(ctx: &PyContext) -> PyObjectRef {
    py_module!(ctx, "random", {
        "guass" => ctx.new_rustfunc(random_gauss),
        "normalvariate" => ctx.new_rustfunc(random_normalvariate),
        "random" => ctx.new_rustfunc(random_random),
        // "weibull", ctx.new_rustfunc(random_weibullvariate),
    })
}

fn random_gauss(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    // TODO: is this the same?
    random_normalvariate(vm, args)
}

fn random_normalvariate(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!(
        vm,
        args,
        required = [
            (mu, Some(vm.ctx.float_type())),
            (sigma, Some(vm.ctx.float_type()))
        ]
    );
    let mu = objfloat::get_value(mu);
    let sigma = objfloat::get_value(sigma);
    let normal = Normal::new(mu, sigma);
    let value = normal.sample(&mut rand::thread_rng());
    let py_value = vm.ctx.new_float(value);
    Ok(py_value)
}

fn random_random(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!(vm, args);
    let value = rand::random::<f64>();
    let py_value = vm.ctx.new_float(value);
    Ok(py_value)
}

/*
 * TODO: enable this function:
fn random_weibullvariate(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!(vm, args, required = [(alpha, Some(vm.ctx.float_type())), (beta, Some(vm.ctx.float_type()))]);
    let alpha = objfloat::get_value(alpha);
    let beta = objfloat::get_value(beta);
    let weibull = Weibull::new(alpha, beta);
    let value = weibull.sample(&mut rand::thread_rng());
    let py_value = vm.ctx.new_float(value);
    Ok(py_value)
}
*/
