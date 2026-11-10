//! The xmonad tiling algorithm, ported from `XMonad/Layout.hs` and the stack
//! operations of `XMonad/StackSet.hs` (xmonad v0.18.1, BSD-3-Clause, vendored
//! at `repos/xmonad`). Function names follow the Haskell source so the two can
//! be read side by side; integer arithmetic matches it exactly.

/// `Rectangle` of X11: position and size in pixels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
}

impl Rect {
    pub const fn new(x: i32, y: i32, w: u32, h: u32) -> Self {
        Self { x, y, w, h }
    }

    pub fn contains(&self, px: i32, py: i32) -> bool {
        px >= self.x && py >= self.y && px < self.x + self.w as i32 && py < self.y + self.h as i32
    }

    /// The rectangle shrunk by `by` pixels on every side.
    pub fn inset(&self, by: u32) -> Self {
        let by = by.min(self.w / 2).min(self.h / 2);
        Self::new(self.x + by as i32, self.y + by as i32, self.w - 2 * by, self.h - 2 * by)
    }
}

/// `mirrorRect`: swap the axes.
pub fn mirror_rect(r: Rect) -> Rect {
    Rect::new(r.y, r.x, r.h, r.w)
}

/// `splitVertically`: `n` rows; each takes an equal share of what is left, so
/// the remainder pixels end up in the last rows.
pub fn split_vertically(n: u32, r: Rect) -> Vec<Rect> {
    let mut out = Vec::with_capacity(n.max(1) as usize);
    let (mut n, mut r) = (n, r);
    while n >= 2 {
        let smallh = r.h / n;
        out.push(Rect::new(r.x, r.y, r.w, smallh));
        r = Rect::new(r.x, r.y + smallh as i32, r.w, r.h - smallh);
        n -= 1;
    }
    out.push(r);
    out
}

/// `splitHorizontallyBy`: a left part `percent`/100 of the width, and the rest.
pub fn split_horizontally_by(percent: u32, r: Rect) -> (Rect, Rect) {
    let leftw = (u64::from(r.w) * u64::from(percent) / 100) as u32;
    (Rect::new(r.x, r.y, leftw, r.h), Rect::new(r.x + leftw as i32, r.y, r.w - leftw, r.h))
}

/// `tile`: the master pane holds `nmaster` windows, the rest stack beside it.
pub fn tile(percent: u32, r: Rect, nmaster: u32, n: u32) -> Vec<Rect> {
    if n <= nmaster || nmaster == 0 {
        split_vertically(n, r)
    } else {
        let (r1, r2) = split_horizontally_by(percent, r);
        let mut out = split_vertically(nmaster, r1);
        out.extend(split_vertically(n - nmaster, r2));
        out
    }
}

/// `Tall`: xmonad's default layout. The ratio is kept in hundredths because
/// every value xmonad reaches from 1/2 in steps of 3/100 is one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tall {
    /// Windows in the master pane (default 1).
    pub nmaster: u32,
    /// Percent of the screen to resize by (default 3).
    pub delta: u32,
    /// Percent of the screen the master pane takes (default 50).
    pub ratio: u32,
}

impl Default for Tall {
    fn default() -> Self {
        Self { nmaster: 1, delta: 3, ratio: 50 }
    }
}

impl Tall {
    /// `pureLayout`: a rectangle for each of `n` windows in stack order, or
    /// `None` for the windows a ratio of 0 or 1 hides.
    pub fn layout(&self, r: Rect, n: u32) -> Vec<Option<Rect>> {
        let rects = tile(self.ratio, r, self.nmaster, n);
        let nmaster = self.nmaster as usize;
        rects
            .into_iter()
            .enumerate()
            .map(|(i, rect)| match self.ratio {
                0 if i < nmaster => None,
                100 if i >= nmaster => None,
                _ => Some(rect),
            })
            .collect()
    }

    /// `Shrink`.
    pub fn shrink(&mut self) {
        self.ratio = self.ratio.saturating_sub(self.delta);
    }

    /// `Expand`.
    pub fn expand(&mut self) {
        self.ratio = (self.ratio + self.delta).min(100);
    }

