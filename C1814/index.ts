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

  const tests = inputData.slice(1);
};

main();
