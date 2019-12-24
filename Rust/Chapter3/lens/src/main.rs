#![allow(dead_code)]

mod lens;

mod address {
    use crate::lens::*;

    #[derive(Clone, Debug)]
    pub struct Address {
        pub no:     String,
        pub street: String,
        pub city:   String,
        pub state:  String,
        pub zip:    String
    }

    impl Address {
        pub fn new(no: &str, street: &str, city: &str, state: &str, zip: &str) -> Address {
            Address{
                no:     String::from(no), 
                street: String::from(street), 
                city:   String::from(city), 
                state:  String::from(state), 
                zip:    String::from(zip)
            }
        }
    }

    pub fn no_lens() -> Lens<Address, String> {
        fn get_lens(o: &Address) -> String { 
            o.no.clone()
        }
        fn set_lens(o: &Address, v: &String) -> Address {
            Address{no: v.clone(), ..o.clone()}
        }
        Lens {
            get: Box::new(get_lens),
            set: Box::new(set_lens)
        }
    }
}

mod customer {
    use crate::lens::*;
    use crate::address::Address;

    #[derive(Clone, Debug)]
    pub struct Customer {
        pub id:      i64,
        pub name:    String, 
        pub address: Address
    }

    pub fn address_lens() -> Lens<Customer, Address> {
        fn get_lens(o: &Customer) -> Address {
            o.address.clone() 
        }
        fn set_lens(o: &Customer, v: &Address) -> Customer { 
            Customer{address: v.clone(), ..o.clone()}
        }
        Lens {
            get: Box::new(get_lens),
            set: Box::new(set_lens)
        }
    }
}

fn main() {
    println!("Are all tests Ok?");
}

#[cfg(test)]
mod tests {
    use crate::*;
    use crate::lens::{compose_get, compose_set};
    use crate::address::*;
    use crate::customer::*;
    
    #[test]
    fn get_address_no() {
        let a  = Address::new("B-12", "Monroe Street", "Denver", "CO", "80231");
        let gl = address::no_lens().get;
        assert!(gl(&a) == "B-12");
    }

    #[test]
    fn set_address_no() {
        let a1 = Address::new("B-12", "Monroe Street", "Denver", "CO", "80231");
        let sl = address::no_lens().set;
        let a2 = sl(&a1, &String::from("Z-99"));
        assert!(a2.no == "Z-99");
    }

    #[test]
    fn get_customer_address() {
        let c = Customer {
            id: 1,
            name: String::from("Cust1"),
            address: Address::new("B-12", "Monroe Street", "Denver", "CO", "80231")
        };
        let cgl = customer::address_lens().get;
        let a   = cgl(&c);
        assert!(a.no == "B-12");
    }

    #[test]
    fn compo_get_customer_address_no() {
        let c = Customer {
            id: 1,
            name: String::from("Cust1"),
            address: Address::new("B-12", "Monroe Street", "Denver", "CO", "80231")
        };
        let cal  = customer::address_lens();
        let anol = address::no_lens();
        let get_cust_addr_no = |c| compose_get(&c, &cal, &anol);
        let no   = get_cust_addr_no(c);
        assert!(no == "B-12");
    }

    #[test]
    fn set_customer_address_no() {
        let c1 = Customer {
            id: 1,
            name: String::from("Cust1"),
            address: Address::new("B-12", "Monroe Street", "Denver", "CO", "80231")
        };
        let cgl = customer::address_lens().get;
        let csl = customer::address_lens().set;
        let asl = address::no_lens().set;
        let c2  = csl(&c1, &asl(&cgl(&c1), &String::from("A-1")));
        assert!(c2.address.no == "A-1");
    }

    #[test]
    fn compo_set_customer_address_no() {
        let c1 = Customer {
            id: 1,
            name: String::from("Cust1"),
            address: Address::new("B-12", "Monroe Street", "Denver", "CO", "80231")
        };
        let cal  = customer::address_lens();
        let anol = address::no_lens();
        let set_cust_addr_no = |c, no| compose_set(c, no, &cal, &anol);
        let c2   = set_cust_addr_no(&c1, &String::from("A-1"));
        assert!(c2.address.no == "A-1");
    }
}