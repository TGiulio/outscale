use std::io;
use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;

#[derive(FromPrimitive)]
enum MusicKey{
    A = 0,
    AdBb,
    B,
    C,
    CdDb,
    D,
    DdEb,
    E,
    F,
    FdGb,
    G,
    GdAb
}

enum ScaleType{
    Major = 0,
    Minor
}

struct Scale
{
    key: MusicKey,
    scale_type: ScaleType
}

const SEMITONES: usize = 12;
const SEMITONES_MAX_INDEX: usize = SEMITONES - 1;
const SEMITONE: usize = 1;
const TONE: usize = 2 * SEMITONE;

const SCALES : [[usize; 7]; 2] = [
    // MAJOR
    [TONE, TONE, SEMITONE, TONE, TONE, TONE, SEMITONE], 
    //MINOR
    [TONE, SEMITONE, TONE, TONE, SEMITONE, TONE, TONE]
];

const SOUNDS: [&str; SEMITONES] = ["A", "A#/Bb", "B", "C", "C#/Db", "D", "D#/Eb", "E", "F", "F#/Gb", "G", "G#/Ab"];


fn main() {
    let mut input: String = String::new();
    println!("write the scale you would like to know:");
    match io::stdin().read_line(&mut input){
        Ok(_size) => {
            if let Some(scale_req) = check_input(&input){
                println!("{:?}", generate_scale(scale_req));
            }else{
                println!("Can't compute this scale, please check your request"); 
            }
        }
        Err(error) => {
            println!("error: {}", error);
        }
                  
    }
}

fn generate_scale(scale_req: Scale) -> Vec<&'static str> {
    let mut index = scale_req.key as usize;
    let mut scale: Vec<&str> = vec![SOUNDS[index]];

    for interval in SCALES[ scale_req.scale_type as usize] {
        index += interval;
        match index{
            0..=SEMITONES_MAX_INDEX => {scale.push(SOUNDS[index]);},
            _ => {
                index-=12;
                scale.push(SOUNDS[index]);
            }
        }
    }
    scale
}

fn check_input(input: &str) -> Option<Scale>{
    let info = input.split_whitespace().collect::<Vec<&str>>();
    let mut scale_req= Scale {
        key: MusicKey::A, 
        scale_type: ScaleType::Major
    };
    match info.len(){
        2 => {
            if let Some(key_index) = SOUNDS.iter().position(|&x| x == info[0]){
                if let Some(s_type) = MusicKey::from_usize(key_index) {
                    scale_req.key = s_type;
                }else{
                    return None;
                }
            }else{
                return None;
            }
            match info[1]{
                "Major"|"M"|"MAJOR"|"MAJ"|"Maj"|"major"|"maj" => {
                    scale_req.scale_type = ScaleType::Major;
                    Some(scale_req)
                },
                "Minor"|"m"|"MINOR"|"MIN"|"Min"|"minor"|"min" => {
                    scale_req.scale_type = ScaleType::Minor;
                    Some(scale_req)
                },
                _ => {None}
            }
        },
        _ => {None}
    }
}

#[test]
fn generate_scale_test(){
    let scale_a_major= Scale {
        key: MusicKey::C, 
        scale_type: ScaleType::Major
    };
    assert_eq!(generate_scale(scale_a_major), vec!["C", "D", "E", "F", "G", "A", "B", "C"]);
    let scale_a_minor= Scale {
        key: MusicKey::A, 
        scale_type: ScaleType::Minor
    };
    assert_eq!(generate_scale(scale_a_minor), vec!["A", "B", "C", "D", "E", "F", "G", "A"]);
    let scale_e_major= Scale {
        key: MusicKey::E, 
        scale_type: ScaleType::Major
    };
    assert_eq!(generate_scale(scale_e_major), vec!["E", "F#/Gb", "G#/Ab", "A", "B", "C#/Db", "D#/Eb", "E"]);
}
