%%time
import pandas as pd
import math
from IPython.display import clear_output
from itertools import combinations
from collections import defaultdict

orda = ord('a')

def string_to_bin(s):
    bitmask = 0
    for c in s:
        if 'a' <= c <= 'z':
            bitmask |= 1 << (ord(c) - orda)
    return bitmask

def print_bin(i):
    print("zyxwvutsrqponmlkjihgfedcba")
    print(f"{i:026b}")

def find_three_words_that_cover_most_used_letters(file_path, word_length):
    # Predefined letter frequency dictionary
    letter_frequency = {
        'e': 349588141984, 't': 247577342738, 'a': 243662684512, 'o': 228025627088,
        'i': 223353030415, 'n': 207910712159, 's': 207080253606, 'r': 201896673641,
        'l': 130649920346, 'c': 113913698859, 'd': 107605388542, 'h': 106367962556,
        'u': 86950627146, 'm': 84155576549, 'p': 77553040250, 'g': 63045208347,
        'f': 61328927423, 'y': 52941043438, 'b': 49798922187, 'w': 44294405401,
        'v': 34402346309, 'k': 24380950863, 'x': 9151143994, 'j': 7637833834,
        'q': 4218467887, 'z': 4192477980
    }

    # Precompute the most frequent letters
    sorted_letters = sorted(letter_frequency, key=letter_frequency.get, reverse=True)
    desired_letters = ''.join(sorted_letters[:3 * word_length])
    dl_bin = string_to_bin(desired_letters)
    
    df = pd.read_csv(file_path)
    
    old_word_collection = [word for word in df['word'] if type(word) == str and len(word) == word_length and len(set(word)) == word_length]
    bin_lookup = defaultdict(list)
    word_collection = set()
    for s in old_word_collection:
        s_bin = string_to_bin(s)
        if (s_bin & dl_bin) == s_bin:
            bin_lookup[s_bin].append(s)
            word_collection.add(s_bin)

    print(math.comb(len(word_collection), 3))
    # 579_544_309_975
    #  93_008_596_184 | len(set(word)) == word_length
    #   1_637_974_191 | bin_lookup = defaultdict(list) +++ word_collection = set()

    word_collection = list(word_collection)
    result = []
    for ai in range(len(word_collection)):
        for bi in range(ai+1, len(word_collection)):
            if (word_collection[ai] & word_collection[bi]) == 0:
                for ci in range(bi+1, len(word_collection)):
                    combined_letters = word_collection[ai]|word_collection[bi]|word_collection[ci]
                    if combined_letters == dl_bin:
                        result.append((word_collection[ai], word_collection[bi], word_collection[ci]))
        
        clear_output(wait=True)
        # print((ai/(len(word_collection)-1))*100, "%")

    print(math.comb(len(word_collection), 3))
    return result, bin_lookup

file_path = "/workspace/aoc/henrik/unigram_freq.csv"
res, bin_lookup = find_three_words_that_cover_most_used_letters(file_path, 3)