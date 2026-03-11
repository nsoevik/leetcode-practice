class Solution:
    def numIslands(self, grid: List[List[str]]) -> int:
        directions = [(-1, 0), (0, -1), (1, 0), (0, 1)]

        visited = set()
        def dfs(i, j, grid):
            if (i, j) in visited or grid[i][j] == 0:
                return
            visited.add((i, j))
            for d in directions:
                next_i = i + d[0]
                next_j = j + d[1]
                if next_i < 0 or next_i >= len(grid) or next_j < 0 or next_j >= len(grid[i]):
                    continue
                dfs((next_i, next_j))

        count = 0
        for i in range(len(grid)):
            for j in range(len(grid[i])):
                if grid[i][j] == 1 and (i, j) not in visited:
                    count += 1
                    dfs(i, j, grid)
        return count




                

        

