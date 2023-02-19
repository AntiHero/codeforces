import * as readline from 'readline';

const rl = readline.createInterface({
  input: process.stdin,
});

let t: number, n: number, m: number;

let t1: string = '';
let t2: string = '';

rl.on('line', (line) => {
  if (!t) {
    t = parseInt(line, 10);
  } else if (!n && !m) {
    [n, m] = line.split(' ').map(Number);
  } else if (!t1) {
    t1 = line;
  } else if (!t2) {
    t2 = line;
  }

  if (t1 && t2) {
    t--;

    isBeautiful(t1, t2);

    (t1 = ''), (t2 = ''), (n = m = 0);
    if (!t) rl.close();
  }
});

function isBeautiful(t1: string, t2: string): void {
  // const str = t1 + t2.split('').map((_, i) => t2.at(~i)).join('');
  const str = t1 + t2.split('').reverse().join('');

  let n = 0;

  for (let i = 0; i < str.length; i++) {
    if (str[i] == str[i + 1]) n++;
    if (n >= 2) {
      return console.log('NO');
    }
  }

  return console.log('YES');
}
