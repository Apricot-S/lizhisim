# SPDX-FileCopyrightText: 2025 Apricot S.
# SPDX-License-Identifier: MIT
# This file is part of https://github.com/Apricot-S/lizhisim

# ruff: noqa: FBT001, S101, PLR2004

from dataclasses import dataclass

from mahjong.hand_calculating.hand import HandCalculator
from mahjong.hand_calculating.hand_config import OptionalRules


@dataclass(frozen=True)
class FuFanAnalysis:
    fu: int
    fan: int


class Tool:
    def __init__(
        self,
        has_open_tanyao: bool,
        has_double_yakuman: bool,
        kiriage: bool,
        renhou_as_yakuman: bool,
    ) -> None:
        self._options = OptionalRules(
            has_open_tanyao=has_open_tanyao,
            has_aka_dora=True,
            has_double_yakuman=has_double_yakuman,
            kiriage=kiriage,
            renhou_as_yakuman=renhou_as_yakuman,
        )
        self._hand_calculator = HandCalculator()

    def calculate_fu_fan(
        self,
        bingpai136: list[int],
        hulepai37: int,
    ) -> FuFanAnalysis:
        assert len(bingpai136) in (1, 4, 7, 10, 13)
        assert 0 <= hulepai37 < 37

        response = self._hand_calculator.estimate_hand_value(
            tiles=bingpai136,
            win_tile=hulepai37,
        )

        if response.error is not None:
            raise RuntimeError(response.error)

        assert response.fu is not None
        assert response.han is not None

        return FuFanAnalysis(fu=response.fu, fan=response.han)
