class Point<X1, Y1> {
  x: X1;
  y: Y1;

  constructor(x: X1, y: Y1) {
      this.x = x;
      this.y = y;
  }

  mixup<X2, Y2>(other: Point<X2, Y2>): Point<X1, Y2> {
      return new Point(this.x, other.y);
  }
}

function main() {
  let p1 = new Point(5, 10.4);
  let p2 = new Point("Hello", 'c');
  let p3 = p1.mixup(p2);

  console.log(`p3.x = ${p3.x}, p3.y = ${p3.y}`);
}

main();