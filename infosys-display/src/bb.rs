// BetaBrite helper functions + definitions

// Don't fret
#![allow(dead_code)]

// Constants
pub static START_PACKET: &'static [u8] = &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x5A, 0x30, 0x30];
pub static END_PACKET: &'static [u8] = &[0x04, 0x0a];


// ===== SPECIAL MODES =====
pub static SPECIAL_SPEAKER_ON: &'static [u8] = &[0x21, 0x30, 0x30];
pub static SPECIAL_SPEAKER_OFF: &'static [u8] = &[0x21, 0x46, 0x46];
pub static SPECIAL_SPEAKER_TEST: &'static [u8] = &[0x28, 0x31];
pub static SPECIAL_SPEAKER: &'static [u8] = &[0x28];
pub static SPECIAL_SET_DATE: &'static [u8] = &[0x3B];
pub static SPECIAL_SET_TIME: &'static [u8] = &[0x20];
pub static SPECIAL_SET_DOTS: &'static [u8] = &[0x38];

// ===== SPECIAL WRITE SEQUENCES =====
// Display Parameters
pub static DOUBLE_HEIGHT_OFF: &'static [u8] = &[0x05, 0x30];
pub static DOUBLE_HEIGHT_ON: &'static [u8] = &[0x05, 0x31];
pub static TRUE_DESCENDERS_OFF: &'static [u8] = &[0x06, 0x30];
pub static TRUE_DESCENDERS_ON: &'static [u8] = &[0x06, 0x31];
pub static CHARACTER_FLASH_OFF: &'static [u8] = &[0x07, 0x30];
pub static CHARACTER_FLASH_ON: &'static [u8] = &[0x07, 0x31];
pub static SPEED_1: &'static [u8] = &[0x15];
pub static SPEED_2: &'static [u8] = &[0x16];
pub static SPEED_3: &'static [u8] = &[0x17];
pub static SPEED_4: &'static [u8] = &[0x18];
pub static SPEED_5: &'static [u8] = &[0x19];

// Display Modes
pub static MODE_STANDARD_ROTATE: &'static [u8] = &[0x61];
pub static MODE_STANDARD_HOLD: &'static [u8] = &[0x62];
pub static MODE_STANDARD_FLASH: &'static [u8] = &[0x63];
pub static MODE_STANDARD_ROLL_UP: &'static [u8] = &[0x65];
pub static MODE_STANDARD_ROLL_DOWN: &'static [u8] = &[0x66];
pub static MODE_STANDARD_ROLL_LEFT: &'static [u8] = &[0x67];
pub static MODE_STANDARD_ROLL_RIGHT: &'static [u8] = &[0x68];
pub static MODE_STANDARD_WIPE_UP: &'static [u8] = &[0x69];
pub static MODE_STANDARD_WIPE_DOWN: &'static [u8] = &[0x6A];
pub static MODE_STANDARD_WIPE_LEFT: &'static [u8] = &[0x6B];
pub static MODE_STANDARD_WIPE_RIGHT: &'static [u8] = &[0x6C];
pub static MODE_STANDARD_SCROLL: &'static [u8] = &[0x6D];
pub static MODE_STANDARD_AUTO_MODE: &'static [u8] = &[0x6F];
pub static MODE_STANDARD_ROLL_IN: &'static [u8] = &[0x70];
pub static MODE_STANDARD_ROLL_OUT: &'static [u8] = &[0x71];
pub static MODE_STANDARD_WIPE_IN: &'static [u8] = &[0x72];
pub static MODE_STANDARD_WIPE_OUT: &'static [u8] = &[0x73];
pub static MODE_STANDARD_COMPRESSED_ROTATE: &'static [u8] = &[0x74];
pub static MODE_STANDARD_EXPLODE: &'static [u8] = &[0x75];
pub static MODE_STANDARD_CLOCK: &'static [u8] = &[0x76];

