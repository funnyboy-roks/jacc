use anyhow::ensure;
use std::collections::HashMap;

use super::{AstEvaluator, AstStatement};

pub type FnMap =
    HashMap<String, Box<dyn Fn(&AstEvaluator, Vec<AstStatement>) -> anyhow::Result<f64>>>;

// This function signature is awful
pub fn default_functions() -> FnMap {
    let mut map: FnMap = HashMap::new();

    macro_rules! def_fn {
        ($name: ident = |$expr: ident, $args: ident| $body: block) => {
            map.insert(stringify!($name).into(), Box::new(|$expr, $args| $body));
        };
        ($name: ident => $f: ident) => {
            def_fn!(
                $name = |expr, args| {
                    ensure!(
                        args.len() == 1,
                        concat!("Expected 1 arg: `", stringify!($name), "(x)`")
                    );
                    Ok(expr.eval(&args[0])?.$f())
                }
            )
        };
        ($name: ident) => {
            def_fn!($name => $name)
        };
    }

    // logaritms
    def_fn!(ln);
    def_fn!(log => log10);

    // trig
    def_fn!(sin);
    def_fn!(cos);
    def_fn!(tan);

    def_fn!(asin);
    def_fn!(acos);
    def_fn!(atan);

    def_fn!(sinh);
    def_fn!(cosh);
    def_fn!(tanh);

    def_fn!(asinh);
    def_fn!(acosh);
    def_fn!(atanh);

    // misc
    def_fn!(sqrt);
    def_fn!(cbrt);
    def_fn!(floor);
    def_fn!(ceil);
    def_fn!(round);
    def_fn!(abs);

    def_fn!(
        gcd = |expr, args| {
            let mut args = args.iter().map(|a| expr.eval(a));
            let mut r = args.next().unwrap()?;
            for n in args {
                r = gcd(r, n?);
            }
            Ok(r)
        }
    );

    map
}

fn gcd(mut a: f64, mut b: f64) -> f64 {
    if a == 0.0 {
        return b;
    }
    while b > 0.0 {
        let r = a % b;
        a = b;
        b = r;
    }
    return a;
}
