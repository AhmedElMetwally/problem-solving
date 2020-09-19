//
// Link: https://leetcode.com/problems/merge-two-sorted-lists
//


/**
 *
 *
 * Definition for singly-linked list.
 * function ListNode(val, next) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.next = (next===undefined ? null : next)
 * }
 */
/**
 * Solution 1
 *
 * @param {ListNode} l1
 * @param {ListNode} l2
 * @return {ListNode}
 */
const mergeTwoLists = (l1, l2) => {
  const head = new ListNode();
  let current = head;

  while (l1 && l2) {
    if (l1.val > l2.val) {
      current.next = l2;
      l2 = l2.next;
    } else {
      current.next = l1;
      l1 = l1.next;
    }

    current = current.next;
  }

  if (l1 !== null) {
    current.next = l1;
  } else {
    current.next = l2;
  }

  return head.next;
};
