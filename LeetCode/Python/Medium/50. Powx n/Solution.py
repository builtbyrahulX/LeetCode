class Solution:
    def myPow(self, x: float, n: int) -> float:
        # Base case: any number raised to 0 is 1
        if n == 0:
            return 1.0

        # Handle negative exponents: x^(-n) = (1/x)^n
        exp = n
        if exp < 0:
            x = 1.0 / x
            exp = -exp

        result = 1.0
        current_product = x

        while exp > 0:
            # If current bit of exponent is 1, multiply into result
            if exp % 2 == 1:
                result *= current_product

            # Square the base and halve the exponent
            current_product *= current_product
            exp //= 2

        return result