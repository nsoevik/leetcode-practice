class Node:
    def __init__(self, is_word = False) -> None:
        self.next_letters = {}
        self.is_word = is_word

    def search(self, word) -> bool:

class WordDictionary:

    def __init__(self):
        self.head = Node()

    def addWord(self, word: str) -> None:
        temp = self.head
        for i in range(len(word)):
            if word[i] in temp.next_letters:
                temp = temp.next_letters[c]
            else:
                temp.next_letters[c] = Node()
                temp = temp.next_letters[c]
            if i == len(word) - 1:
                temp.is_word = True

    def _search(self, node: Node):


    def search(self, word: str) -> bool:

        temp = self.head
        for i in range(len(word)):
            if word[i] in temp.next_letters:
                temp = temp.next_letters[c]
            else:
                return False
            if i == len(word) - 1:
                return temp.is_word

        


