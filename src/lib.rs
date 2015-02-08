/*
 * Copyright (c) 2015 Brandon Sanderson
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
 * THE SOFTWARE.
 *
 */
#![feature(core)]

use std::fmt::{Debug, Formatter, Error};
use std::num::{SignedInt, Int, FromPrimitive};
use std::ops::{Add, Sub, Mul, Div};
use std::cmp::Ordering;

/**
 * Structure storing a number in a format similar
 * to scientific notation.
 *
 * Probably inefficient, but also probably quite
 * accurate.
 */
pub struct SciValue<BASEVAL,EXPSTORE:SignedInt>{
  base: BASEVAL,
  e_exp: EXPSTORE
}

impl<B:Int,E:SignedInt> SciValue<B,E> {
  pub fn wrap(val:B) -> SciValue<B,E> {
    SciValue{base: val, e_exp : <E as Int>::zero()}
  }

  pub fn wrap_with_exponent(val:B, exp:E) -> SciValue<B,E> {
    SciValue{base: val, e_exp: exp}
  }
}

impl<B:Int, E:SignedInt> SciValue<B,E> {
  pub fn pow(self, exp: E) -> SciValue<B,E>{
    let mut newbase = self.base.clone();

    //Using a range, multiple *exp* times -1
    //(Not *exp* times exactly as we already start
    // with base^1)
    for _ in range::<E>(<E as Int>::one(),exp) {
      newbase = newbase * self.base.clone();
    }
    SciValue{base: newbase, e_exp: self.e_exp * exp}
  }

}

impl<B:Int + FromPrimitive, E:SignedInt> SciValue<B,E> {
  pub fn reduce(&self) -> SciValue<B,E> {
    let mut new_base = self.base.clone();
    let mut new_exp  = self.e_exp;
    let type_b_0 = <B as Int>::zero();
    let type_b_10 = <B as FromPrimitive>::from_int(10is).expect("Couldn't get a 10 value");
    while new_base.clone() % type_b_10.clone() == type_b_0.clone() {
      new_base = new_base / type_b_10.clone();
      new_exp  = new_exp + <E as Int>::one();
    }
    SciValue::wrap_with_exponent(new_base, new_exp)
  }
}

impl<B: Int + Debug, E: SignedInt + Debug> Debug for SciValue<B,E> {
  fn fmt(&self, fmtr:&mut Formatter) -> Result<(),Error> {
    fmtr.write_str(format!("SciValue{}base : {:?}, e_exp : {:?}{}", "{", self.base, self.e_exp, "}").as_slice())
  }
}

impl<B,E> Clone for SciValue<B,E> where B: Int, E:SignedInt {
  fn clone(&self) -> SciValue<B,E> {
    SciValue::wrap_with_exponent(self.base, self.e_exp)
  }
}


impl<B:Int, E:SignedInt> PartialEq for SciValue<B,E>{
  fn eq(&self, rhs: &SciValue<B,E>) -> bool {
    return self.base == rhs.base && self.e_exp == rhs.e_exp;
  }
}

impl<B:Int, E:SignedInt> Eq for SciValue<B,E>{}

impl<B:Int, E:SignedInt> PartialOrd for SciValue<B,E>{
  fn partial_cmp(&self, other:&SciValue<B,E>) -> Option<Ordering>{
    match self.e_exp.partial_cmp(&other.e_exp) {
      Some(Ordering::Equal) => self.base.partial_cmp(&other.base),
      retval@Some(_)        => retval,
      None                  => None
    }
  }
}

impl<B:Int, E:SignedInt> Ord for SciValue<B,E>{
  fn cmp(&self, other:&SciValue<B,E>) -> Ordering {
    match self.e_exp.cmp(&other.e_exp) {
      Ordering::Equal => self.base.cmp(&other.base),
      retval          => retval,
    }
  }
}

impl<B:Int + FromPrimitive,E:SignedInt> Add for SciValue<B,E> {
  type Output = SciValue<B,E>;

  fn add(self, unmatched_rhs:SciValue<B,E>) -> SciValue<B,E> {
    let (lhs, rhs) = match_exponents(self, unmatched_rhs);
    SciValue{base: lhs.base + rhs.base, e_exp: lhs.e_exp}
  }
}

impl<B:Int + FromPrimitive + Debug, E:SignedInt + Debug> Sub for SciValue<B,E>{
  type Output = SciValue<B,E>;

  fn sub(self, unmatched_rhs:SciValue<B,E>) -> SciValue<B,E> {
    let (lhs, rhs) = match_exponents(self, unmatched_rhs);
    println!("Matched: {:?},{:?}", lhs, rhs);
    SciValue{base: lhs.base - rhs.base, e_exp: lhs.e_exp}
  }
}

impl<B:Int, E:SignedInt> Mul for SciValue<B,E> {
  type Output = SciValue<B,E>;

  fn mul(self, rhs:SciValue<B,E>) -> SciValue<B,E> {
    SciValue{base: self.base * rhs.base, e_exp: self.e_exp + rhs.e_exp}
  }
}

impl<B:Int + FromPrimitive, E:SignedInt> Div for SciValue<B,E> {
  type Output = SciValue<B,E>;

  fn div(mut self, rhs:SciValue<B,E>) -> SciValue<B,E> {
    let b_ten = <B as FromPrimitive>::from_int(10is).expect("Couldn't get a value of 10 for the base type");

    while self.base % rhs.base != (<B as Int>::zero()) &&
          self.base < (<B as Int>::max_value() / b_ten) {
      self.base = self.base * b_ten;
      self.e_exp = self.e_exp - <E as Int>::one();
    }
    SciValue{base: self.base / rhs.base, e_exp: self.e_exp - rhs.e_exp}
  }
}

