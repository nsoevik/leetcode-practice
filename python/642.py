class Node:
    def __init__(self, occurences) -> None:
        self.occurences = occurences
        self.next_nodes = {}

class Trei:
    def __init__(self) -> None:
        self.head = Node(0)

    def add_word(self, word):
        temp = self.head
        for i in range(len(word)):
            if word[i] not in temp.next_nodes:
                temp.next_nodes[word[i]] = Node(0)
            if i == len(word) - 1:
                temp.next_nodes[word[i]].occurences += 1
            temp = temp.next_nodes[word[i]]

    def get_following_words(self, word):
        ret = []
        temp = self.head
        for i in range(len(word)):
            if word[i] not in temp.next_nodes:
                return ret
            temp = temp.next_nodes[word[i]]


        

class AutocompleteSystem:

    def __init__(self, sentences: List[str], times: List[int]):
        

    def input(self, c: str) -> List[str]:
        

