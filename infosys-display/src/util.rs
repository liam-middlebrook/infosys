extern crate chrono;

use bb::*;

use self::chrono::naive::time::NaiveTime;
use self::chrono::Timelike;

pub fn get_naivetime_now() -> NaiveTime {
    let curtime = chrono::prelude::Local::now();

    return NaiveTime::from_hms(
        curtime.hour(),
        curtime.minute(),
        curtime.second());
}


// Parser for Input Strings into BB strings
#[derive(PartialEq)]
#[derive(Debug)]
enum ParserState {
    Normal,
    EscapeCatch,
    EscapeSpecial,
    EscapeSpecialHex,
    EscapeSpecialHexLong,
}

pub fn tuple_to_bytestring(input: (String, String)) -> (Vec<u8>, Vec<u8>) {
    let mut mode_vec: Vec<u8> = Vec::new();
    let mut str_vec: Vec<u8> = Vec::new();

    // Parse modetype
    mode_vec.extend(match input.0.as_ref() {
        "STANDARD_ROTATE" => MODE_STANDARD_ROTATE,
        "STANDARD_HOLD" => MODE_STANDARD_HOLD,
        "STANDARD_FLASH" => MODE_STANDARD_FLASH,
        "STANDARD_ROLL_UP" => MODE_STANDARD_ROLL_UP,
        "STANDARD_ROLL_DOWN" => MODE_STANDARD_ROLL_DOWN,
        "STANDARD_ROLL_LEFT" => MODE_STANDARD_ROLL_LEFT,
        "STANDARD_ROLL_RIGHT" => MODE_STANDARD_ROLL_RIGHT,
        "STANDARD_WIPE_UP" => MODE_STANDARD_WIPE_UP,
        "STANDARD_WIPE_DOWN" => MODE_STANDARD_WIPE_DOWN,
        "STANDARD_WIPE_LEFT" => MODE_STANDARD_WIPE_LEFT,
        "STANDARD_WIPE_RIGHT" => MODE_STANDARD_WIPE_RIGHT,
        "STANDARD_SCROLL" => MODE_STANDARD_SCROLL,
        "STANDARD_AUTO_MODE" => MODE_STANDARD_AUTO_MODE,
        "STANDARD_ROLL_IN" => MODE_STANDARD_ROLL_IN,
        "STANDARD_ROLL_OUT" => MODE_STANDARD_ROLL_OUT,
        "STANDARD_WIPE_IN" => MODE_STANDARD_WIPE_IN,
        "STANDARD_WIPE_OUT" => MODE_STANDARD_WIPE_OUT,
        "STANDARD_COMPRESSED_ROTATE" => MODE_STANDARD_COMPRESSED_ROTATE,
        "STANDARD_EXPLODE" => MODE_STANDARD_EXPLODE,
        "STANDARD_CLOCK" => MODE_STANDARD_CLOCK,
        "SPECIAL_TWINKLE" => MODE_SPECIAL_TWINKLE,
        "SPECIAL_SPARKLE" => MODE_SPECIAL_SPARKLE,
        "SPECIAL_SNOW" => MODE_SPECIAL_SNOW,
        "SPECIAL_INTERLOCK" => MODE_SPECIAL_INTERLOCK,
        "SPECIAL_SWITCH" => MODE_SPECIAL_SWITCH,
        "SPECIAL_SLIDE" => MODE_SPECIAL_SLIDE,
        "SPECIAL_SPRAY" => MODE_SPECIAL_SPRAY,
        "SPECIAL_STARBURST" => MODE_SPECIAL_STARBURST,
        "SPECIAL_WELCOME" => MODE_SPECIAL_WELCOME,
        "SPECIAL_SLOT_MACHINE" => MODE_SPECIAL_SLOT_MACHINE,
        "SPECIAL_NEWS_FLASH" => MODE_SPECIAL_NEWS_FLASH,
        "SPECIAL_TRUMPET" => MODE_SPECIAL_TRUMPET,
        "SPECIAL_CYCLE_COLORS" => MODE_SPECIAL_CYCLE_COLORS,
        "SPECIAL_THANK_YOU" => MODE_SPECIAL_THANK_YOU,
        "SPECIAL_NO_SMOKING" => MODE_SPECIAL_NO_SMOKING,
        "SPECIAL_DONT_DRINK_AND_DRIVE" => MODE_SPECIAL_DONT_DRINK_AND_DRIVE,
        "SPECIAL_RUNNING_ANIMAL_OR_FISH" => MODE_SPECIAL_RUNNING_ANIMAL_OR_FISH,
        "SPECIAL_FIREWORKS" => MODE_SPECIAL_FIREWORKS,
        "SPECIAL_BALLOON_ANIMATION" => MODE_SPECIAL_BALLOON_ANIMATION,
        "SPECIAL_CHERRY_BOMB" => MODE_SPECIAL_CHERRY_BOMB,
        _ => MODE_STANDARD_AUTO_MODE,
    });

    // string parse
    str_vec.extend(parse_string_to_infosys_blob(input.1.as_ref()));

    return (mode_vec, str_vec);
}

