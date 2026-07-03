package com.giggymantis.probable_broccoli;

import java.nio.file.Paths;

import com.giggymantis.linguistics.*;

public class Main {

	public static void main(String[] args) {
		SimpleSoundChange ssc = new SimpleSoundChange("a", "y");
		try {
			Lect[] langs = FileLoader.readCSV(Paths.get(args[0]), true, true);
			Word w = langs[0].get(3);
			System.out.println(ssc.apply(w));
		} catch (Exception e) {
			e.printStackTrace();
		}
	}

}
