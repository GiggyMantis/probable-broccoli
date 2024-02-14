fn to_broccoli_sampa(s:String) -> String {
    let ret = String::new();
    for c in s.chars() {
        match c {
            'p' => ret = ret.push_str("Lppf");
            'b' => ret = ret.push_str("Lppv");
            't' => ret = ret.push_str("Ctpf");
            'd' => ret = ret.push_str("Ctpv");
            'ʈ' => ret = ret.push_str("Cʈpf");
            'ɖ' => ret = ret.push_str("Cʈpv");
            'c' => ret = ret.push_str("Dcpf");
            'ɟ' => ret = ret.push_str("Dcpv");
            'k' => ret = ret.push_str("Dkpf")
            'ɡ' => ret = ret.push_str("Dkpv");
            'g' => ret = ret.push_str("Dkpv");
            'q' => ret = ret.push_str("Dqpf");
            'ɢ' => ret = ret.push_str("Dqpv");
            'ʡ' => ret = ret.push_str("Gʡpf");
            'ʔ' => ret = ret.push_str("Gʔpf");

            'm' => ret = ret.push_str("Lpmv");
            'ɱ' => ret = ret.push_str("Lɱmv");
            'n' => ret = ret.push_str("Ctmv");
            'ɳ' => ret = ret.push_str("Cʈmv");
            'ɲ' => ret = ret.push_str("Dcmv");
            'ŋ' => ret = ret.push_str("Dkmv");
            'ɴ' => ret = ret.push_str("Dqmv");

            'ʙ' => ret = ret.push_str("Lprv");
            'ⱱ' => ret = ret.push_str("Lɱⱱv");
            'r' => ret = ret.push_str("Ctrv");
            'ɾ' => ret = ret.push_str("Ctⱱv");
            'ɽ' => ret = ret.push_str("Cʈⱱv");
            'ʀ' => ret = ret.push_str("Dqrv");
            'ɺ' => ret = ret.push_str("Ctⱱvl");

            'ɸ' => ret = ret.push_str("Lpɸf");
            'β' => ret = ret.push_str("Lpɸv");
            'f' => ret = ret.push_str("Lɱɸf");
            'v' => ret = ret.push_str("Lɱɸv");
            'θ' => ret = ret.push_str("Ctɸf");
            'ð' => ret = ret.push_str("Ctɸv");
            's' => ret = ret.push_str("Ctsf");
            'z' => ret = ret.push_str("Ctsv");
            'ʃ' => ret = ret.push_str("Cʈsf");
            'ʒ' => ret = ret.push_str("Cʈsv");
            'ʂ' => ret = ret.push_str("Cʈɸf");
            'ʐ' => ret = ret.push_str("Cʈɸv");
            'ç' => ret = ret.push_str("Dcɸf");
            'ʝ' => ret = ret.push_str("Dcɸv");
            'ɕ' => ret = ret.push_str("Dcsf");
            'ʑ' => ret = ret.push_str("Dcsv");
            'x' => ret = ret.push_str("Dkɸf");
            'ɣ' => ret = ret.push_str("Dkɸv");
            'χ' => ret = ret.push_str("Dqɸf");
            'ʁ' => ret = ret.push_str("Dqɸv");
            'ħ' => ret = ret.push_str("Gʡɸf");
            'ʕ' => ret = ret.push_str("Gʡɸv");
            'ʜ' => ret = ret.push_str("Gʡsf"); 
            'ʢ' => ret = ret.push_str("Gʡsv");
            'h' => ret = ret.push_str("Gʔɸf");
            'ɦ' => ret = ret.push_str("Gʔɸv");

            'ʍ' => ret = ret.push_str("Lkwf");
            'w' => ret = ret.push_str("Lkwv");
            'ʋ' => ret = ret.push_str("Lɱwv");
            'ɥ' => ret = ret.push_str("Lcwv");
            'ɹ' => ret = ret.push_str("Ctwv");
            'ɻ' => ret = ret.push_str("Cʈwv");
            'j' => ret = ret.push_str("Dcwv");
            'ɰ' => ret = ret.push_str("Dkwv");
            'ɬ' => ret = ret.push_str("Ctɸfl");
            'ɮ' => ret = ret.push_str("Ctɸvl");
            'l' => ret = ret.push_str("Ctwvl");
            'ɫ' => ret = ret.push_str("Ctwvlg");
            'ɭ' => ret = ret.push_str("Cʈwvl");
            'ʎ' => ret = ret.push_str("Dcwvl");
            'ʟ' => ret = ret.push_str("Dkwvl");
            'ʘ' => ret = ret.push_str("Lp!");
            'ǀ' => ret = ret.push_str("Ct!");
            'ǃ' => ret = ret.push_str("Cʈ!");
            'ǂ' => ret = ret.push_str("Dc!");
            'ǁ' => ret = ret.push_str("Ct!l");
            'ʼ' => ret = ret.push_str("z");
            'ɓ' => ret = ret.push_str("Lppvz");
            'ɗ' => ret = ret.push_str("Ctpvz");
            'ʄ' => ret = ret.push_str("Dcpfz");
            'ɠ' => ret = ret.push_str("Dkpvz");
            'ʛ' => ret = ret.push_str("Dqpvz");
            'ɧ' => ret = ret.push_str("Cʈsfz");

            _ => ret = ret.push(c); 
        }
    }
}