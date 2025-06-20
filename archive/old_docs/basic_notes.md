# Goals

We want to build a bidirectional, extensible and highly-performant transliterator for indic languages that is easy to use, change, and test. 
Transliteration of indic scripts poses several challenges:
1. indic scripts are an abugida and secondly, they encode basic consonants with the implicit 'a'.
2. roman scripts are an alphabet and the 'a' is explicit.
   - moreover, romanizations are either purely ascii based or unicode based.  

To build our transliterator, we want to structure it as an llvm compiler with a dual intermediate representation. 
1. each indic script maps an abugida 
2. each roman script maps an alphabet 
3. the compiler then maintains a mapping between the abugida and alphabet. 
4. transliteration thus is from abugida to abugida, abugida to alphabet, alphabet to abugida or alphabet to alphabet.
5. each of these intermediate representations have to be runtime-extensible using a simple toml / yaml syntax mapping system to be able to handle idosyncratic sources. 



1. round-trip verification
2. for each of the n supported base scripts, we want round-trip tests for all n^2 pairs (enforced for any new addition)
3. extensibility in the form of a toml that maps
4. once the core is built, 
   - we want to write a comprehensive suite of tests 
   - add as many scripts (and pairwise tests to confirm to point 2
5. build python bindings as well as wasm bindings 
6. benchmark against vidyut, aksharamukha and dharmamitra


