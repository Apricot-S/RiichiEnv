import polars as pl
import numpy as np

import torch
from torch.utils.data import Dataset


class RankPredictorDataset(Dataset):
    def __init__(self, dataframe: pl.DataFrame, n_players: int = 4):
        self.n_players = n_players

        # Build normalization columns dynamically
        exprs = [
            (pl.col("chang") / 3).cast(pl.Float32).alias("chang"),
            (pl.col("ju") / 3).cast(pl.Float32).alias("ju"),
            (pl.col("ben") / 4).cast(pl.Float32).alias("ben"),
            (pl.col("liqibang") / 4).cast(pl.Float32).alias("liqibang"),
        ]
        for i in range(n_players):
            exprs.extend([
                (pl.col(f"p{i}_init_score") / 25000.0).cast(pl.Float32).alias(f"p{i}_init_score"),
                (pl.col(f"p{i}_end_score") / 25000.0).cast(pl.Float32).alias(f"p{i}_end_score"),
                (pl.col(f"p{i}_delta_score") / 12000.0).cast(pl.Float32).alias(f"p{i}_delta_score"),
            ])

        self.df = dataframe.with_columns(*exprs)

    def __len__(self):
        return len(self.df) * self.n_players

    def __getitem__(self, idx):
        n = self.n_players
        row = self.df.row(idx // n, named=True)
        player_idx = idx % n

        scores = np.array(
            [row[f"p{i}_init_score"] for i in range(n)]
            + [row[f"p{i}_end_score"] for i in range(n)]
            + [row[f"p{i}_delta_score"] for i in range(n)]
        )
        round_meta = np.array([
            row["chang"], row["ju"], row["ben"], row["liqibang"]
        ])
        player = np.zeros(n)
        player[player_idx] = 1.0
        x = np.concatenate([scores, round_meta, player])
        y = np.zeros(n)
        y[row[f"p{player_idx:d}_rank"]] = 1.0

        return torch.Tensor(x), torch.Tensor(y)