fn match_exponents<B:Int + FromPrimitive,E:SignedInt>(lhs:SciValue<B,E>, rhs:SciValue<B,E>) -> (SciValue<B,E>, SciValue<B,E>) {
  if lhs.e_exp == rhs.e_exp {
    (lhs, rhs)
  }else if lhs.e_exp > rhs.e_exp {
    let (newrhs, newlhs) = match_exponents_rhs_greater(rhs, lhs);
    (newlhs, newrhs)
  }else{
    match_exponents_rhs_greater(lhs, rhs)
  }
}

fn match_exponents_rhs_greater<B:Int + FromPrimitive,E:SignedInt>(lhs:SciValue<B,E>, rhs:SciValue<B,E>) -> (SciValue<B,E>, SciValue<B,E>) {
    let extra_exp = rhs.e_exp - lhs.e_exp;

    let ten_to_pow : Option<B> = extra_exp.to_uint().and_then(|usz| <B as FromPrimitive>::from_int(10is.pow(usz)));
    let rhs_new_base           = rhs.base * ten_to_pow.expect("Couldn't convert exponent type to base type");

    (lhs, SciValue{base: rhs_new_base, e_exp: rhs.e_exp - extra_exp})
}

#[cfg(test)]
mod test{
  use std::num::SignedInt;
  use super::SciValue;
  use super::match_exponents;

  #[test]
  fn test_equals(){
    let v1 = SciValue::wrap_with_exponent(2us,2is);
    let v2 = SciValue::wrap_with_exponent(2us,2is);
    assert_eq!(v1,v1);
    assert_eq!(v2,v1);
  }

  #[test]
  fn test_not_equals(){
    let v1 = SciValue::wrap_with_exponent(2us,2is);
    let v2 = SciValue::wrap_with_exponent(2us,3is);
    let v3 = SciValue::wrap_with_exponent(3us,2is);
    let v4 = SciValue::wrap_with_exponent(3us,3is);

    assert!(v1 != v2);
    assert!(v1 != v3);
    assert!(v1 != v4);
  }

  #[test]
  fn test_exponent_matching() {
    let lhs          = SciValue::wrap_with_exponent(5us, 2is);
    let rhs          = SciValue::wrap_with_exponent(5us, 4is);
    let expected_rhs = SciValue::wrap_with_exponent(500us,2is);

    assert_eq!(match_exponents(lhs.clone(), rhs.clone()), (lhs.clone(), expected_rhs.clone()));
    assert_eq!(match_exponents(rhs.clone(), lhs.clone()), (expected_rhs.clone(), lhs.clone()));
    assert_eq!(match_exponents(lhs.clone(), lhs.clone()), (lhs.clone(), lhs.clone()));
  }

  #[test]
  fn test_simple_add() {
    let lhs = SciValue::wrap_with_exponent(5us, 2is);
    let rhs = SciValue::wrap_with_exponent(16us, 2is);

    assert_eq!(lhs + rhs, SciValue::wrap_with_exponent(21us, 2is));
  }

  #[test]
  fn test_add() {
    let lhs = SciValue::wrap_with_exponent(5us, 2is);
    let rhs = SciValue::wrap_with_exponent(21us,5is);

    assert_eq!(lhs + rhs, SciValue::wrap_with_exponent(21005us, 2is));
  }

  #[test]
  fn test_simple_sub() {
    let lhs = SciValue::wrap_with_exponent(5us, 2is);
    let rhs = SciValue::wrap_with_exponent(2us, 2is);

    assert_eq!(lhs - rhs, SciValue::wrap_with_exponent(3us, 2is));
  }

  #[test]
  fn test_sub() {
    let lhs = SciValue::wrap_with_exponent(-2is, 2is);
    let rhs = SciValue::wrap_with_exponent(1is, 1is);
    let v3  = SciValue::wrap_with_exponent(2is, 2is);

    assert_eq!(lhs.clone() - rhs.clone(), SciValue::wrap_with_exponent(-21is, 1is));
    assert_eq!(rhs.clone() - lhs, SciValue::wrap_with_exponent(21is, 1is));
    assert_eq!(rhs - v3, SciValue::wrap_with_exponent(-19is, 1is));
  }

  #[test]
  fn test_mul() {
    let lhs = SciValue::wrap_with_exponent(2, 1is);
    let rhs = SciValue::wrap_with_exponent(10, 2is);

    assert_eq!(lhs * rhs, SciValue::wrap_with_exponent(20, 3is));
  }

  #[test]
  fn test_simple_div(){
    let lhs = SciValue::wrap_with_exponent(10, 1is);
    let rhs = SciValue::wrap_with_exponent(2, 3is);

    assert_eq!(lhs / rhs, SciValue::wrap_with_exponent(5, -2is));
  }

  #[test]
  fn test_div(){
    let lhs = SciValue::wrap_with_exponent(1, 0is);
    let rhs = SciValue::wrap_with_exponent(2, 0is);

    assert_eq!(lhs / rhs, SciValue::wrap_with_exponent(5, -1is));
  }

  #[test]
  fn test_reduce(){
    let val1 = SciValue::wrap_with_exponent(2, 10is);
    assert_eq!(val1.reduce(), val1);

    let val2 = SciValue::wrap_with_exponent(200, 10is);
    assert_eq!(val2.reduce(), SciValue::wrap_with_exponent(2, 12is));
  }

  #[test]
  fn test_pow(){
    let val1 = SciValue::wrap_with_exponent(2, 0is);
    assert_eq!(val1.pow(4), SciValue::wrap(16));

    let val2 = SciValue::wrap_with_exponent(11, 2is);
    assert_eq!(val2.pow(4), SciValue::wrap_with_exponent(14641, 8is));
  }
}
