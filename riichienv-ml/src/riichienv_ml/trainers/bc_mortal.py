"""Mortal BC trainer — AGPL-isolated.

This module requires `libriichi` (Mortal's AGPL-licensed library) which is NOT
listed in pyproject.toml dependencies. Import is lazy to ensure the rest of the
package works without it.

Only used via configs/4p/bc_mortal.yml (Mortal is 4P-only).
"""


def _require_libriichi():
    """Lazily import libriichi; raise clear error if not installed."""
    try:
        import libriichi  # noqa: F811
        return libriichi
    except ImportError:
        raise ImportError(
            "libriichi is required for Mortal BC training but is not installed. "
            "Install it from https://github.com/Equim-chan/Mortal (AGPL-3.0 licensed). "
            "Note: libriichi is intentionally NOT listed in riichienv-ml's dependencies "
            "due to its AGPL license."
        )


class MortalBCTrainer:
    """Behavior cloning trainer using Mortal's libriichi for data loading.

    This trainer uses Mortal's replay parser and feature encoder to train
    a policy network via behavior cloning on Mortal-format replay data.

    AGPL Notice: Using this trainer means your code must comply with AGPL-3.0.
    """

    def __init__(self, config_path: str):
        self.libriichi = _require_libriichi()
        self.config_path = config_path

    def train(self):
        raise NotImplementedError(
            "MortalBCTrainer.train() is a placeholder. "
            "Implement Mortal-specific BC logic here using libriichi."
        )
