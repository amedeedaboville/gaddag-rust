from string import ascii_uppercase as au
gcg="""#character-encoding UTF-8
#player1 p1 p1
#player2 p2 p2
>p1: IDRBOP? H4 BIPOD +26 26
>p2: YRUAADI I7 YAUD +25 25
>p1: R?IEJMW G7 JoW +48 74
>p2: RAIRTGA G3 RAG +12 37
>p1: RIEMDOS 11G MISDOER +79 153
>p2: IRTAINL 12J AIL +15 52
>p1: OSOOHOA J4 OOHS +29 182
>p2: RTINWZE 12D WIZEN +43 95
>p1: OOANUAC 11B ANOA +17 199
>p2: RTVHRUR 10B HUT +35 130
>p1: OUCFUFE K2 CUFF +39 238
>p2: RVRRIIE 2K .IRRI +14 144
>p1: OUECAID O1 O.DIA +21 259
>p2: VREEIVT M2 .EVIVE +26 170
>p1: UECETES 1G CUTES +34 293
>p2: RTQENEX 8M REX +43 213
>p1: EEOINLE F12 .EE +14 307
>p2: TQNEETT L1 T.T +16 229
>p1: OINLESG A4 LEGIONS +84 391
>p2: QNEETMY 13C EYN. +27 256
>p1: GTAKPLL F2 PAK +26 417
>p2: QETM?BA 14A QAT +26 282
>p1: GTLLOEN B5 LOT +14 431
>p2: EM?BRAN 15A gRAB +59 341
>p1: GLEN H11 ..GLE +18 449
>p2: EMN N7 N.EM +14 355
>p2:  (N) +2 357"""
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