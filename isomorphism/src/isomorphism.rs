#![allow(dead_code)]

/// so, when are two type, `a` and `b`, considered equal?
/// a definition might be, it is possible to go from `a` to `b`,
/// and from `b` to `a`.
/// Going a roundway trip should leave you the same value.
/// Unfortunately it is virtually impossible to test this.
/// This is called ISO.
pub enum Void { }

impl PartialEq for Void {
    fn eq(&self, _: &Void) -> bool {
        true
    }
}

pub fn absurd(_: Void) -> ! {
    panic!("You must be kidding! Where did you find that void instance?");
}

pub type ISO<A: 'static, B: 'static> = (Box<Fn(A) -> B>, Box<Fn(B) -> A>);

pub fn iso<A: 'static, B: 'static, F1, F2>(a: F1, b: F2) -> ISO<A, B>
    where F1: 'static + Fn(A) -> B,
          F2: 'static + Fn(B) -> A,
{
    (Box::new(a), Box::new(b))
}

/// given ISO a b, we can go from a to b
pub fn sub_st_l<A, B>(iso: ISO<A, B>) -> Box<Fn(A) -> B> { iso.0 }

/// and vice versa
pub fn sub_st_r<A, B>(iso: ISO<A, B>) -> Box<Fn(B) -> A> { iso.1 }

/// There can be more than one ISO a b
pub fn iso_bool() -> ISO<bool, bool> {
    iso(|a| a, |b| b)
}

pub fn iso_bool_not() -> ISO<bool, bool> {
    iso(|a: bool| !a, |b| !b)
}

/// isomorphism is reflexive
pub fn refl<A: 'static>() -> ISO<A, A> {
    iso(|a| a, |a| a)
}

/// isomorphism is symmetric
pub fn symm<A: 'static, B: 'static>(i: ISO<A, B>) -> ISO<B, A> {
    (sub_st_r(i), sub_st_l(i))
}

/// isomorphism is transitive
pub fn trans<A: 'static, B: 'static, C: 'static>
    (ab: ISO<A, B>, bc: ISO<B, C>) -> ISO<A, C> {
        iso(|a| (*bc.0)((*ab.0)(a)), |c| (*ab.1)((*bc.1)(c)))
    }

/// we can combine isomorphism
pub fn iso_tuple<A: 'static, B: 'static, C: 'static, D: 'static>
    (ab: ISO<A, B>, cd: ISO<C, D>) -> ISO<(A, C), (B, D)> {
        iso(|(a, c)| ((*ab.0)(a), (*cd.0)(c)), |(b, d)| ((*ab.1)(b), (*cd.1)(d)))
    }

pub fn iso_vec<A: 'static, B: 'static>(i: ISO<A, B>) -> ISO<Vec<A>, Vec<B>> {
    iso(
        |av| av.map(*i.0).collect(),
        |bv| bv.map(*i.1).collect())
}

pub fn iso_option<A: 'static, B: 'static>
    (i: ISO<A, B>) -> ISO<Option<A>, Option<B>> {
        iso(
            |ao| ao.map(*i.0),
            |bo| bo.map(*i.1))
    }

pub fn iso_result<A: 'static, B: 'static, C: 'static, D: 'static>
    (ab: ISO<A, B>, cd: ISO<C, D>) -> ISO<Result<A, C>, Result<B, D>> {
        iso(|ac_res| ac_res.map(*ab.0).map_err(*cd.0),
            |bd_res| bd_res.map(*ab.1).map_err(*cd.1))
    }

/// Going another way is hard (and is generally impossible)
/// Remember, for all valid ISO, converting and converting back
/// is the same as the original value.
/// You need this to prove some case are impossible.
pub fn iso_un_option<A: 'static, B: 'static>
    (i: ISO<Option<A>, Option<B>>) -> ISO<A, B> {
        iso(|a| (*i.0)(a).unwrap(), |b| (*i.1)(b).unwrap())
    }

/// inf + 0 = inf + 1
pub fn iso_eu() -> ISO<Result<Vec<()>, ()>, Result<Vec<()>, Void>> {
    iso(|res_v_empty| res_v_empty.map_err(|e| Void),
        |res_v_void| res_v_void.map_err(absurd))
}

pub type IsoFL<A, B, C, D> = Box<FnOnce(Box<Fn(A) -> C>) -> Box<FnOnce(B) -> D>>;
pub type IsoFR<A, B, C, D> = Box<FnOnce(Box<Fn(B) -> D>) -> Box<FnOnce(A) -> C>>;
pub type IsoF<A, B, C, D> = (IsoFL<A, B, C, D>, IsoFR<A, B, C, D>);

/// translator note:
/// FnBox is not yet supported, we can only return an uncallable
/// Box<FnOnce> (RetFunc). You should return the function with
/// correct type, which will be checked by the tests.
/// The type annotation is shown above. You need you return something like
/// (Box::new(...), Box::new(...))
pub fn iso_func<A: 'static, B: 'static, C: 'static, D: 'static>
    (ab: ISO<A, B>, cd: ISO<C, D>) -> IsoF<A, B, C, D> {
        (Box::new(|ac_fn_bx| Box::new(|b| (*cd.0)((*ac_fn_bx)((*ab.1)(b))))),
        Box::new(|bd_fn_bx| Box::new(|a| (*cd.1)((*bd_fn_bx)((*ab.0)(a))))))
    }

/// And we have isomorphism on isomorphism!
pub fn iso_symm<A: 'static, B: 'static>() -> ISO<ISO<A, B>, ISO<B, A>> {
    iso(symm, symm)
}
