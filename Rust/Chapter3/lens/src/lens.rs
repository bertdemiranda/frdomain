#![allow(dead_code)]

pub type GetLens<O, V> = Box<dyn Fn(&O) -> V>;
pub type SetLens<O, V> = Box<dyn Fn(&O, &V) -> O>;

pub struct Lens<O, V> {
    pub get: GetLens<O, V>,
    pub set: SetLens<O, V>
}

pub fn compose_get<Outer, Inner, Value>(
                    o: &Outer,
                    outer: &Lens<Outer, Inner>, 
                    inner: &Lens<Inner, Value>) 
                    -> Value {
    let outer_get = &outer.get;
    let inner_get = &inner.get;
    inner_get(&outer_get(o))
}

pub fn compose_set<Outer, Inner, Value>(
                    o: &Outer,
                    v: &Value,
                    outer: &Lens<Outer, Inner>, 
                    inner: &Lens<Inner, Value>)
                    -> Outer {
    let outer_get = &outer.get;
    let outer_set = &outer.set;
    let inner_set = &inner.set;
    outer_set(o, &inner_set(&outer_get(o), v))
}

macro_rules! lens {
    ($name:ident, $fld:ident, $o:ident, $v:ident) => {
        pub fn $name() -> Lens<$o, $v> {
            fn get_lens(o: &$o) -> String { 
                o.$fld.clone()
            }
            fn set_lens(o: &$o, v: &$v) -> $o {
                $o{$fld: v.clone(), ..o.clone()}
            }
            Lens {
                get: Box::new(get_lens),
                set: Box::new(set_lens)
            }
        }
    };
}

//---------------------------------------------------
//- This does not compile.
//- Not possible (yet) to get the lifetimes correct!
//--------------------------------------------------- 
// fn compose<'a, Outer, Inner, Value>(outer: &'a Lens<Outer, Inner>, 
//                                     inner: &'a Lens<Inner, Value>) 
//                                     -> &'a Lens<Outer, Value> {
//     let get_lens = |o: &Outer| {
//         let outer_get = outer.get;
//         let inner_get = inner.get;
//         inner_get(&outer_get(&o))
//     };
//     let set_lens = |o: &Outer, v: &Value| {
//         let outer_get = outer.get;
//         let outer_set = outer.set;
//         let inner_set = inner.set;
//         outer_set(&o, &inner_set(&outer_get(&o), &v))
//     };
//     &Lens {
//         get: Box::new(get_lens),
//         set: Box::new(set_lens)
//     }
// }

