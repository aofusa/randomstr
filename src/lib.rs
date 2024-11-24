
use rand::Rng;

pub fn randomstr(n: usize) -> String {
  const ASCII_SIZE: usize = 26;
  const ASCII_TABLE: [char; ASCII_SIZE] = [
    'a', 'b', 'c', 'd', 'e',
    'f', 'g', 'h', 'i', 'j',
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't',
    'u', 'v', 'w', 'x', 'y', 'z'
  ];
  let mut rng = rand::thread_rng();

  let mut str_arr = vec!['\0'; n];
  (0..n).for_each(|idx| {
    let i = rng.gen_range(0..ASCII_SIZE);
    str_arr[idx] = ASCII_TABLE[i];
  });

  String::from_iter(str_arr)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_randomstr() {
    let a = randomstr(10);
    assert_eq!(a.len(), 10);

    let b = randomstr(3);
    assert_ne!(a, b);
  }
}
