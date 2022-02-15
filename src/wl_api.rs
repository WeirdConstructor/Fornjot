use wlambda;
use wlambda::vval::VVal;
use wlambda::vval;
use wlambda::*;

use std::rc::Rc;
use std::cell::RefCell;

use thiserror::Error;

use fj::*;

#[derive(Debug, Clone)]
pub enum AShape {
    Trans(fj::Transform),
    Diff2d(fj::Difference2d),
    Circle(fj::Circle),
    Sketch(fj::Sketch),
    Sweep(fj::Sweep),
    Union(fj::Union),
    Diff(fj::Difference),
}

impl AShape {
    pub fn to_shape3d(&self) -> Option<fj::Shape3d> {
        match self {
              AShape::Circle(_)
            | AShape::Sketch(_)
            | AShape::Diff2d(_) => Some(fj::Sweep {
                shape: self.to_shape2d()?, length: 1.0
            }.into()),
            AShape::Sweep(sw) => Some(sw.clone().into()),
            AShape::Trans(t)  => Some(t.clone().into()),
            AShape::Union(u)  => Some(u.clone().into()),
            AShape::Diff(d)   => Some(d.clone().into()),
        }
    }

    pub fn to_shape2d(&self) -> Option<fj::Shape2d> {
        match self {
            AShape::Circle(c) => Some(c.clone().into()),
            AShape::Sketch(s) => Some(s.clone().into()),
            AShape::Diff2d(d) => Some(d.clone().into()),
            AShape::Sweep(sw) => Some(sw.shape.clone().into()),
            AShape::Trans(_)  => None,
            AShape::Union(_)  => None,
            AShape::Diff(_)   => None,
        }
    }

    pub fn to_shape(&self) -> fj::Shape {
        match self {
            AShape::Circle(c) => c.clone().into(),
            AShape::Sketch(s) => s.clone().into(),
            AShape::Diff2d(d) => d.clone().into(),
            AShape::Sweep(sw) => sw.clone().into(),
            AShape::Trans(t)  => t.clone().into(),
            AShape::Union(u)  => u.clone().into(),
            AShape::Diff(d)   => d.clone().into(),
        }
    }
}

#[derive(Clone)]
struct VVShape {
    shape: AShape
}

impl VVShape {
    pub fn new(shape: AShape) -> Self {
        Self { shape }
    }
}

impl vval::VValUserData for VVShape {
    fn s(&self) -> String {
        format!("$<Shape:{:?}>", self.shape)
    }

    fn call_method(&self, key: &str, env: &mut Env)
        -> Result<VVal, StackAction>
    {
        Ok(VVal::None)
//        let args = env.argv_ref();
//
//        match key {
//            "s" => {
//                arg_chk!(args, 0, "atom.s[]");
//                Ok(VVal::new_str_mv(self.atom.s()))
//            },
//            "i" => {
//                arg_chk!(args, 0, "atom.i[]");
//                Ok(VVal::Int(self.atom.i()))
//            },
//            "f" => {
//                arg_chk!(args, 0, "atom.f[]");
//                Ok(VVal::Flt(self.atom.f() as f64))
//            },
//            "micro_sample" => {
//                arg_chk!(args, 0, "atom.micro_sample[]");
//
//                if let SAtom::MicroSample(ms) = &self.atom {
//                    let v = VVal::vec();
//                    for s in ms.iter() {
//                        v.push(VVal::Flt(*s as f64));
//                    }
//
//                    Ok(v)
//                } else {
//                    Ok(VVal::vec1(VVal::Flt(self.atom.f() as f64)))
//                }
//            },
//            "default_of" => {
//                arg_chk!(args, 0, "atom.default_of[]");
//
//                Ok(VVal::Usr(Box::new(VVShape {
//                    atom: self.atom.default_of()
//                })))
//            },
//            "is_continous" => {
//                arg_chk!(args, 0, "atom.is_continous[]");
//
//                Ok(VVal::Bol(self.atom.is_continous()))
//            },
//            "type_str" => {
//                arg_chk!(args, 0, "atom.type_str[]");
//
//                Ok(VVal::new_sym(self.atom.type_str()))
//            },
//            _ => Ok(VVal::err_msg(&format!("Unknown method called: {}", key))),
//        }
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any { self }
    fn clone_ud(&self) -> Box<dyn vval::VValUserData> { Box::new(self.clone()) }
}

pub fn shape2vv(shape: AShape) -> VVal {
    VVal::Usr(Box::new(VVShape::new(shape)))
}

pub fn vv2shape(mut v: VVal) -> Result<fj::Shape, WLError> {
    if let Some(shp) = v.with_usr_ref(|vvshp: &mut VVShape| vvshp.shape.clone()) {
        Ok(shp.to_shape())
    } else {
        Err(WLError::NoShape)
    }
}

pub fn vv2shape3d(mut v: VVal) -> Result<fj::Shape3d, WLError> {
    if let Some(shp) = v.with_usr_ref(|vvshp: &mut VVShape| vvshp.shape.clone()) {
        if let Some(shp3d) = shp.to_shape3d() {
            Ok(shp3d)
        } else {
            Err(WLError::NoShape3d)
        }
    } else {
        Err(WLError::NoShape3d)
    }
}

pub fn vv2shape2d(mut v: VVal) -> Result<fj::Shape2d, WLError> {
    if let Some(shp) = v.with_usr_ref(|vvshp: &mut VVShape| vvshp.shape.clone()) {
        if let Some(shp2d) = shp.to_shape2d() {
            Ok(shp2d)
        } else {
            Err(WLError::NoShape2d)
        }
    } else {
        Err(WLError::NoShape2d)
    }
}

#[derive(Debug, Clone, Error)]
pub enum WLError {
    NoShape,
    NoShape2d,
    NoShape3d,
    ExecError(String),
}

impl From<WLError> for StackAction {
    fn from(e: WLError) -> StackAction {
        StackAction::panic_msg(format!("WLShpError: {:?}", e))
    }
}

impl std::fmt::Display for WLError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn run_wl(file: String) -> Result<Shape, WLError> {
    use wlambda::{GlobalEnv, Env};
    let global_env = GlobalEnv::new_default();

