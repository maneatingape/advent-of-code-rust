//! # Elves Look, Elves Say
//!
//! There is a trick to solve this problem in constant time and space. While this is not possible
//! for any arbitrary sequence, in Advent of Code *we only need to solve for our given input*.
//!
//! Examining the input shows that it consists of one of Conway's
//! [atomic elements](https://en.wikipedia.org/wiki/Look-and-say_sequence#Cosmological_decay).
//! Each element breaks down into other elements that do not interact with each other. This means
//! that we only need to track the *count* of each element, rather than deal with the sequence
//! as a whole. Each step we replace the count of each element with its decay products. For example
//! if we had five `Ni` then next step we would decay to five `Zn` and five `Co`.
//!
//! Computing the result is simply multiplying the number of each element by its length. There are
//! 92 elements total so we can use a fixed size array to store the decay chain information.
//!
//! It would be possible (but less fun) to precompute all possible 92 answers into a
//! look up table.
use crate::util::hash::*;

const ELEMENTS: &str = "\
22 -> H -> H
13112221133211322112211213322112 -> He -> Hf Pa H Ca Li
312211322212221121123222112 -> Li -> He
111312211312113221133211322112211213322112 -> Be -> Ge Ca Li
1321132122211322212221121123222112 -> B -> Be
3113112211322112211213322112 -> C -> B
111312212221121123222112 -> N -> C
132112211213322112 -> O -> N
31121123222112 -> F -> O
111213322112 -> Ne -> F
123222112 -> Na -> Ne
3113322112 -> Mg -> Pm Na
1113222112 -> Al -> Mg
1322112 -> Si -> Al
311311222112 -> P -> Ho Si
1113122112 -> S -> P
132112 -> Cl -> S
3112 -> Ar -> Cl
1112 -> K -> Ar
12 -> Ca -> K
3113112221133112 -> Sc -> Ho Pa H Ca Co
11131221131112 -> Ti -> Sc
13211312 -> V -> Ti
31132 -> Cr -> V
111311222112 -> Mn -> Cr Si
13122112 -> Fe -> Mn
32112 -> Co -> Fe
11133112 -> Ni -> Zn Co
131112 -> Cu -> Ni
312 -> Zn -> Cu
13221133122211332 -> Ga -> Eu Ca Ac H Ca Zn
31131122211311122113222 -> Ge -> Ho Ga
11131221131211322113322112 -> As -> Ge Na
13211321222113222112 -> Se -> As
3113112211322112 -> Br -> Se
11131221222112 -> Kr -> Br
1321122112 -> Rb -> Kr
3112112 -> Sr -> Rb
1112133 -> Y -> Sr U
12322211331222113112211 -> Zr -> Y H Ca Tc
1113122113322113111221131221 -> Nb -> Er Zr
13211322211312113211 -> Mo -> Nb
311322113212221 -> Tc -> Mo
132211331222113112211 -> Ru -> Eu Ca Tc
311311222113111221131221 -> Rh -> Ho Ru
111312211312113211 -> Pd -> Rh
132113212221 -> Ag -> Pd
3113112211 -> Cd -> Ag
11131221 -> In -> Cd
13211 -> Sn -> In
3112221 -> Sb -> Pm Sn
1322113312211 -> Te -> Eu Ca Sb
311311222113111221 -> I -> Ho Te
11131221131211 -> Xe -> I
13211321 -> Cs -> Xe
311311 -> Ba -> Cs
11131 -> La -> Ba
1321133112 -> Ce -> La H Ca Co
31131112 -> Pr -> Ce
111312 -> Nd -> Pr
132 -> Pm -> Nd
311332 -> Sm -> Pm Ca Zn
1113222 -> Eu -> Sm
13221133112 -> Gd -> Eu Ca Co
3113112221131112 -> Tb -> Ho Gd
111312211312 -> Dy -> Tb
1321132 -> Ho -> Dy
311311222 -> Er -> Ho Pm
11131221133112 -> Tm -> Er Ca Co
1321131112 -> Yb -> Tm
311312 -> Lu -> Yb
11132 -> Hf -> Lu
13112221133211322112211213322113 -> Ta -> Hf Pa H Ca W
312211322212221121123222113 -> W -> Ta
111312211312113221133211322112211213322113 -> Re -> Ge Ca W
1321132122211322212221121123222113 -> Os -> Re
3113112211322112211213322113 -> Ir -> Os
111312212221121123222113 -> Pt -> Ir
132112211213322113 -> Au -> Pt
31121123222113 -> Hg -> Au
111213322113 -> Tl -> Hg
123222113 -> Pb -> Tl
3113322113 -> Bi -> Pm Pb
1113222113 -> Po -> Bi
1322113 -> At -> Po
311311222113 -> Rn -> Ho At
1113122113 -> Fr -> Rn
132113 -> Ra -> Fr
3113 -> Ac -> Ra
1113 -> Th -> Ac
13 -> Pa -> Th
3 -> U -> Pa";

type Result = (usize, usize);

pub fn parse(input: &str) -> Result {
    let elements: Vec<Vec<_>> =
        ELEMENTS.lines().map(|line| line.split_ascii_whitespace().collect()).collect();
    let mut indices = FastMap::with_capacity(92 * 2);

    // Map both sequence and element name to indices.
    for (i, tokens) in elements.iter().enumerate() {
        indices.insert(tokens[0], i);
        indices.insert(tokens[2], i);
    }

    // Build list of decay chains.
    let sizes: Vec<_> = elements.iter().map(|e| e[0].len()).collect();
    let decays: Vec<Vec<_>> =
        elements.iter().map(|e| e[4..].iter().map(|t| indices[t]).collect()).collect();

    // Each input is a single element.
    let mut current = [0; 92];
    current[indices[input.trim()]] = 1;

    for _ in 0..40 {
        current = step(&current, &decays);
    }
    let part1 = length(&current, &sizes);

    for _ in 0..10 {
        current = step(&current, &decays);
    }
    let part2 = length(&current, &sizes);

    (part1, part2)
}

pub fn part1(input: &Result) -> usize {
    input.0
}

pub fn part2(input: &Result) -> usize {
    input.1
}

fn step(current: &[usize], decays: &[Vec<usize>]) -> [usize; 92] {
    let mut next = [0; 92];

    for (i, &count) in current.iter().enumerate() {
        if count > 0 {
            for &element in &decays[i] {
                next[element] += count;
            }
        }
    }

    next
}

fn length(current: &[usize], sizes: &[usize]) -> usize {
    current.iter().zip(sizes.iter()).map(|(c, s)| c * s).sum()
}
