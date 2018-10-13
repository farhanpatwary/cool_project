#[macro_use] extern crate vob;
use vob::Vob;


fn main() {
    //creating a vector of boolean values
    let v1 = [true,false,true,false,true,true];
    let mut v2 = Vob::new();
    //pushing items in vector v1 to vector of bits v2
    for mut x in 0..v1.len(){
        v2.push(v1[x]);
        println!("{:?}",v2[x]);
    }
    /*
    let v3 = vob![true,false];
    let v4 = vob![true,false];
    let c = assert_eq!(v3,v4);
    println!("are the expressions equal? {:?}", c);
    */


}
