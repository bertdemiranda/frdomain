#![allow(dead_code)]

pub type GetLens<O, V> = fn(o: &O) -> V;
pub type SetLens<O, V> = fn(o: &O, v: &V) -> O;

pub struct Lens<O, V> {
    pub get: GetLens<O, V>,
    pub set: SetLens<O, V>
}


// struct Compose<Outer, Inner, Value> {}

// fn compose_get<Outer, Inner, Value>(outer_g: GetLens<Outer, Inner>,
//                                     inner_g: GetLens<Inner, value>)
//                                     -> GetLens<Outer, Value> {
//     fn get_lens(o: Outer) -> GetLens<Outer, Value> {
//         inner_g(outer_g(o))
//     }
//     get_lens
// }

// fn compose_set<Outer, Inner, Value>(outer_g: GetLens<Outer, Inner>,
//                                     outer_s: SetLens<Outer, Inner>,
//                                     inner_s: SetLens<Inner, value>)
//                                     -> SetLens<Outer, Value> {
//     fn set_lens(o: Outer, v: Value) -> SetLens<Outer, Value> {
//         outer_s(o, inner_s(outer_g(o), v))
//     }
//     set_lens
// }
