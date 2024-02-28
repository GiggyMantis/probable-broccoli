use regex::{Regex};

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

pub(crate) fn to_binary_vec(s: &String) -> Vec<u16> {
    let mut ret: Vec<u16> = Vec::new();
    // TODO: Seperate out modifiers
    for c in geminate(s).chars() {
        ret.push(to_binary(c, &""));
    }
    ret
}

pub(crate) fn to_binary(c: &char, modifiers: &str) -> u16 {
    /* Bitwise Layout of Phonemes (Little Endian)
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
    let mut ret = 0u16;
    if "qrɾɽʀʁɹɻɺɚɝ".contains(c) || modifiers.contains('˞') {
        ret = 1u16;
    }
    if "mɱnɳɲŋɴ".contains(c) || modifiers.contains('̃') || modifiers.contains('ⁿ') {
        ret |= 2u16;
    }
    if "ɬɮlɭʎʟɫɺǁ".contains(c) || modifiers.contains('ˡ') {
        ret |= 4u16;
    }
    if "hɦʔ".contains(c) || modifiers.contains('̰') || modifiers.contains('ˀ') {
        ret |= 8u16;
    }
    if "qɢɴʀχʁħʕʛʜʢʡ".contains(c) || modifiers.contains('ˤ') {
        ret |= 16u16;
    }
    if "kɡgŋxɣɰʟɠʍw".contains(c) || modifiers.contains('ˠ') {
        ret |= 32u16;
    }
    if "cɟɲçʝjʎʄɥɕʑ".contains(c) || modifiers.contains('ʲ') {
        ret |= 64u16;
    }
    if "pbmɱʙⱱɸβfvʋʘʍwɥyʉuʊʏøɵoœɞɔɶɒ".contains(c) || modifiers.contains('ʷ') {
        ret |= 128u16;
    }

    // ʘǀǃǂǁ (clicks are by default 0 which is why they have no if statement here)

    if "ɓɗʄɠʛ".contains(c) {
        ret |= 256u16; // 1
    } else if "ⱱɾɽɺ".contains(c) {
        ret |= 512u16; // 10
    } else if "pbtdʈɖcɟkɡgqɢʔʢʡ".contains(c) && modifiers.contains('ʰ') {
        ret |= 1024u16; // 100
    } else if "pbtdʈɖcɟkɡgqɢʔʢʡ".contains(c) && modifiers.contains('ʼ') {
        ret |= 1280u16; // 101
    } else if "pbtdʈɖcɟkɡgqɢʔʢʡ".contains(c) {
        ret |= 768u16; // 11
    } else if "ʧʨʦꭧʤʥʣꭦ".contains(c) && modifiers.contains('ʰ') {
        ret |= 1792u16; // 111
    } else if "ʧʨʦꭧʤʥʣꭦ".contains(c) && modifiers.contains('ʼ') {
        ret |= 2048u16; // 1000
    } else if "ʧʨʦꭧʤʥʣꭦ".contains(c) {
        ret |= 1536u16; // 110
    } else if "ɸβfvθðʂʐçʝxɣχʁħʕhɦɧʜszʃʒɕʑ".contains(c) && modifiers.contains('ʼ') {
        ret |= 2816u16; // 1011
    } else if "ɸβfvθðʂʐçʝxɣχʁħʕhɦɧʜ".contains(c) {
        ret |= 2304u16; // 1001
    } else if "szʃʒɕʑ".contains(c) {
        ret |= 2560u16; // 1010
    } else if "ʙrʀ".contains(c) && modifiers.contains('̝') {
        ret |= 3072u16; // 1100
    } else if "ʙrʀ" {
        ret |= 3328u16; // 1101
    } else if "mɱnɳɲŋɴʋɹɻjɰlɭʎʟʍwɥ" {
        ret |= 3584u16; // 1110
    } else if "iyɨʉɯuɪʏʊeøɘɵɤoəɛœɜɞʌɔæɐaɶɑɒ" {
        ret |= 3840u16; // 1111
    }

    // Bilabial sounds are by default 0 so they have no if statement here
    if "ɱⱱfvʋ".contains(c) {
        ret |= 4096u16; // 1
    } else if "tdnrɾθðszɹɬɮlǀǃ!|".contains(c) {
        ret |= 8192u16; // 10
    } //TODO: continue

    ret
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