class Solution(object):
    def minOperations(self, s):
        """
        :type s: str
        :rtype: int
        """
        n = len(s)
        t = sorted(s)
        
        if list(s) == t:
            return 0
        
        if n == 2:
            return -1
        
        min_char = t[0]
        max_char = t[-1]
        
        if s[0] == min_char or s[-1] == max_char:
            return 1
        
        has_min_in_prefix = min_char in s[:n - 1]
        has_max_in_suffix = max_char in s[1:]
        
        if s[0] == max_char and s[-1] == min_char and not has_min_in_prefix and not has_max_in_suffix:
            return 3
            
        return 2