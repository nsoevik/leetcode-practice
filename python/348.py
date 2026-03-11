class ScoreTracker:
    def __init__(self):
        self.scores = [0, 0]
    def score(self, player):
        self.scores[player-1]
        return self.scores[player-1]

class TicTacToe:

    def __init__(self, n: int):
        self.rows = {}
        for i in range(n):
            self.rows[i] = ScoreTracker()
        self.columns = {}
        for i in range(n):
            self.columns[i] = ScoreTracker()
        self.diagonal = ScoreTracker()
        
    def move(self, row: int, col: int, player: int) -> int:
        if self.rows[row].score(player) == n:
            return player
        if self.columns[col].score(player) == n:
            return player
        if row == col and self.diagonal.score(player) == n:
            return player

        
