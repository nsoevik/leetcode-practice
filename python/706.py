class Node:
    def __init__(self, key, val):
        self.key = key
        self.val = val


class MyHashMap:

    def __init__(self):
        self.table_size = 997
        self.table = [[] for _ in range(self.table_size)] 

    def _get_hash_key(self, key: int) -> int:
        return key % self.table_size

    def put(self, key: int, value: int) -> None:
        entries = self.table[self._get_hash_key(key)]
        for entry in entries:
            if entry.key == key:
                entry.val = value
                return

        entries.append(Node(key, value))

    def get(self, key: int) -> int:
        entries = self.table[self._get_hash_key(key)]
        for entry in entries:
            if entry.key == key:
                return entry.val
        return -1

    def remove(self, key: int) -> None:
        entries = self.table[self._get_hash_key(key)]
        for i in range(len(entries)):
            if entries[i].key == key:
                if i != len(entries) - 1:
                    last = entries[-1]
                    entries[i] = last
                entries.pop()
                return
