package com.giggymantis.linguistics;

public class OrthographicWord implements Word {
	// Should be replaced with phone array once that is implemented.
	private String stringform;
	private String gloss;
	
	public OrthographicWord(String stringform, String gloss) {
		this.stringform = stringform;
		this.gloss = gloss;
	}
	
	public OrthographicWord(Object[] units, String gloss) { 
		this.gloss = gloss;
		String[] strArray = new String[units.length];
		for (int i = 0; i < units.length; i++) {
			strArray[i] = units[i].toString();
		}
		stringform = String.join("", strArray);
	}
	
	public OrthographicWord(Object[] units) { 
		this.gloss = "";
		String[] strArray = new String[units.length];
		for (int i = 0; i < units.length; i++) {
			strArray[i] = units[i].toString();
		}
		stringform = String.join("", strArray);
	}
	
	public OrthographicWord(String stringform) {
		this.stringform = stringform;
		this.gloss = "";
	}

	@Override
	public double distance(Word q) {
		return (double)levenshtein(this.stringform, q.toString());
	}
	
	@Override
	public String toString() {
		return stringform;
	}

	@Override
	public String[] getUnits() {
		return stringform.split("");
	}

	@Override
	public void setUnits(Object[] units) {
		String[] strArray = new String[units.length];
		for (int i = 0; i < units.length; i++) {
			strArray[i] = units[i].toString();
		}
		stringform = String.join("", strArray);
	}

	@Override
	public Object getUnit(int index) {
		return stringform.charAt(index);
	}

	@Override
	public void setUnit(int index, Object object) {
		StringBuilder sb = new StringBuilder(stringform);
		sb.setCharAt(index, (char)object);
		stringform = sb.toString();
	}
	
	@Override
	public String getGloss() {
		return gloss;
	}

	@Override
	public void setGloss(String newGloss) {
		gloss = newGloss;		
	}

	@Override
	public OrthographicWord withNewUnits(Object[] units) {
		return new OrthographicWord(units, gloss);
	}
	
	public static int levenshtein(String p, String q) {
		// Redefine p and q in terms of a (shorter string) and b (longer string).
		int aLen = Math.min(p.length(), q.length());
		int bLen = Math.max(p.length(), q.length());
		String a, b;
		if (aLen == p.length()) {
			a = p;
			b = q;
		} else {
			a = q;
			b = p;
		}
		
		// If one string is zero, the distance is the length of the other string.
		if (aLen == 0) {
			return bLen;
		}
		
		// Otherwise, iterate thru the first aLen letters of both strings and compare distance; plus the difference in length between a and b.
		int dist = bLen - aLen;
		for (int i = 0; i < aLen; i++) {
			dist += (a.charAt(i) == b.charAt(i)) ? 0 : 1; 
		}
		return dist;
	}
}
