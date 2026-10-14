# 📝 1358. Number of Substrings Containing All Three Characters (LeetCode)

🔗 [Problem Link](https://leetcode.com/problems/number-of-substrings-containing-all-three-characters)

![Difficulty](https://img.shields.io/badge/Difficulty-Medium-orange) ![Language](https://img.shields.io/badge/Language-Kotlin-blue)

### 💡 Tags
Hash Table, String, Sliding Window

### 🚀 Performance
- **Runtime:** N/A
- **Memory:** N/A

---

### 📜 Problem Description

Given a string  `s`  consisting only of characters  *a* ,  *b*  and  *c* .

Return the number of substrings containing  **at least**  one occurrence of all these characters  *a* ,  *b*  and  *c* .

**Example 1:**

```
Input: s = "abcabc"
Output: 10
Explanation: The substrings containing at least one occurrence of the characters a, b and c are "abc", "abca", "abcab", "abcabc", "bca", "bcab", "bcabc", "cab", "cabc" and "abc" (again). 

```

**Example 2:**

```
Input: s = "aaacb"
Output: 3
Explanation: The substrings containing at least one occurrence of the characters a, b and c are "aaacb", "aacb" and "acb". 

```

**Example 3:**

```
Input: s = "abc"
Output: 1

```

**Constraints:**

	
- `3 <= s.length <= 5 x 104`
	
- `s`  only consists of  `'a'` ,  `'b'`  or  `'c'`  characters.