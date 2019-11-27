from string import ascii_uppercase as au
gcg="""#character-encoding UTF-8
#player1 p1 p1
#player2 p2 p2
>p1: RRD?MEV H7 REV +12 12
>p2: TMAPIRE I1 PRIMATE +66 66
>p1: RD?MOUI G2 hUMIDOR +67 79
>p2: NIFEDNE J3 FINE +24 90
>p1: EXOHITO 1H O.HITE +36 115
>p2: DNEOTJR 1F JO +14 104
>p1: XOAWUYE K5 YEOW +35 150
>p2: DNETRNY K8 .YTED +48 152
>p1: XAULECA F4 AXAL +35 185
>p2: NRNSA?S N8 .NSNAReS +68 220
>p1: UECPLGO 15I GLUCO.E +39 224
>p2: AOEIQCN 6M QA. +13 233
>p1: PRSAVUI N1 SPIV +30 254
>p2: OEICNBZ I13 BONZE. +40 273
>p1: RAUDEEG L10 GEE. +28 282
>p2: ICWSEBK G14 SKEW +39 312
>p1: RAUDIDN F10 DURIAN +27 309
>p2: ICBTHAT D12 BI.TH +28 340
>p1: DRGTOEU 4L GU. +14 323
>p2: CATLIOF 15A FALCO. +42 382
>p1: DRTOEIA 13C OID.A +25 348
>p2: TILNA 5D TA.. +22 404
>p1: RTE 11E R.E +18 366
>p2: ILN 14A IN +13 417
>p1: T E4 T. +7 373
>p1:  (L) +2 375"""
for move in gcg.split("\n")[3:]:
    player, rack, pos, word, score, total = move.split()
    if pos[0].isdigit():
        direc = "utils::Direction::Across"
        row, col = pos[:-1], pos[-1]
    else:
        direc = "utils::Direction::Down"
        col, row = pos[0], pos[1:]
    row = str(int(row) - 1)
    col = au.index(col)

    print('board.play_word(utils::Position { row: %s, col: %s }, String::from("%s"), %s, true);' % (row, col, word, direc))
