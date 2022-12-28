use display_interface::WriteOnlyDataCommand;
use embedded_graphics_core::pixelcolor::{Rgb565, Rgb666};
use embedded_hal::{blocking::delay::DelayUs, digital::v2::OutputPin};

use crate::{
    dcs::{Dcs, SetAddressMode, SoftReset},
    error::InitError,
    models::{ili934x, Model},
    Builder, Error, ModelOptions,
};

/// ILI9342C display with Reset pin
/// in Rgb565 color mode
/// Backlight pin is not controlled
pub struct ILI9342CRgb565;

/// ILI9342C display with Reset pin
/// in Rgb666 color mode
/// Backlight pin is not controlled
pub struct ILI9342CRgb666;

impl Model for ILI9342CRgb565 {
    type ColorFormat = Rgb565;

    fn init<RST, DELAY, DI>(
        &mut self,
        dcs: &mut Dcs<DI>,
        delay: &mut DELAY,
        options: &ModelOptions,
        rst: &mut Option<RST>,
    ) -> Result<SetAddressMode, InitError<RST::Error>>
    where
        RST: OutputPin,
        DELAY: DelayUs<u32>,
        DI: WriteOnlyDataCommand,
    {
        match rst {
            Some(ref mut rst) => self.hard_reset(rst, delay)?,
            None => dcs.write_command(SoftReset)?,
        }

        ili934x::init_common::<_, _, Self::ColorFormat>(dcs, delay, options).map_err(Into::into)
    }

    fn write_pixels<DI, I>(&mut self, dcs: &mut Dcs<DI>, colors: I) -> Result<(), Error>
    where
        DI: WriteOnlyDataCommand,
        I: IntoIterator<Item = Self::ColorFormat>,
    {
        ili934x::write_pixels_rgb565(dcs, colors)
    }

    fn default_options() -> ModelOptions {
        ModelOptions::with_sizes((320, 240), (320, 240))
    }
}

impl Model for ILI9342CRgb666 {
    type ColorFormat = Rgb666;

    fn init<RST, DELAY, DI>(
        &mut self,
        dcs: &mut Dcs<DI>,
        delay: &mut DELAY,
        options: &ModelOptions,
        rst: &mut Option<RST>,
    ) -> Result<SetAddressMode, InitError<RST::Error>>
    where
        RST: OutputPin,
        DELAY: DelayUs<u32>,
        DI: WriteOnlyDataCommand,
    {
        match rst {
            Some(ref mut rst) => self.hard_reset(rst, delay)?,
            None => dcs.write_command(SoftReset)?,
        }

        ili934x::init_common::<_, _, Self::ColorFormat>(dcs, delay, options).map_err(Into::into)
    }

    fn write_pixels<DI, I>(&mut self, dcs: &mut Dcs<DI>, colors: I) -> Result<(), Error>
    where
        DI: WriteOnlyDataCommand,
        I: IntoIterator<Item = Self::ColorFormat>,
    {
        ili934x::write_pixels_rgb666(dcs, colors)
    }

    fn default_options() -> ModelOptions {
        ModelOptions::with_sizes((320, 240), (320, 240))
    }
}

// simplified constructor for Display

impl<DI> Builder<DI, ILI9342CRgb565>
where
    DI: WriteOnlyDataCommand,
{
    ///
    /// Creates a new [Display] instance with [ILI9342C] as the [Model]
    /// with the default framebuffer size and display size of 320x240
    /// *WARNING* Rgb565 only works on non-SPI setups with the ILI9342C!
    ///
    /// # Arguments
    ///
    /// * `di` - a [DisplayInterface](WriteOnlyDataCommand) for talking with the display
    ///
    pub fn ili9342c_rgb565(di: DI) -> Self {
        Self::with_model(di, ILI9342CRgb565)
    }
}

impl<DI> Builder<DI, ILI9342CRgb666>
where
    DI: WriteOnlyDataCommand,
{
    ///
    /// Creates a new [Display] instance with [ILI9342C] as the [Model]
    /// with the default framebuffer size and display size of 320x240
    ///
    /// # Arguments
    ///
    /// * `di` - a [DisplayInterface](WriteOnlyDataCommand) for talking with the display
    ///
    pub fn ili9342c_rgb666(di: DI) -> Self {
        Self::with_model(di, ILI9342CRgb666)
    }
}