pub static MODE_SPECIAL_TWINKLE: &'static [u8] = &[0x6E, 0x30];
pub static MODE_SPECIAL_SPARKLE: &'static [u8] = &[0x6E, 0x31];
pub static MODE_SPECIAL_SNOW: &'static [u8] = &[0x6E, 0x32];
pub static MODE_SPECIAL_INTERLOCK: &'static [u8] = &[0x6E, 0x33];
pub static MODE_SPECIAL_SWITCH: &'static [u8] = &[0x6E, 0x34];
pub static MODE_SPECIAL_SLIDE: &'static [u8] = &[0x6E, 0x35];
pub static MODE_SPECIAL_SPRAY: &'static [u8] = &[0x6E, 0x36];
pub static MODE_SPECIAL_STARBURST: &'static [u8] = &[0x6E, 0x37];
pub static MODE_SPECIAL_WELCOME: &'static [u8] = &[0x6E, 0x38];
pub static MODE_SPECIAL_SLOT_MACHINE: &'static [u8] = &[0x6E, 0x39];
pub static MODE_SPECIAL_NEWS_FLASH: &'static [u8] = &[0x6E, 0x3A];
pub static MODE_SPECIAL_TRUMPET: &'static [u8] = &[0x6E, 0x3B];
pub static MODE_SPECIAL_CYCLE_COLORS: &'static [u8] = &[0x6E, 0x43];
pub static MODE_SPECIAL_THANK_YOU: &'static [u8] = &[0x6E, 0x53];
pub static MODE_SPECIAL_NO_SMOKING: &'static [u8] = &[0x6E, 0x55];
pub static MODE_SPECIAL_DONT_DRINK_AND_DRIVE: &'static [u8] = &[0x6E, 0x56];
pub static MODE_SPECIAL_RUNNING_ANIMAL_OR_FISH: &'static [u8] = &[0x6E, 0x57];
pub static MODE_SPECIAL_FIREWORKS: &'static [u8] = &[0x6E, 0x58];
pub static MODE_SPECIAL_BALLOON_ANIMATION: &'static [u8] = &[0x6E, 0x59];
pub static MODE_SPECIAL_CHERRY_BOMB: &'static [u8] = &[0x6E, 0x5A];


