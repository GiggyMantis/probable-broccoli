package com.giggymantis.probable_broccoli;

import java.nio.file.Paths;

import com.giggymantis.linguistics.*;

public class Main {

	public static void main(String[] args) {
		try {
			Lect[] langs = FileLoader.readCSV(Paths.get(args[0]), true, true);
			System.out.println(langs[0].name);
		} catch (Exception e) {
			e.printStackTrace();
		}
	}

}
