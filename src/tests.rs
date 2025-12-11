use crate::config::parse;

const INPUT: &str = r#"
internal_address = 331
master_volume_show = 1
name = "Инженер КСРС"
    [AvailableRadiostations]
        [AvailableRadiostation01]
radio_name = "Улан-Удэ 134.1"
slot = -1
[#AvailableRadiostation01]
[AvailableRadiostation02]
radio_name = "Талакан 135.4"
slot = -1
[#AvailableRadiostation02]
[#AvailableRadiostations]
[PhonePanels]
[Panel01]
[Button01]
internal_address = 303
position_x = 0.02
position_y = 0.016
size_height = 0.147
size_width = 0.225
text = "С-6 ПУ"
[#Button01]
[Button02]
internal_address = 309
position_x = 0.264
position_y = 0.016
size_height = 0.147
size_width = 0.225
text = "С-9 ПУ"
[#Button02]
[#Panel01]
[Panel02]
[Button01]
internal_address = 338
position_x = 0.02
position_y = 0.016
size_height = 0.147
size_width = 0.225
text = "Диспетчер ПИВП вне ВТ"
[#Button01]
[#Panel02]
[#PhonePanels]
[RadioPanels]
[Panel01]
[Button01]
position_x = 0.02
position_y = 0.007
size_height = 0.158
size_width = 0.961
slot = 5
text = ""
[#Button01]
[#Panel01]
[#RadioPanels]
"#;

#[test]
fn test_parse_screen_globals() {
    let result = parse(INPUT);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    let config = result.unwrap();

    // 1. Check Globals
    assert_eq!(config.internal_address, Some(331));
    // assert_eq!(config.master_volume_show, Some(1));
    assert_eq!(config.name, Some("Инженер КСРС".to_string()));
}

#[test]
fn test_parse_screen_radiostations() {
    let result = parse(INPUT);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    let config = result.unwrap();

    // 2. Check Radiostations
    assert_eq!(config.available_radiostations.len(), 2);
    let radio1 = &config.available_radiostations[0];
    assert_eq!(radio1.id, "AvailableRadiostation01");
    assert_eq!(radio1.radio_name, "Улан-Удэ 134.1");
    assert_eq!(radio1.slot, -1);
}

#[test]
fn test_parse_screen_phone_panels() {
    let result = parse(INPUT);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    let config = result.unwrap();

    // 3. Check Phone Panels
    assert_eq!(config.phone_panels.len(), 2);

    // Panel 01
    let p1 = &config.phone_panels[0];
    assert_eq!(p1.id, "Panel01");
    assert_eq!(p1.buttons.len(), 2);

    let p1_b1 = &p1.buttons[0];
    assert_eq!(p1_b1.id, "Button01");
    assert_eq!(p1_b1.internal_address, 303);
    assert_eq!(p1_b1.text, "С-6 ПУ");
    assert!((p1_b1.position_x - 0.02).abs() < f32::EPSILON);

    // Panel 02
    let p2 = &config.phone_panels[1];
    assert_eq!(p2.id, "Panel02");
    assert_eq!(p2.buttons.len(), 1);
    assert_eq!(p2.buttons[0].text, "Диспетчер ПИВП вне ВТ");
}

#[test]
fn test_parse_screen_radio_panels() {
    let result = parse(INPUT);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    let config = result.unwrap();

    // 4. Check Radio Panels
    assert_eq!(config.radio_panels.len(), 1);
    let rp1 = &config.radio_panels[0];
    assert_eq!(rp1.id, "Panel01");
    assert_eq!(rp1.buttons.len(), 1);
    assert_eq!(rp1.buttons[0].slot, 5);
}
