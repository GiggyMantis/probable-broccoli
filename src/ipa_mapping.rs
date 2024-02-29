use std::cmp;
use simple_matrix::Matrix;
use regex::{Regex};
use strsim::{normalized_damerau_levenshtein, normalized_levenshtein};

pub(crate) fn to_broccoli_sampa(s: &String) -> String {
    let mut formatted_string: String = geminate(&s.replace("r̝̊", "ʃ˞").replace("r̝", "ʒ˞"));

    let mut ret = String::new();
    for c in formatted_string.chars() {
        match c {
            'p' => ret.push_str("Lppf"),
            'b' => ret.push_str("Lppv"),
            't' => ret.push_str("Ctpf"),
            'd' => ret.push_str("Ctpv"),
            'ʈ' => ret.push_str("Cʈpf"),
            'ɖ' => ret.push_str("Cʈpv"),
            'c' => ret.push_str("Dcpf"),
            'ɟ' => ret.push_str("Dcpv"),
            'k' => ret.push_str("Dkpf"),
            'ɡ' => ret.push_str("Dkpv"),
            'g' => ret.push_str("Dkpv"),
            'q' => ret.push_str("DqpfR"),
            'ɢ' => ret.push_str("Dqpv"),
            'ʡ' => ret.push_str("Gʡpf"),
            'ʔ' => ret.push_str("Gʔpf"),

            'm' => ret.push_str("Lpmv"),
            'ɱ' => ret.push_str("Lɱmv"),
            'n' => ret.push_str("Ctmv"),
            'ɳ' => ret.push_str("Cʈmv"),
            'ɲ' => ret.push_str("Dcmv"),
            'ŋ' => ret.push_str("Dkmv"),
            'ɴ' => ret.push_str("Dqmv"),

            'ʙ' => ret.push_str("Lprv"),
            'ⱱ' => ret.push_str("Lɱⱱv"),
            'r' => ret.push_str("CtrvR"),
            'ɾ' => ret.push_str("CtⱱvR"),
            'ɽ' => ret.push_str("CʈⱱvR"),
            'ʀ' => ret.push_str("DqrvR"),
            'ɺ' => ret.push_str("CtⱱvlR"),

            'ɸ' => ret.push_str("Lpɸfh"),
            'β' => ret.push_str("Lpɸvh"),
            'f' => ret.push_str("Lɱɸfh"),
            'v' => ret.push_str("Lɱɸvh"),
            'θ' => ret.push_str("Ctɸfh"),
            'ð' => ret.push_str("Ctɸvh"),
            's' => ret.push_str("Ctsfh"),
            'z' => ret.push_str("Ctsvh"),
            'ʃ' => ret.push_str("Cʈsfh"),
            'ʒ' => ret.push_str("Cʈsvh"),
            'ʂ' => ret.push_str("Cʈɸfh"),
            'ʐ' => ret.push_str("Cʈɸvh"),
            'ç' => ret.push_str("Dcɸfh"),
            'ʝ' => ret.push_str("Dcɸvh"),
            'ɕ' => ret.push_str("Dcsfh"),
            'ʑ' => ret.push_str("Dcsvh"),
            'x' => ret.push_str("Dkɸfh"),
            'ɣ' => ret.push_str("Dkɸvh"),
            'χ' => ret.push_str("Dqɸfh"),
            'ʁ' => ret.push_str("DqɸvhR"),
            'ħ' => ret.push_str("Gʡɸfh"),
            'ʕ' => ret.push_str("Gʡɸvh"),
            'ʜ' => ret.push_str("Gʡsfh"),
            'ʢ' => ret.push_str("Gʡsvh"),
            'h' => ret.push_str("Gʔɸfh"),
            'ɦ' => ret.push_str("Gʔɸvh"),

            'ʍ' => ret.push_str("DkwfW"),
            'w' => ret.push_str("DkwvW"),
            'ʋ' => ret.push_str("Lɱwv"),
            'ɥ' => ret.push_str("DcwvW"),
            'ɹ' => ret.push_str("CtwvR"),
            'ɻ' => ret.push_str("CʈwvR"),
            'j' => ret.push_str("Dcwv"),
            'ɰ' => ret.push_str("Dkwv"),
            'ɬ' => ret.push_str("Ctɸfl"),
            'ɮ' => ret.push_str("Ctɸvl"),
            'l' => ret.push_str("Ctwvl"),
            'ɫ' => ret.push_str("Ctwvlg"),
            'ɭ' => ret.push_str("Cʈwvl"),
            'ʎ' => ret.push_str("Dcwvl"),
            'ʟ' => ret.push_str("Dkwvl"),

            'ʘ' => ret.push_str("Lp!"),
            'ǀ' => ret.push_str("Ct!"),
            'ǃ' => ret.push_str("Cʈ!"),
            'ǂ' => ret.push_str("Dc!"),
            'ǁ' => ret.push_str("Ct!l"),

            'ʼ' => ret.push('z'),
            'ɓ' => ret.push_str("Lppvz"),
            'ɗ' => ret.push_str("Ctpvz"),
            'ʄ' => ret.push_str("Dcpfz"),
            'ɠ' => ret.push_str("Dkpvz"),
            'ʛ' => ret.push_str("Dqpvz"),

            'ɧ' => ret.push_str("Cʈsfz"),
            'ʰ' => ret.push('h'),
            'ⁿ' => ret.push('n'),
            'ˤ' => ret.push('a'),
            'ˠ' => ret.push('g'),
            'ʲ' => ret.push('j'),
            'ʷ' => ret.push('W'),

            '̃' => ret.push('n'),
            'ˈ' => ret.push('^'),
            'ˌ' => (),
            '‿' => (),
            '˞' => ret.push_str("R"),

            'ꭧ' => ret.push_str("CʈɸfT"),
            'ꭦ' => ret.push_str("CʈɸvT"),
            'ʧ' => ret.push_str("CʈsfT"),
            'ʤ' => ret.push_str("CʈsvT"),
            'ʨ' => ret.push_str("DcsfT"),
            'ʥ' => ret.push_str("DcsvT"),
            'ʦ' => ret.push_str("CtsfT"),
            'ʣ' => ret.push_str("CtsvT"),

            'i' => ret.push_str("Dcwv@+"),
            'y' => ret.push_str("DcwvW+"),
            'ɨ' => ret.push_str("DXwv@+"),
            'ʉ' => ret.push_str("DXwvW+"),
            'ɯ' => ret.push_str("Dkwv@+"),
            'u' => ret.push_str("DkwvW+"),
            'ɪ' => ret.push_str("Dcwv@-"),
            'ʏ' => ret.push_str("DcwvW-"),
            'ʊ' => ret.push_str("DkwvW-"),
            'e' => ret.push_str("Dqwv@+"),
            'ø' => ret.push_str("DqwvW+"),
            'ɘ' => ret.push_str("DXwv@-"),
            'ɵ' => ret.push_str("DXwvW+"),
            'ɤ' => ret.push_str("Gʡwv@+"),
            'o' => ret.push_str("GʡwvW+"),
            'ə' => ret.push_str("Ctwv@-"),
            'ɛ' => ret.push_str("Dqwv@-"),
            'œ' => ret.push_str("DqwvW-"),
            'ɜ' => ret.push_str("Dʔwv@-"),
            'ɞ' => ret.push_str("DʔwvW-"),
            'ʌ' => ret.push_str("Gʡwv@-"),
            'ɔ' => ret.push_str("GʡwvW-"),
            'ɐ' => ret.push_str("Gʡwv@+"),
            'æ' => ret.push_str("Gʔwv@<"),
            'ɶ' => ret.push_str("GʔwvW<"),
            'a' => ret.push_str("Gʔwv@+"),
            'ɑ' => ret.push_str("Gʔwv@>"),
            'ɒ' => ret.push_str("GʔwvW>"),

            'ɚ' => ret.push_str("Ctwv@-R"),
            'ɝ' => ret.push_str("Dʔwv@-R"),

            '˥' => ret.push_str("|----"),
            '˦' => ret.push_str("|---0"),
            '˧' => ret.push_str("|--00"),
            '˨' => ret.push_str("|-000"),
            '˩' => ret.push_str("|0000"),

            '.' => (),
            ' ' => (),
            '[' => (),
            ']' => (),
            '/' => (),

            _ => ret.push(c),
        }


    }

    // TODO: Make the voiceless diacritics work.
    // TODO: Make aspirates closer to fricatives
    // TODO: Make affricates closer to plosives
    // TODO: Make e and o colored vowels closer to a colored vowels
    // TODO: (maybe) make l closer to w
    // TODO: Make nasal vowels closer to nasal consonants
    // TODO: Make the rhotic fricatives closer to palatalized trill
    // TODO: Make "-ized" consonants closer to their modifier

    return ret;
}

