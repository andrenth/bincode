use super::{Encode, Encoder};
use crate::error::EncodeError;

impl<A> Encode for (A,)
where
    A: Encode,
{
    fn encode<_E: Encoder>(&self, mut encoder: _E) -> Result<(), EncodeError> {
        self.0.encode(&mut encoder)?;
        Ok(())
    }
}

impl<A, B> Encode for (A, B)
where
    A: Encode,
    B: Encode,
{
    fn encode<_E: Encoder>(&self, mut encoder: _E) -> Result<(), EncodeError> {
        self.0.encode(&mut encoder)?;
        self.1.encode(&mut encoder)?;
        Ok(())
    }
}

impl<A, B, C> Encode for (A, B, C)
where
    A: Encode,
    B: Encode,
    C: Encode,
{
    fn encode<_E: Encoder>(&self, mut encoder: _E) -> Result<(), EncodeError> {
        self.0.encode(&mut encoder)?;
        self.1.encode(&mut encoder)?;
        self.2.encode(&mut encoder)?;
        Ok(())
    }
}

impl<A, B, C, D> Encode for (A, B, C, D)
where
    A: Encode,
    B: Encode,
    C: Encode,
    D: Encode,
{
    fn encode<_E: Encoder>(&self, mut encoder: _E) -> Result<(), EncodeError> {
        self.0.encode(&mut encoder)?;
        self.1.encode(&mut encoder)?;
        self.2.encode(&mut encoder)?;
        self.3.encode(&mut encoder)?;
        Ok(())
    }
}

impl<A, B, C, D, E> Encode for (A, B, C, D, E)
where
    A: Encode,
    B: Encode,
    C: Encode,
    D: Encode,
    E: Encode,
{
    fn encode<_E: Encoder>(&self, mut encoder: _E) -> Result<(), EncodeError> {
        self.0.encode(&mut encoder)?;
        self.1.encode(&mut encoder)?;
        self.2.encode(&mut encoder)?;
        self.3.encode(&mut encoder)?;
        self.4.encode(&mut encoder)?;
        Ok(())
    }
}

impl<A, B, C, D, E, F> Encode for (A, B, C, D, E, F)
where
    A: Encode,
    B: Encode,
    C: Encode,
    D: Encode,
    E: Encode,
    F: Encode,
{
    fn encode<_E: Encoder>(&self, mut encoder: _E) -> Result<(), EncodeError> {
        self.0.encode(&mut encoder)?;
        self.1.encode(&mut encoder)?;
        self.2.encode(&mut encoder)?;
        self.3.encode(&mut encoder)?;
        self.4.encode(&mut encoder)?;
        self.5.encode(&mut encoder)?;
        Ok(())
    }
}

impl<A, B, C, D, E, F, G> Encode for (A, B, C, D, E, F, G)
where
    A: Encode,
    B: Encode,
    C: Encode,
    D: Encode,
    E: Encode,
    F: Encode,
    G: Encode,
{
    fn encode<_E: Encoder>(&self, mut encoder: _E) -> Result<(), EncodeError> {
        self.0.encode(&mut encoder)?;
        self.1.encode(&mut encoder)?;
        self.2.encode(&mut encoder)?;
        self.3.encode(&mut encoder)?;
        self.4.encode(&mut encoder)?;
        self.5.encode(&mut encoder)?;
        self.6.encode(&mut encoder)?;
        Ok(())
    }
}

impl<A, B, C, D, E, F, G, H> Encode for (A, B, C, D, E, F, G, H)
where
    A: Encode,
    B: Encode,
    C: Encode,
    D: Encode,
    E: Encode,
    F: Encode,
    G: Encode,
    H: Encode,
{
    fn encode<_E: Encoder>(&self, mut encoder: _E) -> Result<(), EncodeError> {
        self.0.encode(&mut encoder)?;
        self.1.encode(&mut encoder)?;
        self.2.encode(&mut encoder)?;
        self.3.encode(&mut encoder)?;
        self.4.encode(&mut encoder)?;
        self.5.encode(&mut encoder)?;
        self.6.encode(&mut encoder)?;
        self.7.encode(&mut encoder)?;
        Ok(())
    }
}