// Special Characters
pub static DISPLAY_SPECIAL_CAP_C_CEDILLA: &'static [u8] = &[0x08, 0x20];
pub static DISPLAY_SPECIAL_LOW_U_DIAERESIS: &'static [u8] = &[0x08, 0x21];
pub static DISPLAY_SPECIAL_LOW_E_ACUTE: &'static [u8] = &[0x08, 0x22];
pub static DISPLAY_SPECIAL_LOW_A_CIRCUMFLEX: &'static [u8] = &[0x08, 0x23];
pub static DISPLAY_SPECIAL_LOW_A_DIAERESIS: &'static [u8] = &[0x08, 0x24];
pub static DISPLAY_SPECIAL_LOW_A_GRAVE: &'static [u8] = &[0x08, 0x25];
pub static DISPLAY_SPECIAL_LOW_A_RING: &'static [u8] = &[0x08, 0x26];
pub static DISPLAY_SPECIAL_LOW_C_CEDILLA: &'static [u8] = &[0x08, 0x27];
pub static DISPLAY_SPECIAL_LOW_E_CIRCUMFLEX: &'static [u8] = &[0x08, 0x28];
pub static DISPLAY_SPECIAL_LOW_E_DIAERESIS: &'static [u8] = &[0x08, 0x29];
pub static DISPLAY_SPECIAL_LOW_E_GRAVE: &'static [u8] = &[0x08, 0x2A];
pub static DISPLAY_SPECIAL_LOW_I_DIAERESIS: &'static [u8] = &[0x08, 0x2B];
pub static DISPLAY_SPECIAL_LOW_I_CIRCUMFLEX: &'static [u8] = &[0x08, 0x2C];
pub static DISPLAY_SPECIAL_LOW_I_GRAVE: &'static [u8] = &[0x08, 0x2D];
pub static DISPLAY_SPECIAL_CAP_A_DIAERESIS: &'static [u8] = &[0x08, 0x2E];
pub static DISPLAY_SPECIAL_CAP_A_RING: &'static [u8] = &[0x08, 0x2F];
pub static DISPLAY_SPECIAL_CAP_E_ACUTE: &'static [u8] = &[0x08, 0x30];
pub static DISPLAY_SPECIAL_LOW_AE: &'static [u8] = &[0x08, 0x31];
pub static DISPLAY_SPECIAL_CAP_AE: &'static [u8] = &[0x08, 0x32];
pub static DISPLAY_SPECIAL_LOW_O_CIRCUMFLEX: &'static [u8] = &[0x08, 0x33];
pub static DISPLAY_SPECIAL_LOW_O_DIAERESIS: &'static [u8] = &[0x08, 0x34];
pub static DISPLAY_SPECIAL_LOW_O_GRAVE: &'static [u8] = &[0x08, 0x35];
pub static DISPLAY_SPECIAL_LOW_U_CIRCUMFLEX: &'static [u8] = &[0x08, 0x36];
pub static DISPLAY_SPECIAL_LOW_U_GRAVE: &'static [u8] = &[0x08, 0x37];
pub static DISPLAY_SPECIAL_LOW_Y_DIAERESIS: &'static [u8] = &[0x08, 0x38];
pub static DISPLAY_SPECIAL_CAP_O_DIAERESIS: &'static [u8] = &[0x08, 0x39];
pub static DISPLAY_SPECIAL_CAP_U_DIAERESIS: &'static [u8] = &[0x08, 0x3A];
pub static DISPLAY_SPECIAL_CENT: &'static [u8] = &[0x08, 0x3B];
pub static DISPLAY_SPECIAL_POUND: &'static [u8] = &[0x08, 0x3C];
pub static DISPLAY_SPECIAL_YEN: &'static [u8] = &[0x08, 0x3D];
pub static DISPLAY_SPECIAL_MYSTERY_MONEY: &'static [u8] = &[0x08, 0x3E]; // XXX TODO Help? maybe peseta????
pub static DISPLAY_SPECIAL_FORTE: &'static [u8] = &[0x08, 0x3F];
pub static DISPLAY_SPECIAL_LOW_A_ACUTE: &'static [u8] = &[0x08, 0x40];
pub static DISPLAY_SPECIAL_LOW_I_ACUTE: &'static [u8] = &[0x08, 0x41];
pub static DISPLAY_SPECIAL_LOW_O_ACUTE: &'static [u8] = &[0x08, 0x42];
pub static DISPLAY_SPECIAL_LOW_U_ACUTE: &'static [u8] = &[0x08, 0x43];
pub static DISPLAY_SPECIAL_LOW_N_TILDE: &'static [u8] = &[0x08, 0x44];
pub static DISPLAY_SPECIAL_CAP_N_TILDE: &'static [u8] = &[0x08, 0x45];
pub static DISPLAY_SPECIAL_LOW_A_MACRON: &'static [u8] = &[0x08, 0x46];
pub static DISPLAY_SPECIAL_LOW_O_MACRON: &'static [u8] = &[0x08, 0x47];
pub static DISPLAY_SPECIAL_INVERTED_QUESTION: &'static [u8] = &[0x08, 0x48];
pub static DISPLAY_SPECIAL_RING_DIACRITIC: &'static [u8] = &[0x08, 0x49];
pub static DISPLAY_SPECIAL_INVERTED_EXCLAMATION: &'static [u8] = &[0x08, 0x4A];
pub static DISPLAY_SPECIAL_BLANK: &'static [u8] = &[0x08, 0x4B];
pub static DISPLAY_SPECIAL_LOW_O_STROKE: &'static [u8] = &[0x08, 0x4C];
pub static DISPLAY_SPECIAL_CAP_O_STROKE: &'static [u8] = &[0x08, 0x4D];
pub static DISPLAY_SPECIAL_LOW_C_ACUTE: &'static [u8] = &[0x08, 0x4E];
pub static DISPLAY_SPECIAL_CAP_C_ACUTE: &'static [u8] = &[0x08, 0x4F];
pub static DISPLAY_SPECIAL_LOW_C_BREVE: &'static [u8] = &[0x08, 0x50];
pub static DISPLAY_SPECIAL_CAP_C_BREVE: &'static [u8] = &[0x08, 0x51];
pub static DISPLAY_SPECIAL_LOW_D_CARON: &'static [u8] = &[0x08, 0x52];
pub static DISPLAY_SPECIAL_CAP_D_STROKE: &'static [u8] = &[0x08, 0x53];
pub static DISPLAY_SPECIAL_CAP_S_BREVE: &'static [u8] = &[0x08, 0x54];
pub static DISPLAY_SPECIAL_LOW_Z_BREVE: &'static [u8] = &[0x08, 0x55];
pub static DISPLAY_SPECIAL_CAP_Z_BREVE: &'static [u8] = &[0x08, 0x56];
pub static DISPLAY_SPECIAL_CAP_BETA: &'static [u8] = &[0x08, 0x57];
pub static DISPLAY_SPECIAL_LOW_S_BREVE: &'static [u8] = &[0x08, 0x58];
pub static DISPLAY_SPECIAL_LOW_BETA: &'static [u8] = &[0x08, 0x59];
pub static DISPLAY_SPECIAL_CAP_A_ACUTE: &'static [u8] = &[0x08, 0x5A];
pub static DISPLAY_SPECIAL_CAP_A_GRAVE: &'static [u8] = &[0x08, 0x5B];
pub static DISPLAY_SPECIAL_CAP_A_TILDE: &'static [u8] = &[0x08, 0x5C];
pub static DISPLAY_SPECIAL_LOW_A_TILDE: &'static [u8] = &[0x08, 0x5D];
pub static DISPLAY_SPECIAL_CAP_E_INVERTED_BREVE: &'static [u8] = &[0x08, 0x5E];
pub static DISPLAY_SPECIAL_CAP_I_ACUTE: &'static [u8] = &[0x08, 0x5F];
pub static DISPLAY_SPECIAL_CAP_O_TILDE: &'static [u8] = &[0x08, 0x60];
pub static DISPLAY_SPECIAL_LOW_O_TILDE: &'static [u8] = &[0x08, 0x61];
pub static DISPLAY_SPECIAL_EURO: &'static [u8] = &[0x08, 0x62];
pub static DISPLAY_SPECIAL_Y_KEY: &'static [u8] = &[0x08, 0x63];
pub static DISPLAY_SPECIAL_UP_ARROW: &'static [u8] = &[0x08, 0x64];
pub static DISPLAY_SPECIAL_DOWN_ARROW: &'static [u8] = &[0x08, 0x65];
pub static DISPLAY_SPECIAL_LEFT_ARROW: &'static [u8] = &[0x08, 0x66];
pub static DISPLAY_SPECIAL_RIGHT_ARROW: &'static [u8] = &[0x08, 0x67];
pub static DISPLAY_SPECIAL_PACMAN: &'static [u8] = &[0x08, 0x68];
pub static DISPLAY_SPECIAL_SAILBOAT: &'static [u8] = &[0x08, 0x69];
pub static DISPLAY_SPECIAL_BALL: &'static [u8] = &[0x08, 0x6A];
pub static DISPLAY_SPECIAL_TELEPHONE: &'static [u8] = &[0x08, 0x6B];
pub static DISPLAY_SPECIAL_HEART: &'static [u8] = &[0x08, 0x6C];
pub static DISPLAY_SPECIAL_CAR: &'static [u8] = &[0x08, 0x6D];
pub static DISPLAY_SPECIAL_HANDICAP: &'static [u8] = &[0x08, 0x6E];
pub static DISPLAY_SPECIAL_RHINO: &'static [u8] = &[0x08, 0x6F];
pub static DISPLAY_SPECIAL_MUG: &'static [u8] = &[0x08, 0x70];
pub static DISPLAY_SPECIAL_SATELLITE: &'static [u8] = &[0x08, 0x71];
pub static DISPLAY_SPECIAL_COPYRIGHT: &'static [u8] = &[0x08, 0x72];