pub(crate) fn compare(s1: &String, s2: &String) -> f64 {
    1.0 - normalized_levenshtein(&*format_binary_vec(to_binary_vec(s1)), &*format_binary_vec(to_binary_vec(s2)))
}

fn format_binary_vec(v: Vec<u32>) -> String {
    let mut ret = String::new();
    for i in v.iter().rev() {
        ret.push_str(&*i.to_string());
    }
    ret
}

pub(crate) fn to_binary_vec(s: &String) -> Vec<u32> {
    let mut ret: Vec<u32> = Vec::new();
    let mut ch = '\\';
    let mut mods = String::new();
    for c in geminate(s).chars() {
        if "ˈ˩˨˧˦˥ʼʰⁿˡˤˠʲʷ̰̝̊̃˞".contains(c) {
            mods.push(c);
            continue;
        }
        if ch != '\\' {
            ret.push(to_binary(ch, &*mods));
        }
        ch = c;
    }
    ret
}

pub(crate) fn to_binary(c: char, modifiers: &str) -> u32 {
    /* Bitwise Layout of Phonemes (Little Endian)
        Rest of Bits store 5digit base-6 tone information. (0 is no tone, then increasing in order of low to high pitch)
        One Bit   - 0 or 1, representing whether it is stressed (vowels only)
        One Bit   - 0 or 1, representing whether it is voiced
        Four Bits - representing this enum [Bilabial, Labiodental, Alveolar/Dental, Retroflex/Postalveolar/Alveolopalatal, Palatal/Palatalalveolar, Velar, Uvular, Pharyngeal/Epiglottal, Glottal]
        Four Bits - representing this enum [Click, Implosive, Tap, Plosive, Aspirate/Breathy-voiced Plosive, Ejected Plosive, Affricate, Aspirate Affricate, Fricative, Sibilant Fric, Ejected Fricative, Fricative-Trill, Trill, True Sonorant, True Vowel]
        One Bit   - 0 or 1, representing whether it is labialized (round)
        One Bit   - 0 or 1, representing whether it is palatalized
        One Bit   - 0 or 1, representing whether it is velarized
        One Bit   - 0 or 1, representing whether it is emphatic (uvularized/pharyngealized)
        One Bit   - 0 or 1, representing whether it is glotallized
        One Bit   - 0 or 1, representing whether it is lateral
        One Bit   - 0 or 1, representing whether it is nasal(ized)
        One Bit   - 0 or 1, representing whether it is rhotic 
    */
    let mut ret = 0u32;
    if "qrɾɽʀʁɹɻɺɚɝ".contains(c) || modifiers.contains('˞') {
        ret = 1u32;
    }
    if "mɱnɳɲŋɴ".contains(c) || modifiers.contains('̃') || modifiers.contains('ⁿ') {
        ret |= 2u32;
    }
    if "ɬɮlɭʎʟɫɺǁ".contains(c) || modifiers.contains('ˡ') {
        ret |= 4u32;
    }
    if "hɦʔ".contains(c) || modifiers.contains('̰') || modifiers.contains('ˀ') {
        ret |= 8u32;
    }
    if "qɢɴʀχʁħʕʛʜʢʡ".contains(c) || modifiers.contains('ˤ') {
        ret |= 16u32;
    }
    if "kɡgŋxɣɰʟɠʍw".contains(c) || modifiers.contains('ˠ') {
        ret |= 32u32;
    }
    if "cɟɲçʝjʎʄɥɕʑ".contains(c) || modifiers.contains('ʲ') {
        ret |= 64u32;
    }
    if "pbmɱʙⱱɸβfvʋʘʍwɥyʉuʊʏøɵoœɞɔɶɒ".contains(c) || modifiers.contains('ʷ') {
        ret |= 128u32;
    }

    // Clicks are by default 0 which is why they have no if statement here

    if "ɓɗʄɠʛ".contains(c) {
        ret |= 256u32; // 1
    } else if "ⱱɾɽɺ".contains(c) {
        ret |= 512u32; // 10
    } else if "pbtdʈɖcɟkɡgqɢʔʢʡ".contains(c) && modifiers.contains('ʰ') {
        ret |= 1024u32; // 100
    } else if "pbtdʈɖcɟkɡgqɢʔʢʡ".contains(c) && modifiers.contains('ʼ') {
        ret |= 1280u32; // 101
    } else if "pbtdʈɖcɟkɡgqɢʔʢʡ".contains(c) {
        ret |= 768u32; // 11
    } else if "ʧʨʦꭧʤʥʣꭦ".contains(c) && modifiers.contains('ʰ') {
        ret |= 1792u32; // 111
    } else if "ʧʨʦꭧʤʥʣꭦ".contains(c) && modifiers.contains('ʼ') {
        ret |= 2048u32; // 1000
    } else if "ʧʨʦꭧʤʥʣꭦ".contains(c) {
        ret |= 1536u32; // 110
    } else if "ɸβfvθðʂʐçʝxɣχʁħʕhɦɧʜszʃʒɕʑ".contains(c) && modifiers.contains('ʼ') {
        ret |= 2816u32; // 1011
    } else if "ɸβfvθðʂʐçʝxɣχʁħʕhɦɧʜ".contains(c) {
        ret |= 2304u32; // 1001
    } else if "szʃʒɕʑ".contains(c) {
        ret |= 2560u32; // 1010
    } else if "ʙrʀ".contains(c) && modifiers.contains('̝') {
        ret |= 3072u32; // 1100
    } else if "ʙrʀ".contains(c) {
        ret |= 3328u32; // 1101
    } else if "mɱnɳɲŋɴʋɹɻjɰlɭʎʟʍwɥ".contains(c) {
        ret |= 3584u32; // 1110
    } else if "iyɨʉɯuɪʏʊeøɘɵɤoəɛœɜɞʌɔæɐaɶɑɒ".contains(c) {
        ret |= 3840u32; // 1111
    }

    // Bilabial sounds are by default 0, so they have no if statement here
    if "ɱⱱfvʋ".contains(c) {
        ret |= 4096u32; // 1
    } else if "tdnrɾθðszɹɬɮlɫǀǃ!|ɗ".contains(c) {
        ret |= 8192u32; // 10
    } else if "ʈɖɳɽʂʐʃʒɻɭ".contains(c) {
        ret |= 12288u32; // 11
    } else if "cɟɲçʝjʎɕʑǂʄɥ".contains(c) {
        ret |= 16384u32; // 100
    } else if "kɡgŋxɣɰʟɠɧ".contains(c) {
        ret |= 20480u32; // 101
    } else if "qɢɴʀχʁʛ".contains(c) {
        ret |= 24576u32; // 110
    } else if "ħʕʜʢʡ".contains(c) {
        ret |= 28672u32; // 111
    } else if "ʔhɦ".contains(c) {
        ret |= 32768u32; // 1000
    }

    if !("ptʈckqʔɸfθsʃʂçxχħhɬʘǀǃǂǁʍʜʡɕɧ".contains(c) || modifiers.contains('̊')) {
        ret |= 65536u32;
    }
    if modifiers.contains('ˈ') {
        ret |= 131072u32;
    }

    let re = Regex::new(r"[˩˨˧˦˥]").unwrap();

    if !re.is_match(modifiers) {
        return ret;
    }
    let mut alltones = 0u16;
    let mut power_of_six = 1u16;
    for i in re.captures_iter(modifiers) {
        alltones +=
            match &i[0] {
                "˩" => 1u16,
                "˨" => 2u16,
                "˧" => 3u16,
                "˦" => 4u16,
                "˥" => 5u16,
                _ => 0u16,
            } * power_of_six;
        power_of_six *= 6u16;
    }

    ret + ((alltones as u32) << 18)
}

