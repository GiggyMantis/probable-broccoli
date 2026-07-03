package com.giggymantis.probable_broccoli;

import java.nio.file.Paths;
import java.util.Arrays;

import com.giggymantis.linguistics.*;

public class Main {

	public static void main(String[] args) {
		SoundChange sc = new SimplePostconditionedSoundChange("a", "y", "f");
		try {
			Lect[] langs = FileLoader.readCSV(Paths.get(args[0]), true, true);
			Word w = langs[0].get(3);
			System.out.printf("Reversing sound change %s on word %s... result: %s", sc, w, UtilsConstants.trimEdges(sc.applyReverse(w).toString()));
		} catch (Exception e) {
			e.printStackTrace();
		}
	}

}
