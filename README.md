# probable-broccoli
Open-source linguistic computational phylogenetics.

Goals for this project:
- Always 100% free for academics and hobbyists.
- Easy and clean software to compare the geneological relationship of languages.
- Multiple glottochronological models (Bayesian, Parsimonious, etc.)
- Parsimony based on both place and manner, as well as common cross-linguistic sound changes.
- Multiple sets of data types (lexicon, cognates, grammatical features, morphemes, phonology, etc.)
- Reasonably resistant to word borrowings.
- Predicting features of most recent common ancestors (not complete linguistic reconstruction, just features)
- Manual sets of synonyms or near-synonyms to auto-identify cognates.

# Plans for Model Structure
1. Create a preliminary tree model using a minimal distance joining model.
2. Evolve the tree using a Monte Carlo Markov chain method.
- To do this, we propose a random local change in the tree, and then calculate the probability of this tree using a likelihood model.
- We then randomly select this proposed change based on the likelihood (we do not always take the "correct" improvement because of the Monte Carlo law of large numbers)
3. Eventually, the tree will converge on the most probable tree.

# Eventual Plans
- Use the same MCMC method to establish sound changes to allow for the most accurate results.