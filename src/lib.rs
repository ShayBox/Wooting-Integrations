pub mod integrations;

use anyhow::{Context, Result};
use hidapi::{HidApi, HidDevice, HidResult};

/* HID */
pub const WOOTING_COMMAND_SIZE: usize = 8;
pub const WOOTING_REPORT_SIZE: usize = 257;
pub const WOOTING_RESPONSE_SIZE: usize = 256;

pub const WOOTING_READ_RESPONSE_TIMEOUT: i32 = 1000;

/* Devices */
pub const CFG_USAGE_PAGE: u16 = 0x1337;
pub const WOOTING_VID: u16 = 0x31e3;

/* RGB */
pub const WOOTING_MAX_RGB_DEVICES: u32 = 10;

pub const RGB_RAW_BUFFER_SIZE: u32 = 96;

pub const WOOTING_RGB_ROWS: usize = 6;
pub const WOOTING_RGB_COLS: usize = 21;
pub const WOOTING_ONE_RGB_COLS: u32 = 17;
pub const WOOTING_TWO_RGB_COLS: u32 = 21;

pub const WOOTING_ONE_KEY_CODE_LIMIT: u32 = 95;
pub const WOOTING_TWO_KEY_CODE_LIMIT: u32 = 116;

/* Types */
pub type Rgb = [u8; 3];
pub type RgbMatrix = [[u16; WOOTING_RGB_COLS]; WOOTING_RGB_ROWS];
pub type Response = [u8; WOOTING_RESPONSE_SIZE];

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Command {
    Ping = 0,
    GetVersion = 1,
    ResetToBootloader = 2,
    GetSerial = 3,
    GetRgbProfileCount = 4,
    REMOVED_GetCurrentRgbProfileIndex = 5,
    REMOVED_GetRgbMainProfile = 6,
    ReloadProfile0 = 7,
    SaveRgbProfile = 8,
    GetDigitalProfilesCount = 9,
    GetAnalogProfilesCount = 10,
    GetCurrentKeyboardProfileIndex = 11, // WOOTING_RAW_COLORS_REPORT
    GetDigitalProfile = 12,
    GetAnalogProfileMainPart = 13,
    GetAnalogProfileCurveChangeMapPart1 = 14,
    GetAnalogProfileCurveChangeMapPart2 = 15,
    GetNumberOfKeys = 16,
    GetMainMappingProfile = 17,
    GetFunctionMappingProfile = 18,
    GetDeviceConfig = 19, // WOOTING_DEVICE_CONFIG_COMMAND
    GetAnalogValues = 20,
    KeysOff = 21,
    KeysOn = 22,
    ActivateProfile = 23,
    getDKSProfile = 24,
    doSoftReset = 25,
    REMOVED_GetRgbColorsPart1 = 26,
    REMOVED_GetRgbColorsPart2 = 27,
    REMOVED_GetRgbEffects = 28,
    RefreshRgbColors = 29,
    WootDevSingleColor = 30, // WOOTING_SINGLE_COLOR_COMMAND
    WootDevResetColor = 31,  // WOOTING_SINGLE_RESET_COMMAND
    WootDevResetAll = 32,    // WOOTING_RESET_ALL_COMMAND
    WootDevInit = 33,        // WOOTING_COLOR_INIT_COMMAND
    REMOVED_GetRgbProfileBase = 34,
    GetRgbProfileColorsPart1 = 35,
    GetRgbProfileColorsPart2 = 36,
    REMOVED_GetRgbProfileEffect = 37,
    ReloadProfile = 38,
    GetKeyboardProfile = 39,
    GetGamepadMapping = 40,
    GetGamepadProfile = 41,
    SaveKeyboardProfile = 42,
    ResetSettings = 43,
    SetRawScanning = 44,
    StartXinputDetection = 45,
    StopXinputDetection = 46,
    SaveDKSProfile = 47,
    GetMappingProfile = 48,
    GetActuationProfile = 49,
    GetRgbProfileCore = 50,
    GetGlobalSettings = 51,
    GetAKCProfile = 52,
    SaveAKCProfile = 53,
    GetRapidTriggerProfile = 54,
    GetProfileMetadata = 55,
    IsFLashChipConnected = 56,
    GetRgbLayer = 57,
    GetFlashStats = 58,
    GetRGBBins = 59,
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Report {
    REMOVED_RgbMainPart = 0,
    REMOVED_DigitalProfileMainPart = 1,
    REMOVED_AnalogProfileMainPart = 2,
    REMOVED_AnalogProfileCurveChangeMapPart1 = 3,
    REMOVED_AnalogProfileCurveChangeMapPart2 = 4,
    REMOVED_MainMappingProfile = 5,
    REMOVED_FunctionMappingProfile = 6,
    DeviceConfig = 7,
    SetDKSProfile = 8,
    RgbColorsPart = 9,
    REMOVED_RgbEffects = 10,
    WootDevRawReport = 11,
    SerialNumber = 12,
    REMOVED_RgbProfileBase = 13,
    RgbProfileColorsPart1 = 14,
    RgbProfileColorsPart2 = 15,
    REMOVED_RgbProfileEffect = 16,
    KeyboardProfile = 17,
    GamepadMapping = 18,
    GamepadProfile = 19,
    MappingProfile = 20,
    ActuationProfile = 21,
    RgbProfileCore = 22,
    GlobalSettings = 23,
    AKCProfile = 24,
    RapidTriggerProfile = 25,
    ProfileMetadata = 26,
    RgbLayer = 27,
    RGBBins = 28,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Product {
    WootingOne = 0x1100,
    WootingTwo = 0x1200,
    WootingTwoLe = 0x1210,
    WootingTwoHe = 0x1220,
    WootingTwoHeArm = 0x1230,
    Wooting60He = 0x1300,
    Wooting60HeArm = 0x1310,
    Wooting60HePlus = 0x1320,
    WootingUwu = 0x1500,
    WootingUwuRgb = 0x1510,
}

pub struct KeyColor(pub u16);

impl From<Rgb> for KeyColor {
    #[allow(clippy::cast_lossless)] // Lossy
    fn from([r, g, b]: Rgb) -> Self {
        let mut encode = 0;

        encode |= ((r & 0xf8) as u16) << 8;
        encode |= ((g & 0xfc) as u16) << 3;
        encode |= ((b & 0xf8) as u16) >> 3;

        Self(encode)
    }
}

pub struct Keyboard(HidDevice);

impl Keyboard {
    /// # Find a matching keyboard
    ///
    /// # Errors
    /// Will return `Err` if `DeviceInfo::open_device` fails.
    pub fn find(product: Product) -> Result<Self> {
        let api = HidApi::new()?;
        let device = api
            .device_list()
            .filter(|info| info.usage_page() == CFG_USAGE_PAGE)
            .filter(|info| info.vendor_id() == WOOTING_VID)
            .find(|info| info.product_id() == product as u16)
            .context("Couldn't find device")?
            .open_device(&api)?;

        Ok(Self(device))
    }

    /// # Send an RGB matrix to the device
    ///
    /// # Errors
    /// Will return `Err` if `HidDevice::write` fails.
    ///
    /// # Panics
    /// Will panic if buffer is not 257 bytes long.
    pub fn send_rgb_matrix(&self, matrix: RgbMatrix) -> HidResult<()> {
        let src = matrix
            .into_iter()
            .flatten()
            .flat_map(u16::to_le_bytes)
            .collect::<Vec<_>>();

        let mut buf = [0u8; WOOTING_REPORT_SIZE];
        buf[1] = 0xD0;
        buf[2] = 0xDA;
        buf[3] = Command::GetCurrentKeyboardProfileIndex as u8;
        buf[4..(4 + src.len())].copy_from_slice(&src);

        assert!(buf.len() == WOOTING_REPORT_SIZE, "Invalid command size");
        self.0.write(&buf)?;

        Ok(())
    }

    /// # Send an RGB buffer to the device
    ///
    /// # Errors
    /// Will return `Err` if `HidDevice::send_feature_report` fails.
    ///
    /// # Panics
    /// Will panic if command is not 8 bytes long or response is not 256 bytes long.
    pub fn send_command(&self, command: Command) -> HidResult<Response> {
        self.send_command_with_args(command, 0, 0, 0, 0)
    }

    /// # Send an RGB buffer to the device
    ///
    /// # Errors
    /// Will return `Err` if `HidDevice::send_feature_report` fails.
    ///
    /// # Panics
    /// Will panic if command is not 8 bytes long or response is not 256 bytes long.
    pub fn send_command_with_args(
        &self,
        command: Command,
        p0: u8,
        p1: u8,
        p2: u8,
        p3: u8,
    ) -> HidResult<Response> {
        let buf = [0, 0xD0, 0xDA, command as u8, p3, p2, p1, p0];
        assert!(buf.len() == WOOTING_COMMAND_SIZE, "Invalid command size");
        self.0.send_feature_report(&buf)?;

        let mut buf = [0u8; WOOTING_RESPONSE_SIZE];
        let buf_len = self.0.read_timeout(&mut buf, 1000)?;
        assert!(buf_len == WOOTING_RESPONSE_SIZE, "Invalid command size");

        Ok(buf)
    }

    /// # Update keyboard RGB with integrations
    ///
    /// # Errors
    /// Will return `Err` if `Self::send_rgb_matrix` fails.
    pub fn update<F>(&self, f: &mut F) -> HidResult<()>
    where
        F: FnMut(&Self, &mut Rgb, (usize, usize)),
    {
        let mut rgb = Rgb::default();
        let mut matrix = RgbMatrix::default();
        for (row, scanline) in matrix.iter_mut().enumerate() {
            for (col, pixel) in scanline.iter_mut().enumerate() {
                f(self, &mut rgb, (col, row));

                *pixel = KeyColor::from(rgb).0;
            }
        }

        self.send_rgb_matrix(matrix)
    }
}
