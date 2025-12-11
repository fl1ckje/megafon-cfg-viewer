use crate::generic::LineScanner;
use thiserror::Error;

// -----------------------------------------------------------------------------
// specific data structures
// -----------------------------------------------------------------------------

/// Объект, представляющий конфигурацию сенсорной панели ОРМ СКРС "Мегафон".
///
/// Экземпляр `ScreenConfig` хранит основные поля, такие как:
///
/// * [`internal_address`] - внутренний номер абонента.
/// * [`name`] - наименование рабочего места.
/// * [`available_radiostations`] - доступные радиостанции на рабочем месте.
/// * [`phone_panels`] - список панелей с кнопками оперативного вызова.
/// * [`radio_panels`] - список панелей с кнопками радиостанций.
#[derive(Debug, Clone, Default)]
pub struct ScreenConfig {
    pub internal_address: Option<u32>,
    pub name: Option<String>,
    pub available_radiostations: Vec<AvailableRadiostation>,
    pub phone_panels: Vec<PhonePanel>,
    pub radio_panels: Vec<RadioPanel>,
}

#[derive(Debug, Clone, Default)]
pub struct AvailableRadiostation {
    pub id: String,
    pub radio_name: String,
    pub slot: i32,
}

#[derive(Debug, Clone, Default)]
pub struct PhonePanel {
    pub id: String,
    pub buttons: Vec<PhoneButton>,
}

#[derive(Debug, Clone, Default)]
pub struct PhoneButton {
    pub id: String,
    pub internal_address: u32,
    pub position_x: f32,
    pub position_y: f32,
    pub size_height: f32,
    pub size_width: f32,
    pub text: String,
}

#[derive(Debug, Clone, Default)]
pub struct RadioPanel {
    pub id: String,
    pub buttons: Vec<RadioButton>,
}

#[derive(Debug, Clone, Default)]
pub struct RadioButton {
    pub id: String,
    pub position_x: f32,
    pub position_y: f32,
    pub size_height: f32,
    pub size_width: f32,
    pub slot: i32,
    pub text: String,
}

