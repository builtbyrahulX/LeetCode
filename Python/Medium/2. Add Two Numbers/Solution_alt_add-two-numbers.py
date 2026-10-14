# Definition for singly-linked list.
# class ListNode(object):
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next

class Solution(object):
    def addTwoNumbers(self, l1, l2):
        """
        :type l1: Optional[ListNode]
        :type l2: Optional[ListNode]
        :rtype: Optional[ListNode]
        """
        dummy_head = ListNode(0)
        current = dummy_head
        carry = 0

        while l1 or l2 or carry:
            val1 = l1.val if l1 else 0
            val2 = l2.val if l2 else 0

            # Calculate total digit sum and update carry
            total = val1 + val2 + carry
            carry = total // 10

            # Append the single digit result
            current.next = ListNode(total % 10)
            current = current.next

            # Move to the next nodes if available
            if l1:
                l1 = l1.next
            if l2:
                l2 = l2.next

        return dummy_head.next