    let argv = VVal::vec();
    for e in std::env::args() {
        argv.push(VVal::new_str_mv(e.to_string()));
    }
    global_env.borrow_mut().set_var("ARGV", &argv);

    let mut st = wlambda::SymbolTable::new();

    st.fun(
        "diff", move |env: &mut Env, _argc: usize| {
            let a = vv2shape3d(env.arg(0))?;
            let b = vv2shape3d(env.arg(1))?;
            Ok(shape2vv(AShape::Diff(fj::Difference { a, b })))
        }, Some(2), Some(2), false);

    st.fun(
        "union", move |env: &mut Env, _argc: usize| {
            let a = vv2shape3d(env.arg(0))?;
            let b = vv2shape3d(env.arg(1))?;
            Ok(shape2vv(AShape::Union(fj::Union { a, b })))
        }, Some(2), Some(2), false);

    st.fun(
        "trans", move |env: &mut Env, _argc: usize| {
            let shape = vv2shape3d(env.arg(0))?;
            let offs  = env.arg(1);
            let axis  = env.arg(2);
            let angle = env.arg(3).f();
            let axis =
                if axis.is_none() {
                    [1.0, 0.0, 0.0]
                } else {
                    [
                        axis.v_f(0),
                        axis.v_f(1),
                        axis.v_f(2),
                    ]
                };
            Ok(shape2vv(AShape::Trans(fj::Transform {
                shape,
                axis,
                angle,
                offset: [
                    offs.v_f(0),
                    offs.v_f(1),
                    offs.v_f(2),
                ],
            })))
        }, Some(2), Some(4), false);

    st.fun(
        "sketch", move |env: &mut Env, _argc: usize| {
            let mut pts = vec![];
            for (v, _) in env.arg(0).iter() {
                pts.push([v.v_f(0), v.v_f(1)]);
            }
            Ok(shape2vv(AShape::Sketch(fj::Sketch::from_points(pts))))
        }, Some(1), Some(1), false);

    st.fun(
        "circle", move |env: &mut Env, _argc: usize| {
            Ok(shape2vv(AShape::Circle(fj::Circle { radius: env.arg(0).f() })))
        }, Some(1), Some(1), false);

    st.fun(
        "diff2d", move |env: &mut Env, _argc: usize| {
            let a = vv2shape2d(env.arg(0))?;
            let b = vv2shape2d(env.arg(1))?;

            Ok(shape2vv(AShape::Diff2d(fj::Difference2d { a, b })))
        }, Some(2), Some(2), false);

    st.fun(
        "sweep", move |env: &mut Env, _argc: usize| {
            let shape  = vv2shape2d(env.arg(0))?;
            let length = env.arg(1).f();

            Ok(shape2vv(AShape::Sweep(fj::Sweep { shape, length })))
        }, Some(2), Some(2), false);

    global_env.borrow_mut().set_module("fj", st);

    let ctx = wlambda::EvalContext::new(global_env.clone());
    let ctx = Rc::new(RefCell::new(ctx));

    let mut ctx = ctx.borrow_mut();

    match ctx.eval_file(&file) {
        Ok(v) => {
            vv2shape(v)
        },
        Err(e) => {
            Err(WLError::ExecError(format!("WLambda Error: {}", e)))
        }
    }
}