// -----------------------------------------------------------------------------
// error types
// -----------------------------------------------------------------------------

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Invalid integer: {0}")]
    InvalidInt(#[from] std::num::ParseIntError),
    #[error("Invalid float: {0}")]
    InvalidFloat(#[from] std::num::ParseFloatError),
    #[error("Unknown global key: {0}")]
    UnknownGlobalKey(String),
}

// -----------------------------------------------------------------------------
// keys for data structures fields
// -----------------------------------------------------------------------------

const AVAILABLE_RADIOSTATIONS: &str = "AvailableRadiostations";
const AVAILABLE_RADIOSTATION: &str = "AvailableRadiostation";
const RADIO_NAME: &str = "radio_name";
const SLOT: &str = "slot";
const PHONE_PANELS: &str = "PhonePanels";
const PANEL: &str = "Panel";
const BUTTON: &str = "Button";
const INTERNAL_ADDRESS: &str = "internal_address";
const POSITION_X: &str = "position_x";
const POSITION_Y: &str = "position_y";
const SIZE_HEIGHT: &str = "size_height";
const SIZE_WIDTH: &str = "size_width";
const TEXT: &str = "text";
const RADIO_PANELS: &str = "RadioPanels";
const NAME: &str = "name";

// -----------------------------------------------------------------------------
// parsing logic
// -----------------------------------------------------------------------------

pub fn parse(input: &str) -> Result<ScreenConfig, ConfigError> {
    let mut scanner = LineScanner::new(input);
    let mut config = ScreenConfig::default();

    while let Some(line) = scanner.peek_line() {
        if let Some(section) = LineScanner::get_section_name(line) {
            scanner.next_line();
            match section {
                AVAILABLE_RADIOSTATIONS => {
                    config.available_radiostations = parse_radiostations(&mut scanner)?
                }
                PHONE_PANELS => config.phone_panels = parse_phone_panels(&mut scanner)?,
                RADIO_PANELS => config.radio_panels = parse_radio_panels(&mut scanner)?,
                _ => consume_block(&mut scanner, section),
            }
        } else if let Some((key, value)) = LineScanner::parse_kv(line) {
            scanner.next_line();
            match key {
                INTERNAL_ADDRESS => config.internal_address = Some(value.parse()?),
                NAME => config.name = Some(LineScanner::clean_string(value)),
                _ => {} // return Err(ConfigError::UnknownGlobalKey(key.to_string())),
            }
        } else {
            scanner.next_line();
        }
    }
    Ok(config)
}

fn parse_radiostations(
    scanner: &mut LineScanner,
) -> Result<Vec<AvailableRadiostation>, ConfigError> {
    let mut stations = Vec::new();
    while let Some(line) = scanner.peek_line() {
        if LineScanner::is_closing_tag(line, AVAILABLE_RADIOSTATIONS) {
            scanner.next_line();
            break;
        }

        if let Some(id) = LineScanner::get_section_name(line) {
            if id.starts_with(AVAILABLE_RADIOSTATION) {
                scanner.next_line();
                let mut station = AvailableRadiostation {
                    id: id.to_string(),
                    ..Default::default()
                };

                while let Some(inner) = scanner.peek_line() {
                    if LineScanner::is_closing_tag(inner, id) {
                        scanner.next_line();
                        break;
                    }
                    if let Some((key, value)) = LineScanner::parse_kv(inner) {
                        scanner.next_line();
                        match key {
                            RADIO_NAME => station.radio_name = LineScanner::clean_string(value),
                            SLOT => station.slot = value.parse()?,
                            _ => {}
                        }
                    } else {
                        scanner.next_line();
                    }
                }
                stations.push(station);
            } else {
                scanner.next_line();
            }
        } else {
            scanner.next_line();
        }
    }
    Ok(stations)
}

fn parse_phone_panels(scanner: &mut LineScanner) -> Result<Vec<PhonePanel>, ConfigError> {
    let mut panels = Vec::new();
    while let Some(line) = scanner.peek_line() {
        if LineScanner::is_closing_tag(line, PHONE_PANELS) {
            scanner.next_line();
            break;
        }

        if let Some(panel_id) = LineScanner::get_section_name(line) {
            if panel_id.starts_with(PANEL) {
                scanner.next_line();
                let mut panel = PhonePanel {
                    id: panel_id.to_string(),
                    buttons: vec![],
                };

                while let Some(inner) = scanner.peek_line() {
                    if LineScanner::is_closing_tag(inner, panel_id) {
                        scanner.next_line();
                        break;
                    }

                    if let Some(btn_id) = LineScanner::get_section_name(inner) {
                        if btn_id.starts_with(BUTTON) {
                            scanner.next_line();
                            let mut btn = PhoneButton {
                                id: btn_id.to_string(),
                                ..Default::default()
                            };
                            while let Some(b_line) = scanner.peek_line() {
                                if LineScanner::is_closing_tag(b_line, btn_id) {
                                    scanner.next_line();
                                    break;
                                }
                                if let Some((k, v)) = LineScanner::parse_kv(b_line) {
                                    scanner.next_line();
                                    match k {
                                        INTERNAL_ADDRESS => btn.internal_address = v.parse()?,
                                        POSITION_X => btn.position_x = v.parse()?,
                                        POSITION_Y => btn.position_y = v.parse()?,
                                        SIZE_HEIGHT => btn.size_height = v.parse()?,
                                        SIZE_WIDTH => btn.size_width = v.parse()?,
                                        TEXT => btn.text = LineScanner::clean_string(v),
                                        _ => {}
                                    }
                                } else {
                                    scanner.next_line();
                                }
                            }
                            panel.buttons.push(btn);
                        } else {
                            scanner.next_line();
                        }
                    } else {
                        scanner.next_line();
                    }
                }
                panels.push(panel);
            } else {
                scanner.next_line();
            }
        } else {
            scanner.next_line();
        }
    }
    Ok(panels)
}

fn parse_radio_panels(scanner: &mut LineScanner) -> Result<Vec<RadioPanel>, ConfigError> {
    let mut panels = Vec::new();
    while let Some(line) = scanner.peek_line() {
        if LineScanner::is_closing_tag(line, RADIO_PANELS) {
            scanner.next_line();
            break;
        }

        if let Some(panel_id) = LineScanner::get_section_name(line) {
            if panel_id.starts_with(PANEL) {
                scanner.next_line();
                let mut panel = RadioPanel {
                    id: panel_id.to_string(),
                    buttons: vec![],
                };

                while let Some(inner) = scanner.peek_line() {
                    if LineScanner::is_closing_tag(inner, panel_id) {
                        scanner.next_line();
                        break;
                    }

                    if let Some(btn_id) = LineScanner::get_section_name(inner) {
                        if btn_id.starts_with(BUTTON) {
                            scanner.next_line();
                            let mut btn = RadioButton {
                                id: btn_id.to_string(),
                                ..Default::default()
                            };
                            while let Some(b_line) = scanner.peek_line() {
                                if LineScanner::is_closing_tag(b_line, btn_id) {
                                    scanner.next_line();
                                    break;
                                }
                                if let Some((key, value)) = LineScanner::parse_kv(b_line) {
                                    scanner.next_line();
                                    match key {
                                        POSITION_X => btn.position_x = value.parse()?,
                                        POSITION_Y => btn.position_y = value.parse()?,
                                        SIZE_HEIGHT => btn.size_height = value.parse()?,
                                        SIZE_WIDTH => btn.size_width = value.parse()?,
                                        SLOT => btn.slot = value.parse()?,
                                        TEXT => btn.text = LineScanner::clean_string(value),
                                        _ => {}
                                    }
                                } else {
                                    scanner.next_line();
                                }
                            }
                            panel.buttons.push(btn);
                        } else {
                            scanner.next_line();
                        }
                    } else {
                        scanner.next_line();
                    }
                }
                panels.push(panel);
            } else {
                scanner.next_line();
            }
        } else {
            scanner.next_line();
        }
    }
    Ok(panels)
}

fn consume_block(scanner: &mut LineScanner, block_name: &str) {
    while let Some(line) = scanner.next_line() {
        if LineScanner::is_closing_tag(line, block_name) {
            break;
        }
    }
}
