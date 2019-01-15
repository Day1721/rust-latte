use super::data::types::*;
use std::cmp;
use colored::*;

static ROW_MARGIN: usize = 3;

pub struct ErrorPrinter {
    lines: Vec<String>
}

impl ErrorPrinter {
    pub fn new(code: &String) -> ErrorPrinter {
        ErrorPrinter {
            lines: code.split("\n").map(|s|s.to_string()).collect()
        }
    }

    pub fn print(&self, position: Position, message: String) {
        use self::cmp::*;

        let fst = self.position_to_row_col(*position.l());
        let snd = self.position_to_row_col(*position.r() - 1);
        self.print_lines(fst.row.saturating_sub(ROW_MARGIN), fst.row);
        if fst.row != snd.row {
            self.print_multiline(fst, snd);
        } else {
            self.print_inline(fst.row, fst.col, snd.col);
        }
        self.print_lines(snd.row + 1, min(snd.row + ROW_MARGIN, self.lines.len()));
        println!("{}", message);
    }

    fn print_multiline(&self, start: Coordinate, end: Coordinate) {
        println!("{}", format!("/-{}v", "-".repeat(start.col)).yellow().bold());
        for i in start.row..(end.row+1) {
            println!("{} {}", "|".yellow().bold(), self.lines[i]);
        }
        println!("{}", format!("\\-{}^", "-".repeat(end.col)).yellow().bold());
    }

    fn print_inline(&self, row: usize, from: usize, to: usize) {
        self.print_lines(row, row + 1);
        let underline = if from == to {
            "^".to_string()
        } else {
            format!( "^{}^", "-".repeat(to - from - 1))
        }.yellow().bold();
        println!("  {}{}", " ".repeat(from), underline);
    }

    fn position_to_row_col(&self, pos: usize) -> Coordinate {
        let mut col = pos;
        let mut row = 0;
        for line in self.lines.iter() {
            if col > line.len() {
                col = col - line.len() - 1;
                row = row + 1;
            } else {
                break;
            }
        }
        Coordinate::new(row, col)
    }

    fn print_lines(&self, beg: usize, end: usize) {
        for i in beg..end {
            println!("  {}", self.lines[i]);
        }
    }
}

#[derive(new, Copy, Clone)]
struct Coordinate {
    pub row: usize,
    pub col: usize
}