#[cfg(test)]
mod tests {
    fn dotest(s: &str, exp: &str) -> () {
        println!("s:{}", s);
        let ans = by_state(s);
        println!("actual:\n{}", ans);
        println!("expect:\n{}", exp);
        println!("{}", ans == exp);
        assert_eq!(ans, exp);
        println!("{}", "-");
    }

    #[test]
    fn provided() {
        let ad="John Pulsett, 321 King Street, Palmouth MA\nAlisa Gord, 22 Prin Broadway, Georges VA\nOreste Thulas, 11354 East Bridge Road, Pensa OK\nPerry Falpas, 420 Land Road, Beaver Halls PA\nErica Adamson, 200 Station Road, Westbury MA\nPaulo Sims, 8A River Street, Richmond VA\nAnn Wildon, 334 Shore Parkway, Hill View CA\nAl Carpenter, 730 3rd Street, Boston MA";
        let adsol = "California\n..... Ann Wildon 334 Shore Parkway Hill View California\n Massachusetts\n..... Al Carpenter 730 3rd Street Boston Massachusetts\n..... Erica Adamson 200 Station Road Westbury Massachusetts\n..... John Pulsett 321 King Street Palmouth Massachusetts\n Oklahoma\n..... Oreste Thulas 11354 East Bridge Road Pensa Oklahoma\n Pennsylvania\n..... Perry Falpas 420 Land Road Beaver Halls Pennsylvania\n Virginia\n..... Alisa Gord 22 Prin Broadway Georges Virginia\n..... Paulo Sims 8A River Street Richmond Virginia";
        dotest(ad, adsol);
        
        let ad3="John Daggett, 341 King Road, Plymouth MA\nAlice Ford, 22 East Broadway, Richmond VA\nSal Carpenter, 73 6th Street, Boston MA";
        let ad3sol="Massachusetts\n..... John Daggett 341 King Road Plymouth Massachusetts\n..... Sal Carpenter 73 6th Street Boston Massachusetts\n Virginia\n..... Alice Ford 22 East Broadway Richmond Virginia";
        dotest(ad3, ad3sol);
    }
}
