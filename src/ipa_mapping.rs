use regex::{Regex};

pub(crate) fn to_broccoli_sampa(s: &String) -> String {
    let mut formatted_string: String = geminate(s);

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
            'q' => ret.push_str("Dqpf"),
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
            'r' => ret.push_str("Ctrv"),
            'ɾ' => ret.push_str("Ctⱱv"),
            'ɽ' => ret.push_str("Cʈⱱv"),
            'ʀ' => ret.push_str("Dqrv"),
            'ɺ' => ret.push_str("Ctⱱvl"),

            'ɸ' => ret.push_str("Lpɸf"),
            'β' => ret.push_str("Lpɸv"),
            'f' => ret.push_str("Lɱɸf"),
            'v' => ret.push_str("Lɱɸv"),
            'θ' => ret.push_str("Ctɸf"),
            'ð' => ret.push_str("Ctɸv"),
            's' => ret.push_str("Ctsf"),
            'z' => ret.push_str("Ctsv"),
            'ʃ' => ret.push_str("Cʈsf"),
            'ʒ' => ret.push_str("Cʈsv"),
            'ʂ' => ret.push_str("Cʈɸf"),
            'ʐ' => ret.push_str("Cʈɸv"),
            'ç' => ret.push_str("Dcɸf"),
            'ʝ' => ret.push_str("Dcɸv"),
            'ɕ' => ret.push_str("Dcsf"),
            'ʑ' => ret.push_str("Dcsv"),
            'x' => ret.push_str("Dkɸf"),
            'ɣ' => ret.push_str("Dkɸv"),
            'χ' => ret.push_str("Dqɸf"),
            'ʁ' => ret.push_str("Dqɸv"),
            'ħ' => ret.push_str("Gʡɸf"),
            'ʕ' => ret.push_str("Gʡɸv"),
            'ʜ' => ret.push_str("Gʡsf"),
            'ʢ' => ret.push_str("Gʡsv"),
            'h' => ret.push_str("Gʔɸf"),
            'ɦ' => ret.push_str("Gʔɸv"),

            'ʍ' => ret.push_str("DkwfW"),
            'w' => ret.push_str("DkwvW"),
            'ʋ' => ret.push_str("Lɱwv"),
            'ɥ' => ret.push_str("DcwvW"),
            'ɹ' => ret.push_str("Ctwv"),
            'ɻ' => ret.push_str("Cʈwv"),
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
            '˞' => ret.push_str("Ctwv"),

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

            'ɚ' => ret.push_str("CtwvW<"),
            'ɝ' => ret.push_str("CtwvW>"),

            '˥' => ret.push('5'),
            '˦' => ret.push('4'),
            '˧' => ret.push('3'),
            '˨' => ret.push('2'),
            '˩' => ret.push('1'),

            '.' => (),
            ' ' => (),

            _ => ret.push(c),
        }


    }

    // TODO: Make the voiceless diacritics work.
    // TODO: Make aspirates closer to fricatives
    // TODO: Make affricates closer to plosives
    // TODO: Make e and o colored vowels closer to a colored vowels
    // TODO: (maybe) make l closer to w
    // TODO: Make nasal vowels closer to nasal consonants

    return ret;
}

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