    /// `IncMasterN d`.
    pub fn inc_master(&mut self, d: i32) {
        self.nmaster = self.nmaster.saturating_add_signed(d);
    }
}

/// xmonad's default `layoutHook`: `tiled ||| Mirror tiled ||| Full`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Choice {
    #[default]
    Tall,
    MirrorTall,
    Full,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Layout {
    pub tall: Tall,
    pub choice: Choice,
}

impl Layout {
    /// `NextLayout`, wrapping.
    pub fn next(&mut self) {
        self.choice = match self.choice {
            Choice::Tall => Choice::MirrorTall,
            Choice::MirrorTall => Choice::Full,
            Choice::Full => Choice::Tall,
        };
    }

    /// `description`.
    pub fn description(&self) -> &'static str {
        match self.choice {
            Choice::Tall => "Tall",
            Choice::MirrorTall => "Mirror Tall",
            Choice::Full => "Full",
        }
    }

    /// `runLayout`: where each of `n` windows goes (stack order, `focus` is
    /// the focused index); `None` means not shown.
    pub fn run(&self, r: Rect, n: u32, focus: usize) -> Vec<Option<Rect>> {
        match self.choice {
            Choice::Tall => self.tall.layout(r, n),
            Choice::MirrorTall => self.tall.layout(mirror_rect(r), n).into_iter().map(|rect| rect.map(mirror_rect)).collect(),
            Choice::Full => (0..n as usize).map(|i| (i == focus).then_some(r)).collect(),
        }
    }
}

/// `Stack`: the windows of a workspace with one focused. xmonad keeps a zipper
/// (`up`, `focus`, `down`); this is its `integrate`d list plus the focus index,
/// with the same operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stack<T> {
    pub items: Vec<T>,
    pub focus: usize,
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self { items: Vec::new(), focus: 0 }
    }
}

impl<T: PartialEq> Stack<T> {
    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn focused(&self) -> Option<&T> {
        self.items.get(self.focus)
    }

    /// `focusUp'`: wraps from the top to the bottom.
    pub fn focus_up(&mut self) {
        if !self.items.is_empty() {
            self.focus = (self.focus + self.items.len() - 1) % self.items.len();
        }
    }

    /// `focusDown'`.
    pub fn focus_down(&mut self) {
        if !self.items.is_empty() {
            self.focus = (self.focus + 1) % self.items.len();
        }
    }

    /// `focusWindow`.
    pub fn focus_item(&mut self, item: &T) {
        if let Some(i) = self.items.iter().position(|x| x == item) {
            self.focus = i;
        }
    }

    /// `swapMaster`: the focused window trades places with the master.
    pub fn swap_master(&mut self) {
        if self.focus < self.items.len() {
            self.items.swap(0, self.focus);
            self.focus = 0;
        }
    }

    /// `swapUp'`: the focused window moves up one place; from the top it
    /// wraps to the bottom.
    pub fn swap_up(&mut self) {
        let n = self.items.len();
        if n < 2 {
            return;
        }
        if self.focus == 0 {
            self.items.rotate_left(1);
            self.focus = n - 1;
        } else {
            self.items.swap(self.focus, self.focus - 1);
            self.focus -= 1;
        }
    }

    /// `swapDown'`.
    pub fn swap_down(&mut self) {
        let n = self.items.len();
        if n < 2 {
            return;
        }
        if self.focus == n - 1 {
            self.items.rotate_right(1);
            self.focus = 0;
        } else {
            self.items.swap(self.focus, self.focus + 1);
            self.focus += 1;
        }
    }

