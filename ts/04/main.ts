import * as fs from "fs";

const dataExample = fs.readFileSync("./ts/04/ex-pt2.txt", "utf-8");
const data = fs.readFileSync("./ts/04/data.txt", "utf-8");

const matrix = data.split("\n").map((row) => row.split(""));
type TupleArray = Array<Array<[number, number]>>;

// console.log(`Part 1 result: `, pt1(matrix), '\n')
console.log(`Part 2 result: `, pt2(matrix), "\n");

function pt1(matrix: string[][]) {
	const words: TupleArray = [];

	for (let y = 0; y < matrix.length; y++) {
		for (let x = 0; x < matrix[y].length; x++) {
			checkForXmas(x, y, matrix, words);
		}
	}

	return words.length;
}

function pt2(matrix: string[][]) {
	let xMasCount = { count: 0 };

	for (let y = 0; y < matrix.length - 2; y++) {
		for (let x = 0; x < matrix[y].length - 2; x++) {
			checkForXmasPt2(x, y, matrix, xMasCount);
		}
	}

	return xMasCount.count;
}

function checkForXmas(
	x: number,
	y: number,
	matrix: string[][],
	words: TupleArray,
) {
	if (!["X", "S"].includes(matrix[y][x])) {
		return;
	}

	const targetWord = matrix[y][x] === "X" ? "XMAS" : "SAMX";

	const current: {
		x: [number, number][];
		y: [number, number][];
		dr: [number, number][];
		dl: [number, number][];
	} = {
		x: [],
		y: [],
		dr: [],
		dl: [],
	};

	for (let indx = 0; indx < targetWord.length; indx++) {
		if (x + indx === matrix[y].length) break;
		if (matrix[y][x + indx] !== targetWord[indx]) break;

		current.x.push([y, x + indx]);
	}

	for (let indx = 0; indx < targetWord.length; indx++) {
		if (y + indx === matrix.length) break;
		if (matrix[y + indx][x] !== targetWord[indx]) break;

		current.y.push([y + indx, x]);
	}

	for (let indx = 0; indx < targetWord.length; indx++) {
		if (x + indx === matrix[y].length) break;
		if (y + indx === matrix.length) break;
		if (matrix[y + indx][x + indx] !== targetWord[indx]) break;

		current.dr.push([y + indx, x + indx]);
	}

	for (let indx = 0; indx < targetWord.length; indx++) {
		if (x - indx < 0) break;
		if (y + indx === matrix.length) break;
		if (matrix[y + indx][x - indx] !== targetWord[indx]) break;

		current.dl.push([y + indx, x - indx]);
	}

	for (const word of Object.values(current)) {
		if (word.length === targetWord.length) {
			words.push(word);
		}
	}
}

function checkForXmasPt2(
	x: number,
	y: number,
	matrix: string[][],
	xMasCount: { count: number },
) {
	if (!["M", "S"].includes(matrix[y][x])) return;

	let word1 = "";
	let word2 = "";
	const targets = ["MAS", "SAM"];

	for (let i = 0; i < 3; i++) {
		word1 += matrix[y + i][x + i];
		word2 += matrix[y + 2 - i][x + i];
	}

	if (targets.includes(word1) && targets.includes(word2)) {
		xMasCount.count += 1;
	}
}

function printMatrix(matrix: string[][]) {
	for (const row of matrix) {
		console.log(row.join(""));
	}
}
