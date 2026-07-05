/// Resource lock/unlock values.
#[derive(Debug, Copy, Clone)]
pub struct PduLockResourceMask {
    /// A ComLogicalLink requests exclusive privilege to modify physical
    /// ComParams for a physical resource. No other ComLogicalLink that
    /// is sharing the physical resource may attempt to modify the physical
    /// ComParams.
    pub lock_physical_com_params: bool,

    /// A ComLogicalLink requests exclusive privilege to transmit on a
    /// physical resource. No other ComLogicalLink that is sharing the
    /// physical resource may transmit any ComPrimitives on the physical
    /// resource. Only monitoring of the vehicle bus may be done by other
    /// ComLogicalLinks (receive only ComPrimitives).
    pub lock_physical_transmit_queue: bool,
}

impl PduLockResourceMask {
    pub(crate) fn get_pdu_data(&self) -> u32 {
        let bit_0 = u32::try_from(self.lock_physical_com_params).unwrap(); // SAFE : infallible
        let bit_1 = u32::try_from(self.lock_physical_transmit_queue).unwrap(); // SAFE : infallible

        bit_0 | (bit_1 << 1)
    }
}
