class MyHashSet:

    def __init__(self):
        self.length = 100
        self.arr = [[] for _ in range(self.length)]

    def add(self, key: int) -> None:
        hashed_value = (key * 2069) % self.length
        for stored_key in self.arr[hashed_value]:
            if stored_key == key:
                return

        self.arr[hashed_value].append(key)

    def remove(self, key: int) -> None:
        hashed_value = (key * 2069) % self.length
        stored_keys = self.arr[hashed_value]
        index_to_swap = -1
        for i in range(len(stored_keys)):
            if stored_keys[i] == key:
                index_to_swap = i
        if index_to_swap == -1:
            return
        stored_keys[index_to_swap] = stored_keys[-1]
        stored_keys.pop()

    def contains(self, key: int) -> bool:
        hashed_value = (key * 2069) % self.length
        for stored_key in self.arr[hashed_value]:
            if stored_key == key:
                return True
        return False

        

