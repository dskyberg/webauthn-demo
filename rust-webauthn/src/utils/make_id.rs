use crate::errors::Error;
use ring::rand::SecureRandom;

pub fn make_id(size: usize) -> Result<Vec<u8>, Error> {
    let mut random: Vec<u8> = vec![0; size];
    let sr = ring::rand::SystemRandom::new();
    sr.fill(&mut random).map_err(|_| Error::GeneralError)?;
    Ok(random)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_32() {
        let v = make_id(32).expect("Oops");
        //dbg!(&v);
        assert!(v.len() == 32);
    }
}
