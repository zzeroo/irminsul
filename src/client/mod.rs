#[cfg(feature = "rtu")]
pub mod rtu;

#[cfg(feature = "tcp")]
pub mod tcp;

pub mod util;

use modbus_core::*;
use crate::slave::*;

use std::{
    io::{Error}
};

/// A transport independent asynchronous client trait.
pub trait Client: SlaveContext + Send {
    fn call<'a>(
        &'a mut self,
        request: Request,
    ) -> Result<Response, Error>;
}

/// An asynchronous Modbus reader.
pub trait Reader: Client {
    fn read_coils<'a>(
        &'a mut self,
        _: Address,
        _: Quantity,
    ) -> Result<Vec<Coil>, Error>;

    fn read_discrete_inputs<'a>(
        &'a mut self,
        _: Address,
        _: Quantity,
    ) -> Result<Vec<Coil>, Error>;

    fn read_input_registers<'a>(
        &'a mut self,
        _: Address,
        _: Quantity,
    ) -> Result<Vec<Word>, Error>;

    fn read_holding_registers<'a>(
        &'a mut self,
        _: Address,
        _: Quantity,
    ) -> Result<Vec<Word>, Error>;

    fn read_write_multiple_registers<'a>(
        &'a mut self,
        _: Address,
        _: Quantity,
        _: Address,
        _: &[Word],
    ) -> Result<Vec<Word>, Error>;
}