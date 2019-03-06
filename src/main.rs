use ncurses::initscr;
use ncurses::refresh;
use ncurses::getch;
use ncurses::endwin;
use std::time::Instant;

#[derive(Debug)]
struct Press {
  time: Instant,
  key: i32,
}

fn main() {
  initscr();
  refresh();
  let mut presses = vec![];
  loop {
    let x = getch();
    if x == 10 {
      break;
    }
    presses.push(Press {
      time: Instant::now(),
      key: x,
    });
  }
  endwin();

  let orig_text: String = presses.iter().map(|x| x.key as u8 as char).collect();


  let start = presses.first().unwrap().time;
  let end = presses.last().unwrap().time;
  let duration = end - start;
  let duration_sec = duration.as_millis() as f64 / 1000.0;

  let count = presses.len() as f64;

  let cps = count / duration_sec;
  let wpm = cps * 12.0;

  let mut get_start = true;
  let mut word_start = presses.first().unwrap().time;
  let mut word_start_i = 0;
  let mut word_end = presses.first().unwrap().time;
  let mut word_end_i = 0;

  for (i, press) in presses.iter().enumerate() {
    if get_start {
      word_start = press.time;
      word_start_i = i;
      get_start = false;
    }
    if press.key == 32 {
      word_end = press.time;
      word_end_i = i;
      get_start = true;

      let word_duration = word_end - word_start;
      let word_duration_sec = word_duration.as_millis() as f64 / 1000.0;
      let word_len = (word_end_i - word_start_i) as f64;
      let word_cps = word_len / word_duration_sec;
      let word_wpm = word_cps * 12.0;

      println!("{:>16}   {:.3} sec   {:.1} wpm", &orig_text[word_start_i..word_end_i], word_duration_sec, word_wpm);
    }
  }

  word_end = end;
  word_end_i = presses.len();

  let word_duration = word_end - word_start;
  let word_duration_sec = word_duration.as_millis() as f64 / 1000.0;
  let word_len = (word_end_i - word_start_i) as f64;
  let word_cps = word_len / word_duration_sec;
  let word_wpm = word_cps * 12.0;

  println!("{:>16}   {:.3} sec   {:.1} wpm", &orig_text[word_start_i..word_end_i], word_duration_sec, word_wpm);

  println!("Original text: {}", orig_text);
  println!("{:.2} CPS   {:.1} WPM", cps, wpm);
}
