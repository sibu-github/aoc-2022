import { readFile } from 'fs';

readFile('inputs.txt', 'utf8', (err, data) => {
  if (err) {
    console.log('not able to read file');
    return;
  }

  const lines = data.split('\n');
  let highest1 = 0;
  let highest2 = 0;
  let highest3 = 0;
  let sum = 0;

  lines.forEach((line) => {
    if (line == '') {
      if (sum > highest1) {
        highest3 = highest2;
        highest2 = highest1;
        highest1 = sum;
      } else if (sum > highest2) {
        highest3 = highest2;
        highest2 = sum;
      } else if (sum > highest3) {
        highest3 = sum;
      }
      sum = 0;
      return;
    }
    const num = Number(line);
    sum += num;
  });

  if (sum > highest1) {
    highest3 = highest2;
    highest2 = highest1;
    highest1 = sum;
  } else if (sum > highest2) {
    highest3 = highest2;
    highest2 = sum;
  } else if (sum > highest3) {
    highest3 = sum;
  }

  console.log('Highest is', highest1);
  console.log('Total of highest 3', highest1 + highest2 + highest3);
});
