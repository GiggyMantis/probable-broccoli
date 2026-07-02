package com.giggymantis.probable_broccoli;

import java.io.FileNotFoundException;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;

import com.giggymantis.linguistics.Lect;
import com.giggymantis.linguistics.OrthographicWord;

public class FileLoader {
	
	public static Lect[] readCSV(Path filepath, boolean firstColumnIsGloss, boolean firstRowIsHeader) throws FileNotFoundException, IOException {
		List<String> file = Files.readAllLines(filepath);
		
		// Get the number of languages, which is equal to the number of commas in each line plus one, or not plus one if the first column represents the gloss value.
		int langcount = file.get(0).length() - file.get(0).replace(",", "").length() + (firstColumnIsGloss ? 0 : 1);
		Lect[] retArray = new Lect[langcount];
		for (int i = 0; i < retArray.length; i++) {
			retArray[i] = new Lect();
		}
		
		for (int i = 0; i < file.size(); i++) {
			String[] splitLine = file.get(i).split(",");
			int offset = 0;
			String gloss = "";
			if (firstColumnIsGloss) {
				gloss = splitLine[0];
				offset = -1;
			}
			
			for (int j = 0; j < splitLine.length; j++) {
				// Skip first column if it is gloss as we have already handled that.
				if ((j == 0) && (firstColumnIsGloss)) {
					continue;
				}
				
				// If it is the first row and the first row is a header, then set the names of the lects.
				if ((i == 0) && firstRowIsHeader) {
					retArray[j + offset].name = splitLine[j];
					continue;
				}
				
				//TODO: Implement other word types.
				retArray[j + offset].put(i, new OrthographicWord(splitLine[j], gloss));
			}
		}
		
		return retArray;
	}
}