// The following symbols don't seem to work on our BetaBrite sign
pub static DISPLAY_SPECIAL_MALE: &'static [u8] = &[0x08, 0x73];
pub static DISPLAY_SPECIAL_FEMALE: &'static [u8] = &[0x08, 0x74];
pub static DISPLAY_SPECIAL_BOTTLE: &'static [u8] = &[0x08, 0x75];
pub static DISPLAY_SPECIAL_DISKETTE: &'static [u8] = &[0x08, 0x76];
pub static DISPLAY_SPECIAL_PRINTER: &'static [u8] = &[0x08, 0x77];
pub static DISPLAY_SPECIAL_MUSIC_NOTE: &'static [u8] = &[0x08, 0x78];
pub static DISPLAY_SPECIAL_INFINITY: &'static [u8] = &[0x08, 0x79];

// Counters
pub static DISPLAY_COUNTER_1: &'static [u8] = &[0x08, 0x7A];
pub static DISPLAY_COUNTER_2: &'static [u8] = &[0x08, 0x7B];
pub static DISPLAY_COUNTER_3: &'static [u8] = &[0x08, 0x7C];
pub static DISPLAY_COUNTER_4: &'static [u8] = &[0x08, 0x7D];
pub static DISPLAY_COUNTER_5: &'static [u8] = &[0x08, 0x7E];

