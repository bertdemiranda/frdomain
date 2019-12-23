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

