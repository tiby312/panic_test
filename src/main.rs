
#![feature(test)]

extern crate test;
use test::Bencher;


#[derive(Debug)]
enum SortErr<E>{
    IndexErr,
    UserErr(E)
}

use core::cmp::Ordering;
fn is_sorted_no_panic<T,E>(arr:&[T],func:impl Fn(&T,&T)->Result<Ordering,E>)->Result<bool,SortErr<E>>{
    for c in arr.chunks_exact(2){
        let a=c.get(0).ok_or(SortErr::IndexErr)?;
        let b=c.get(1).ok_or(SortErr::IndexErr)?;
        if let Ordering::Greater = func(a,b).map_err(|e|SortErr::UserErr(e))?{
            return Ok(false)
        }
    }
    Ok(true)
}

fn is_sorted<T>(arr:&[T],func:impl Fn(&T,&T)->Ordering)->bool{
    for c in arr.chunks_exact(2){
        let a=&c[0];
        let b=&c[1];
        if let Ordering::Greater = func(a,b){
            return false
        }
    }
    true
}



#[bench]
fn testb(b: &mut Bencher){
    let mut arr:Vec<_>=(0..10000).map(|a|a as f32).collect();
    arr[9999]=4.0;
    b.iter(|| {
        let k=is_sorted_no_panic(&arr,|a,b|a.partial_cmp(b).ok_or(())).unwrap();
        test::black_box(k);
    });
}

#[bench]
fn testa(b: &mut Bencher){
    let mut arr:Vec<_>=(0..10000).map(|a|a as f32).collect();
    arr[9999]=4.0;
    b.iter(|| {
        let k=is_sorted(&arr,|a,b|a.partial_cmp(b).unwrap());
        test::black_box(k);
    });
}



fn main(){
    let b:Box<[usize]>=core::default::Default::default();
    dbg!("length={}",b.len());
}