package com.giggymantis.probable_broccoli;

import java.util.Arrays;
import java.util.HashSet;

public class UtilsConstants {

	public static String trimEdges(String s) {
		StringBuilder sb = new StringBuilder(s);
		sb.deleteCharAt(s.length() - 1);
		sb.deleteCharAt(0);
		return sb.toString();
	}
	
	public static Object[] removeDuplicates(Object[] q) {
		return new HashSet<Object>(Arrays.asList(q)).toArray();
	}
	
}
