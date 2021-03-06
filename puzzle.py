import tkinter as tk

import subprocess
import base64
import string
import enum
import dataclasses

class Direction(enum.Enum):
    ACROSS = 1
    DOWN   = 2

    def flip(self):
        if self == Direction.ACROSS:
            return Direction.DOWN
        return Direction.ACROSS

    def __str__(self):
        return {Direction.ACROSS: '⇨', Direction.DOWN: '⇩'}[self]

    @property
    def x(self):
        return {Direction.ACROSS: 1, Direction.DOWN: 0}[self]

    @property
    def y(self):
        return {Direction.ACROSS: 0, Direction.DOWN: 1}[self]    

SPECIAL = {
    '#': 'red',
    '-': 'light blue',
    '+': 'dark blue',
    '^': 'pink',
    '*': 'pink'
}

@dataclasses.dataclass
class Puzzle:
    board: list
    rack: str
    moves: list
    
    @staticmethod
    def load_new(turns=10, difficulty=1):
        x = subprocess.check_output(["./target/release/gaddag-rust", "puzzle", "-d", str(difficulty), str(turns)])
        # x = open("blank_puzzle.txt").read().replace(r"\n", "\n").strip()
        puzzle = str(base64.b64decode(x.split()[-1]))[2:-1].replace(r"\n", "\n")
        board, rack, *moves = puzzle.split("\n")
        rack = list(rack)
        
        board = [board[i:i+15].replace(".", " ") for i in range(0, len(board), 15)]

        return Puzzle(board, rack, moves)

    def rank_of_move(self, position, word, direction):
        y, x = position
        c, r = str(y), string.ascii_uppercase[x - 1]
        s = [r + c, c + r][direction==Direction.ACROSS] + ' ' + word + ' '

        print(s, end='')
        
        try:
            move = [i for i in self.moves if i.startswith(s)][0]
            r = self.moves.index(move) + 1
            print(move.split()[2]) # print score
            return r
        except IndexError:
            print(" (invalid)")
            return None

# thanks https://gist.github.com/mp035/9f2027c3ef9172264532fcd6262f3b01 
class ScrollFrame(tk.Frame):
    def __init__(self, parent):
        super().__init__(parent) # create a frame (self)

        self.canvas = tk.Canvas(self, borderwidth=0, background="#ffffff")          #place canvas on self
        self.viewPort = tk.Frame(self.canvas, background="#ffffff")                    #place a frame on the canvas, this frame will hold the child widgets 
        self.vsb = tk.Scrollbar(self, orient="vertical", command=self.canvas.yview) #place a scrollbar on self 
        self.canvas.configure(yscrollcommand=self.vsb.set)                          #attach scrollbar action to scroll of canvas

        self.vsb.pack(side="right", fill="y")                                       #pack scrollbar to right of self
        self.canvas.pack(side="left", fill="both", expand=True)                     #pack canvas to left of self and expand to fil
        self.canvas_window = self.canvas.create_window((4,4), window=self.viewPort, anchor="nw",            #add view port frame to canvas
                                  tags="self.viewPort")

        self.viewPort.bind("<Configure>", self.onFrameConfigure)                       #bind an event whenever the size of the viewPort frame changes.
        self.canvas.bind("<Configure>", self.onCanvasConfigure)                       #bind an event whenever the size of the viewPort frame changes.

        self.onFrameConfigure(None)                                                 #perform an initial stretch on render, otherwise the scroll region has a tiny border until the first resize

    def onFrameConfigure(self, event):                                              
        '''Reset the scroll region to encompass the inner frame'''
        self.canvas.configure(scrollregion=self.canvas.bbox("all"))                 #whenever the size of the frame changes, alter the scroll region respectively.

    def onCanvasConfigure(self, event):
        '''Reset the canvas window to encompass inner frame when required'''
        canvas_width = event.width
        self.canvas.itemconfig(self.canvas_window, width = canvas_width)            #whenever the size of the canvas changes alter the window region respectively.



