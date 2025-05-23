// SPDX-License-Identifier: MIT OR Apache-2.0

//! USB 2 Host Controller protocol.

use uefi_macros::unsafe_protocol;
use uefi_raw::protocol::usb::host_controller::Usb2HostControllerProtocol;

use crate::{Result, StatusExt};

pub use uefi_raw::protocol::usb::host_controller::{
    HostControllerState, PortChangeStatus, PortFeature, PortStatus, ResetAttributes, Speed,
    UsbPortStatus, TransactionTranslator
};

/// The capabilities of a USB host controller.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Capabilities {
    /// The data transfer speed of the USB host controller.
    pub max_speed: Speed,
    /// The number of root hub ports.
    pub port_count: u8,
    /// Indicates whether the USB host controller is capable of 64-bit  memory
    /// addressing.
    pub is_64_bit_capable: bool,
}

/// USB2 Host Controller protocol.
#[derive(Debug)]
#[repr(transparent)]
#[unsafe_protocol(Usb2HostControllerProtocol::GUID)]
pub struct Usb2HostController(Usb2HostControllerProtocol);

impl Usb2HostController {
    /// Reset the USB host controller.
    pub fn reset(&mut self, attributes: ResetAttributes) -> Result {
        unsafe { (self.0.reset)(&mut self.0, attributes) }.to_result()
    }

    /// Returns the [`Capabilities`] of the USB host controller.
    pub fn capabilities(&mut self) -> Result<Capabilities> {
        let mut max_speed = Speed::FULL;
        let mut port_count = 0;
        let mut is_64_bit_capable = 0;

        unsafe {
            (self.0.get_capability)(
                &mut self.0,
                &mut max_speed,
                &mut port_count,
                &mut is_64_bit_capable,
            )
        }
        .to_result_with_val(|| Capabilities {
            max_speed,
            port_count,
            is_64_bit_capable: is_64_bit_capable != 0,
        })
    }

    /// Returns the current state of the USB host controller.
    pub fn get_state(&mut self) -> Result<HostControllerState> {
        let mut hc_state = HostControllerState::HALT;

        unsafe { (self.0.get_state)(&mut self.0, &mut hc_state) }.to_result_with_val(|| hc_state)
    }

    /// Sets the state of the USB host controller.
    pub fn set_state(&mut self, state: HostControllerState) -> Result {
        unsafe { (self.0.set_state)(&mut self.0, state) }.to_result()
    }

    /// Returns the current [`UsbPortStatus`] of a USB root hub port.
    pub fn root_hub_port_status(&mut self, port_number: u8) -> Result<UsbPortStatus> {
        let mut port_status = UsbPortStatus {
            port_status: PortStatus::empty(),
            port_change_status: PortChangeStatus::empty(),
        };

        unsafe { (self.0.get_root_hub_port_status)(&mut self.0, port_number, &mut port_status) }
            .to_result_with_val(|| port_status)
    }

    /// Sets the [`PortFeature`] of a USB root hub port.
    pub fn set_root_hub_port_feature(&mut self, port_number: u8, feature: PortFeature) -> Result {
        unsafe { (self.0.set_root_hub_port_feature)(&mut self.0, port_number, feature) }.to_result()
    }

    /// Clears the [`PortFeature`] of a USB root hub port.
    pub fn clear_root_hub_port_feature(&mut self, port_number: u8, feature: PortFeature) -> Result {
        unsafe { (self.0.clear_root_hub_port_feature)(&mut self.0, port_number, feature) }
            .to_result()
    }

    pub fn control

    /// Returns the major revision of the USB host controller.
    pub fn major_revision(&self) -> u16 {
        self.0.major_revision
    }

    /// Returns the minor revision of the USB host controller.
    pub fn minor_revision(&self) -> u16 {
        self.0.minor_revision
    }
}
