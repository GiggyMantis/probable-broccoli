package com.giggymantis.phonetics;

public enum MethodOfArticulation {
	/*
	 * Methods of articulation are represented in a 2D phase space, which looks like the following:
	 * ********************************************** *
	 * 	3			Flap			Trill
	 * 	2	Click	Stop			Fr.Trl.	Approx.	Vowel
	 * 	1					Affr.	Fric.
	 * 		1		2		3		4		5		6
	 * ********************************************** *
	 * ("Fr.Tril." is the fricated trill found phonemically in languages such as Czech and phonetically across the world.)
	 * 
	 * Two bytes are used per value, the high byte for the y-value and the low byte for the x-value.
	 * This is stored in a single short called "packed_value".
	 */
	
	CLICK(1, 2),
	STOP(2, 2),
	FLAP(2, 3),
	AFFRICATE(3,1),
	TRILL(4,3),
	FRICATED_TRILL(4,2),
	FRICATIVE(4,1),
	APPROXIMANT(5,2),
	VOWEL(6,2);
	
	private final short packed_value;

	private MethodOfArticulation(int x, int y) {
		packed_value = (short)(((byte)x << 8) + (byte)y);
	}
	
	private MethodOfArticulation(byte x, byte y) {
		packed_value = (short)((x << 8) + y);
	}
	
	public byte getX() {
		return (byte)(packed_value >>> 8);
	}
	
	public byte getY() {
		return (byte)packed_value;
	}
	
	public static int distanceSquared(MethodOfArticulation a, MethodOfArticulation b) {
		int x_dist = a.getX() - b.getX();
		int y_dist = a.getY() - b.getY();
		return (x_dist * x_dist) + (y_dist * y_dist);
	}
	
	public static double distance(MethodOfArticulation a, MethodOfArticulation b) {
		return Math.sqrt((double)distanceSquared(a, b));
	}
	
	public static boolean validate(MethodOfArticulation q) {
		switch (q) {
			case AFFRICATE:
				return true;
			case APPROXIMANT:
				return true;
			case CLICK:
				return true;
			case FLAP:
				return true;
			case FRICATED_TRILL:
				return true;
			case FRICATIVE:
				return true;
			case STOP:
				return true;
			case TRILL:
				return true;
			case VOWEL:
				return true;
			default:
				return false;
		}
	}
}