// Special Modes
pub static SHOW_DATE_MMDDYY: &'static [u8] = &[0x0B, 0x30];
pub static SHOW_SMALL_DOTS_31: &'static [u8] = &[0x14, 0x43];
pub static SHOW_TIME: &'static [u8] = &[0x13];
pub static NEW_PAGE: &'static [u8] = &[0x0C];
pub static NEW_LINE: &'static [u8] = &[0x0D];


// Functions
// preprogrammed write special
pub fn write_special(mode: &[u8]) -> Vec<u8> {
    let mut cmd_vec = vec![0x02, 0x45];
    cmd_vec.extend(mode);
    return cmd_vec;
}

// write special given the mode and data
pub fn write_special_data(mode: &[u8], data: &[u8]) -> Vec<u8> {
    let mut cmd_vec = write_special(mode);
    cmd_vec.extend(data);
    return cmd_vec;
}

// preprogrammed write small dots
pub fn write_small_dots(data: &[u8]) -> Vec<u8> {
    let mut cmd_vec = vec![0x49, 0x41];
    cmd_vec.extend(data);
    return cmd_vec;
}

pub fn write_text(mode: &[u8], text: Vec<u8>, start: bool) -> Vec<u8> {
    // 0x02 - STX - precede command code
    // 0x41 - WRITE TEXT MODE
    // 0x30 - PRIORITY TEXT FILE
    // 0x1b - START OF MODE FIELD
    // 0x30 - DISPLAY POSITION - FILL (irrelevant on our sign)
    let mut file_vec = vec![0x02, 0x41, 0x30, 0x1b, 0x30];

    // if in a chain, don't send STX and friends
    if !start {
        file_vec.drain(..3);
    }
    file_vec.extend(mode);
    file_vec.extend(text);
    return file_vec;
}

// concat a slice of bytestrings into a single vec
pub fn concat_bytestrings(i: &[&[u8]]) -> Vec<u8> {
    let mut file_vec: Vec<u8> = Vec::new();
    for slice in i {
        file_vec.extend_from_slice(slice);
    }
    return file_vec;
}
