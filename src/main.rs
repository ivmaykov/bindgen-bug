use bindgen_bug::{
    AliasOfStructContainingUnion, SimpleStruct, SimpleStructAlias, StructContainingUnion, Union,
    UnionAlias,
};

fn main() {
    let u = Union { bytes: [0; 4] };
    println!("The union is {:?}", u);

    let ua = UnionAlias(u.clone());

    // This breaks when default_alias_style(bindgen::AliasVariation::NewTypeDeref) is set
    // in the bindgen config, because while Union implements Debug, the newtype wrapper
    // UnionAlias does not.
    //
    // This is not the same behavior as what we observe with simple structs, where everything works.
    // println!("The union alias is {:?}", ua);

    let ss = SimpleStruct { i: 1 };
    println!("The simple struct is {:?}", ss);

    let ssa = SimpleStructAlias(ss.clone());
    println!("The simple struct alias is {:?}", ssa);

    let scu = StructContainingUnion { u: u.clone() };
    println!("The struct containing union is {:?}", scu);

    let aoscu = AliasOfStructContainingUnion(scu.clone());

    // However, we do see the same issue with newtype wrappers for structs which contain unions.
    // println!("The alias of a struct containing union is {:?}", aoscu);
}