    /// Reconciles with the channel: drops windows that left (`delete'`: focus
    /// moves down, else up) and adds newcomers at the bottom of the stack.
    /// xmonad's `insertUp` would focus each newcomer and make it master when
    /// the master is focused; a call must not reshuffle on every join.
    pub fn sync(&mut self, present: &[T])
    where
        T: Clone,
    {
        let mut i = 0;
        while i < self.items.len() {
            if present.contains(&self.items[i]) {
                i += 1;
                continue;
            }
            self.items.remove(i);
            if self.focus > i || (self.focus == i && i == self.items.len()) {
                self.focus = self.focus.saturating_sub(1);
            }
        }
        for item in present {
            if !self.items.contains(item) {
                self.items.push(item.clone());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SCREEN: Rect = Rect::new(0, 0, 1920, 1080);

    #[test]
    fn one_window_fills_the_screen() {
        assert_eq!(tile(50, SCREEN, 1, 1), vec![SCREEN]);
    }

    #[test]
    fn master_and_stack() {
        assert_eq!(tile(50, SCREEN, 1, 3), vec![Rect::new(0, 0, 960, 1080), Rect::new(960, 0, 960, 540), Rect::new(960, 540, 960, 540),]);
    }

    #[test]
    fn remainder_pixels_go_to_the_last_rows() {
        // 1080 / 7 = 154 r 2, as xmonad's recursive div does it.
        let rows = split_vertically(7, SCREEN);
        let heights: Vec<u32> = rows.iter().map(|r| r.h).collect();
        assert_eq!(heights, vec![154, 154, 154, 154, 154, 155, 155]);
        assert_eq!(rows.last().unwrap().y + rows.last().unwrap().h as i32, 1080);
    }

    #[test]
    fn tiles_cover_the_screen_without_overlap() {
        for n in 1..=12u32 {
            for nmaster in 0..=3 {
                let rects = tile(53, Rect::new(7, 11, 1367, 769), nmaster, n);
                assert_eq!(rects.len(), n as usize);
                let area: u64 = rects.iter().map(|r| u64::from(r.w) * u64::from(r.h)).sum();
                assert_eq!(area, 1367 * 769, "n={n} nmaster={nmaster}");
            }
        }
    }

    #[test]
    fn mirror_puts_the_master_on_top() {
        let layout = Layout { choice: Choice::MirrorTall, ..Layout::default() };
        assert_eq!(layout.run(SCREEN, 2, 0), vec![Some(Rect::new(0, 0, 1920, 540)), Some(Rect::new(0, 540, 1920, 540))]);
    }

    #[test]
    fn full_shows_only_the_focused_window() {
        let layout = Layout { choice: Choice::Full, ..Layout::default() };
        assert_eq!(layout.run(SCREEN, 3, 1), vec![None, Some(SCREEN), None]);
    }

    #[test]
    fn resize_is_clamped() {
        let mut tall = Tall::default();
        for _ in 0..40 {
            tall.expand();
        }
        assert_eq!(tall.ratio, 100);
        assert_eq!(tall.layout(SCREEN, 3), vec![Some(Rect::new(0, 0, 1920, 1080)), None, None]);
        for _ in 0..40 {
            tall.shrink();
        }
        assert_eq!(tall.ratio, 0);
        assert_eq!(tall.layout(SCREEN, 3)[0], None);
    }

    #[test]
    fn stack_operations_match_xmonad() {
        let mut s = Stack { items: vec!['a', 'b', 'c', 'd'], focus: 2 };
        s.swap_master();
        assert_eq!((s.items.clone(), s.focus), (vec!['c', 'b', 'a', 'd'], 0));
        s.focus_up();
        assert_eq!(s.focus, 3);
        s.focus_down();
        assert_eq!(s.focus, 0);
        s.swap_up(); // from the top: wraps to the bottom
        assert_eq!((s.items.clone(), s.focus), (vec!['b', 'a', 'd', 'c'], 3));
        s.swap_down(); // from the bottom: wraps to the top
        assert_eq!((s.items.clone(), s.focus), (vec!['c', 'b', 'a', 'd'], 0));
    }

    #[test]
    fn sync_keeps_order_and_focus() {
        let mut s = Stack { items: vec![1, 2, 3], focus: 1 };
        s.sync(&[3, 1, 4]);
        assert_eq!((s.items.clone(), s.focus), (vec![1, 3, 4], 1));
        s.focus = 2;
        s.sync(&[1, 3]);
        assert_eq!((s.items.clone(), s.focus), (vec![1, 3], 1));
    }
}