// TODO: Make the geminate function properly geminate affricates into plosive+affricate and not affricate+affricate
fn geminate(input_string: &String) -> String {
    let mut output_string = String::new();
    for current_char in input_string.chars().rev().peekable() {
        output_string.push(
            match (current_char, *input_string.chars().rev().peekable().peek().unwrap()) {
                ('ː', prev) => prev,      //if current char is 'ː', push previous
                (current, _) => current,  //otherwise, push current
            }
        )
    }
    return output_string.chars().rev().collect();  //re-reverse the string to be forward again
}
/*
// https://en.wikipedia.org/wiki/Wagner%E2%80%93Fischer_algorithm
fn wagner_fischer(s: Vec<u32>, t: Vec<u32>) -> u32 {
    let mut d: Matrix<u32> = Matrix::new(s.len()+1,t.len()+1);

    for i in 0..s.len()+1 {
        d.set(i, 0, i as u32);
    }
    for j in 0..t.len()+1 {
        d.set(0, j, j as u32);
    }

    for j in 0..t.len() {
        for i in 0..s.len() {
            let substitution_cost = // s[i] ^ t[j];
                if s[i] == t[j] {0u32} else {1u32};

            d.set(i+1, j+1, cmp::min(cmp::min(
                *d.get(i, j+1).unwrap() + 1,
                *d.get(i+1, j).unwrap() + 1, ),
                *d.get(i, j).unwrap() + substitution_cost)
            );

        }
    }

    return *d.get(s.len(), t.len()).unwrap();
}
*/