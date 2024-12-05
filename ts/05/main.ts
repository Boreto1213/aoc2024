import * as fs from "fs";

type Rule = [number, number];
type Matrix = number[][];

const dataExample = fs.readFileSync("./ts/05/ex-pt1.txt", "utf-8");
const data = fs.readFileSync("./ts/05/data.txt", "utf-8");

const [rulesStr, pagesStr] = data.split(".").map((str) => str.trim());

const rules: Rule[] = rulesStr.split("\n").map((line) => {
	const [before, after] = line.split("|").map((num) => Number(num));

	return [before, after];
});

const pages = pagesStr
	.split("\n")
	.map((line) => line.split(",").map((num) => Number(num)));

console.log("Pt1 result: ", pt1(rules, pages));
console.log("Pt2 result: ", pt2(rules, pages));

function pt1(rules: Rule[], pages: Matrix) {
	const correctPages: Matrix = [];
	for (const page of pages) {
		const pageCopy: number[] = [...page];
		const pageRules: Rule[] = rules.filter((rule) =>
			pageCopy.some((n) => rule.includes(n)),
		);

		for (const [left, right] of pageRules) {
			const leftNumIndx = pageCopy.indexOf(left);
			const rightNumIndx = pageCopy.indexOf(right);

			if (leftNumIndx < 0 || rightNumIndx < 0) continue;

			if (leftNumIndx > rightNumIndx) {
				pageCopy[leftNumIndx] = right;
				pageCopy[rightNumIndx] = left;
			}
		}

		if (areEqual(page, pageCopy)) correctPages.push(page);
	}

	return correctPages.reduce((acc, curr) => {
		return acc + curr[(curr.length - 1) / 2];
	}, 0);
}

function pt2(rules: Rule[], pages: Matrix) {
	const incorrectPages: Matrix = [];
	for (const page of pages) {
		const pageCopy: number[] = [...page];
		const pageRules: Rule[] = rules.filter((rule) =>
			pageCopy.some((n) => rule.includes(n)),
		);

		while (!allRulesValid(pageCopy, pageRules)) {
			for (const [left, right] of pageRules) {
				const leftNumIndx = pageCopy.indexOf(left);
				const rightNumIndx = pageCopy.indexOf(right);

				if (leftNumIndx < 0 || rightNumIndx < 0) continue;

				if (leftNumIndx > rightNumIndx) {
					pageCopy[leftNumIndx] = right;
					pageCopy[rightNumIndx] = left;
				}
			}
		}

		if (!areEqual(page, pageCopy)) incorrectPages.push(pageCopy);
	}

	return incorrectPages.reduce((acc, curr) => {
		return acc + curr[(curr.length - 1) / 2];
	}, 0);
}

function allRulesValid(page: number[], rules: Rule[]) {
	for (const [left, right] of rules) {
		const leftNumIndx = page.indexOf(left);
		const rightNumIndx = page.indexOf(right);

		if (leftNumIndx < 0 || rightNumIndx < 0) continue;

		if (leftNumIndx > rightNumIndx) return false;
	}

	return true;
}

function areEqual(actual: number[], expected: number[]) {
	for (let i = 0; i < actual.length; i++) {
		if (actual[i] !== expected[i]) {
			return false;
		}
	}
	return true;
}