class GUI:
    def __init__(self, **kwargs):
        self.puzzle = Puzzle.load_new(**kwargs)
        self.original_rack = self.puzzle.rack[:]
        
        self.root = tk.Tk()

        self.board_frame = tk.Frame(self.root, borderwidth=1, relief=tk.RIDGE)

        self.labels = []

        for row in range(16):
            self.labels.append([])
            for col in range(16):
                color, fg = "white", "black"

                bw = 1
                relief=tk.FLAT
                
                if row == 0:
                    t = (" " + string.ascii_uppercase)[col]
                elif col == 0:
                    t = str(row).zfill(2)
                else:
                    t = self.puzzle.board[row - 1][col - 1]
                    if t.isalpha() and t == t.lower():
                        relief=tk.RIDGE
                        t = t.upper()
                        
                if t in SPECIAL:
                    color = SPECIAL[t]
                    t = ''

                frame = tk.Frame(self.board_frame, width=20, height=20, borderwidth=bw, relief=relief, bg=color)

                label = tk.Label(frame, text=t, bg=color)
                label.pack()

                frame.pack_propagate(0)
                frame.grid(row=row, column=col)

                label.bind("<Button-1>", self.click)
                
                self.labels[-1].append(label)
        self.board_frame.grid(row=0, column=0, rowspan=16, columnspan=16)

        self.update_rack_frame()

        self.current_direction = Direction.ACROSS
        self.square_at = None
        self.squares_changed = []

        self.root.bind("<Key>", self.type_char)
        self.root.bind("<Return>", self.enter_move)

        self.move_box = ScrollFrame(self.root)
        self.move_box.grid(row=0, column=16, rowspan=8, columnspan=2)

        self.move_btns = []
        self.ml = max(map(len, self.puzzle.moves))
        for i in range(1, len(self.puzzle.moves)):
            btn = tk.Button(self.move_box.viewPort, text=str(i), command=self.show(i - 1), font='TkFixedFont')
            btn.pack(fill=tk.X)

            self.move_btns.append(btn)

        self.max_time = 60000
        self.time_passed = 0
        self.tick = 1000

        self.timer_frame = tk.Frame(self.root)
        self.timer = tk.Label(self.timer_frame, font='TkFixedFont 24')
        
        self.timer_frame.grid(row=8, column=16, rowspan=8, columnspan=2)
        self.timer.pack()
        
        self.start_timer()

        self.can_submit = True
        self.moves_submitted = []
        self.lb = None
        
    def update_rack_frame(self):
        self.rack_frame = tk.Frame(self.root, width=100, height=40, borderwidth=1, relief=tk.SUNKEN)

        for c, l in enumerate(self.puzzle.rack):
            frame = tk.Frame(self.rack_frame, width=20, height=20, borderwidth=1, relief=tk.GROOVE)
            tk.Label(frame, text=l).pack()
            frame.grid(row=0, column=c)
            
        self.rack_frame.grid(row=16, column=3, columnspan=10, rowspan=2)        

    def click(self, e):
        if e.widget['text'].strip().isalpha():
            return
        elif not self.square_at:
            self.square_at = e.widget
            self.squares_changed.append(e.widget)
        elif e.widget == self.square_at:
            self.current_direction = self.current_direction.flip()
        else:
            for sq in self.squares_changed:
                self.reset(sq)
            self.puzzle.rack = self.original_rack[:]
            self.square_at = e.widget
            self.squares_changed = [e.widget]
        e.widget['text'] = str(self.current_direction)

    def reset(self, sq):
        sq.config(text='', fg='black', relief=tk.FLAT)

    def is_blank(self, sq):
        return sq['borderwidth'] == 1 and sq['relief'] == tk.RIDGE

    def type_char(self, e):
        c = e.char.upper()
        if self.square_at and (c in self.puzzle.rack or '?' in self.puzzle.rack):
            if c in self.puzzle.rack:
                self.puzzle.rack.remove(c)
                blank = False
            else:
                self.puzzle.rack.remove('?')
                blank = True
            
            self.square_at.config(text=c, fg='orange')
            if blank:
                self.square_at.config(borderwidth=1, relief=tk.RIDGE)
            self.squares_changed.append(self.square_at)
            sq = None
            while self.square_at['text'].strip() and sq != self.square_at:
                sq, self.square_at = self.square_at, self.next_tile(self.square_at, self.current_direction)
            if sq != self.square_at:
                self.square_at.config(text=self.current_direction)
                self.squares_changed.append(self.square_at)

    def location_of(self, tile):
        return [(row, col) for row, i in enumerate(self.labels) for col, j in enumerate(i) if j == tile][0]

    def enter_move(self, e):
        first = self.squares_changed[0]
        loc = self.location_of(first)
        word = ''
        sq = None
        f2 = first
        sq, f2 = f2, self.next_tile(f2, self.current_direction, opp=-1)
        while f2['text'].isalpha() and sq != f2:
            t = f2['text']
            if self.is_blank(f2): t = t.lower()
            word += ')' + t + '('
            sq, f2 = f2, self.next_tile(f2, self.current_direction, opp=-1)
        if sq != f2:
            f2 = self.next_tile(f2, self.current_direction)
        loc = self.location_of(f2)
        word = word[::-1]
        sq = None
        while first['text'].isalpha() and sq != first:
            t = first['text']
            if self.is_blank(first): t = t.lower()
            if first in self.squares_changed:
                word += t
            else:
                word += '(' + t + ')'
            sq, first = first, self.next_tile(first, self.current_direction)
        word = word.replace(')(', '')
        
        if r := self.puzzle.rank_of_move(loc, word, self.current_direction):
            if self.can_submit:
                self.moves_submitted.append(r)
            else:
                self.show(r - 1)(human=True)

    def next_tile(self, tile, direction, opp=1):
        a, b = self.location_of(tile)
        return self.labels[max(1, min(a + direction.y * opp, 15))][max(1, min(b + direction.x * opp, 15))]

    def show(self, i):
        def clicked(human=False, hidden=False):
            if self.can_submit: return
            
            move = self.puzzle.moves[i]
            diff = round(float(self.puzzle.moves[0].split()[-1]) - float(move.split()[-1]), 2)
            self.move_btns[i]['text'] = (f'[-{diff}] ' + str(i + 1).zfill(3) + '. ' + move).ljust(self.ml + 5)
            if human:
                self.move_btns[i]['fg'] = 'red'
                self.move_box.canvas.yview_moveto(i / len(self.puzzle.moves))
            else:
                self.place(move)
        return clicked

    def place(self, move):
        for sq in self.squares_changed:
            self.reset(sq)
        pos, word, *_ = move.split()

        if pos[0].isalpha():
            c, *r = pos
            direction = Direction.DOWN
        else:
            *r, c = pos
            direction = Direction.ACROSS
            
        first = self.labels[int(''.join(r))][string.ascii_uppercase.index(c) + 1]

        set_text = True
        for i in word:
            if i == '(':
                set_text = False
            elif i == ')':
                set_text = True
            else:
                if set_text:
                    first['text'] = i
                    first['fg'] = 'orange'
                    self.squares_changed.append(first)
                    if i == i.lower():
                        first.config(borderwidth=1, relief=tk.RIDGE, text=i.upper())
                        
                first = self.next_tile(first, direction)
                
    def start_timer(self):
        curr = self.max_time - self.time_passed
        if curr >= 0:
            self.time_passed += self.tick

            self.timer.config(text="{0:0>2d}:{1:0>2d}".format(curr // 60000, (curr % 60000) // 1000))
            self.root.after(self.tick, self.start_timer)
        else:
            self.enter_select()

    def enter_select(self):
        self.can_submit = False

        # print("Found moves", sorted(self.moves_submitted))
        
        # for i in self.moves_submitted:
        #     self.show(i - 1)(human=True, hidden=True)

        self.timer.pack_forget()

        if not self.lb:
            self.lb = tk.Listbox(self.timer_frame)
        else:
            self.lb.delete(0, tk.END)
        
        for i in self.moves_submitted:
            self.lb.insert(tk.END, ' '.join(self.puzzle.moves[i - 1].split()[:-1]).ljust(self.ml + 5))
            
        self.lb.pack()

        self.sub = tk.Button(self.timer_frame, text='Choose', command=self.submit_move)
        self.sub.pack()

    def submit_move(self):
        sel = self.lb.curselection()[0]
        i = self.moves_submitted[sel]
        self.show(i - 1)(human=True)
        self.sub.config(text='Done', command=self.finish)

    def finish(self):
        self.lb.pack_forget()
        self.sub.pack_forget()
        self.timer.pack()

        print("Found moves", sorted(self.moves_submitted))
        for i in self.moves_submitted:
            self.show(i - 1)(human=True)
        self.show(0)
        
    
    
g = GUI(difficulty=1)
g.root.mainloop()


'''
check! todo: blanks on board
check! todo: scroll to move
check! todo: blanks on rack
check! todo: reveal top move
todo: backspace
check! todo: place on click
chcek! todo: froms :(
(hopefully) check! todo: through blanks
check! todo: customizable difficulty
todo: timer
todo: scores (not like move scores, skill scores based on completion)
todo: rate puzzle by difficulty
todo: leave analysis graph (bar graph of each possible leave)
'''