fn parse_string_to_infosys_blob(input: &str) -> Vec<u8> {
    let mut out_vec: Vec<u8> = Vec::new();

    let mut escape_vec: String = String::new();
    let mut escape_catch: ParserState = ParserState::Normal;
    for c in input.chars() {
        if c == '\\' {
            escape_catch = ParserState::EscapeCatch;
            continue;
        }

        if escape_catch == ParserState::EscapeCatch {
            if c == 'n' {
                escape_catch = ParserState::Normal;
                out_vec.extend(&[0x0D]);
            } else if c == 'S' {
                escape_catch = ParserState::EscapeSpecial;
            }
            continue;
        }

        if escape_catch == ParserState::EscapeSpecial {
            if c == '_' {
                escape_catch = ParserState::EscapeSpecialHexLong;
                continue;
            }
            if c == '-' {
                escape_catch = ParserState::EscapeSpecialHex;
                continue;
            }
            // invalid state!
            escape_catch = ParserState::Normal;
            continue;
        }

        if escape_catch == ParserState::EscapeSpecialHex {
            escape_vec.push(c);
            if escape_vec.len() == 2 {
                out_vec.extend(split_octet_to_u8(&escape_vec));
                escape_vec.clear();
                escape_catch = ParserState::Normal;
                continue;
            }
            continue;
        }

        if escape_catch == ParserState::EscapeSpecialHexLong {
            escape_vec.push(c);
            if escape_vec.len() == 4 {
                out_vec.extend(split_octet_to_u8(&escape_vec));
                escape_vec.clear();
                escape_catch = ParserState::Normal;
                continue;
            }
            continue;
        }

        out_vec.push(c as u8);
    }

    return out_vec;
}

fn split_octet_to_u8(inn: &String) -> Vec<u8> {
    let mut out_vec: Vec<u8> = Vec::new();
    let mut input: String =inn.clone();
    while input.len() > 0 {
        let mut p1: u8 = (input.remove(0) as u8) - 48;
        let mut p2: u8 = (input.remove(0) as u8) - 48;
        if p1 > 9 {
            p1 -= 7;
        }
        if p2 > 9 {
            p2 -= 7;
        }
        let total: u8 = p1* 16 + p2;
        out_vec.push(total);
    }
    return out_vec;
}

pub fn convert_to_vec8(input: Vec<(Vec<u8>, Vec<u8>)>) -> Vec<u8> {
    let mut out_vec: Vec<u8> = Vec::new();
    let mut start = true;
    for pair in input {
        out_vec.extend(write_text(pair.0.as_slice(), pair.1, start));
        start = false;
    }
    return out_vec;
}
