function largest<T extends number | string>(list: T[]): T {
  let largest = list[0];
  for (let item of list) {
      if (item > largest) {
          largest = item;
      }
  }
  return largest;
}

function main() {
  const numberList: number[] = [34, 50, 25, 100, 65];
  const numberResult = largest(numberList);
  console.log(`The largest number is ${numberResult}`);

  const charList: string[] = ['y', 'm', 'a', 'q'];
  const charResult = largest(charList);
  console.log(`The largest char is ${charResult}`);
}

main();