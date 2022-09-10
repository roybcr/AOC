type Point = [number, number];

const getInput = () => {
    return 'forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2';
};

function parseLine(line: string): Point {
    const [dir, a]: string[] = line.split(' ');
    const amount: number = +a;

    switch (dir) {
        case 'forward':
            return [amount, 0];
        case 'up':
            return [0, -amount];
        default:
            return [0, amount];
    }
}

export function submarine() {
  const out: Point = getInput()
      .split('\n')
      .map((x: string) => parseLine(x))
      .reduce(
          (acc: Point, amount: Point) => {
              acc[0] += amount[0];
              acc[1] += amount[1];
              return acc;
          },
          [0, 0]
      );

  console.log(out, out[0] * out[1]);
}
