const getInput = () => {
    return `forward 5
down 5
forward 8
up 3
down 8
forward 2`;
};

function parseLine(line: string): [number, number] {
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

const out: [number, number] = getInput()
    .split('\n')
    .map((x: string) => parseLine(x))
    .reduce(
        (acc: [number, number], amount: [number, number]) => {
            acc[0] += amount[0];
            acc[1] += amount[1];
            return acc;
        },
        [0, 0]
    );

console.log(out, out[0] * out[1]);
