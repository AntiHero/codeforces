import { stdin as input, stdout as output } from 'process';
import readline from 'readline';

const rl = readline.createInterface({ input, output, crlfDelay: Infinity });

const main = async () => {
  const inputData: string[] = [];

  await new Promise<void>((resolve) => {
    rl.on('line', (data) => {
      inputData.push(data);

      if (inputData.length === Number(inputData[0]) * 2 + 1) {
        resolve();
        rl.close();
      }
    });
  });

  const [testsNum] = inputData.splice(0, 1);

  for (const _ of Array(+testsNum)) {
    const [_, s1, s2] = inputData.splice(0, 1)[0].split(' ').map(Number);

    const r = inputData.splice(0, 1)[0].split(' ').map(Number);

    let l1 = [];
    let l2 = [];

    let t1 = s1;
    let t2 = s2;

    const sortedR = [...r].sort((a, b) => b - a);
    const map: Record<string, any> = {};

    for (let [key, el] of Object.entries(r)) {
      map[el] ??= [];
      map[el].push(+key + 1);
    }

    for (const el of sortedR) {
      const b = map[el].shift();

      t1 > t2 ? (l2.push(b), (t2 += s2)) : (l1.push(b), (t1 += s1));
    }

    console.log(`${l1.length} ${l1.join(' ')}`);
    console.log(`${l2.length} ${l2.join(' ')}`);
  }
};

main();
