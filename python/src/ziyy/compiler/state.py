from ..value import *
class State:
    def __init__(self):
        self.tags = [""]
        self.saves = [RESET]

    def push(self, tag: str, string: str):
        l = len(self.saves) - 1
        s = self.saves[l]
        self.saves.append(s + string)
        self.tags.append(tag)

    def pop(self) -> tuple[str, str]:
        a = self.tags.pop()
        b = self.saves.pop()
        return a, b

    def current_tag(self) -> str:
        l = len(self.tags) - 1
        return self.tags[l]

    def current_save(self) -> str:
        l = len(self.saves) - 1
        return self.saves[l]