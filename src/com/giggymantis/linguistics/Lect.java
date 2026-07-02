package com.giggymantis.linguistics;

import java.util.Dictionary;
import java.util.Hashtable;

public class Lect {
	private Dictionary<Integer, Word> dictionary; // May be the most literal usage of Dictionary in coding ever?
	public String name;
	
	public Lect() {	
		this.name = "";
		this.dictionary = new Hashtable<Integer, Word>();
	}
	public Lect(Dictionary<Integer, Word> dictionary) {
		this.name = "";
		this.dictionary = dictionary;
	}
	public Lect(String name) {
		this.name = name;
		this.dictionary = new Hashtable<Integer, Word>();
	}
	public Lect(Dictionary<Integer, Word> dictionary, String name) {
		this.name = name;
		this.dictionary = dictionary;
	}
	
	public void put(int id, Word word) {
		dictionary.put(id, word);
	}
	
	public Word get(int id) {
		return dictionary.get(id);
	}
	
}
