const playerX = "X";
const playerO = "O";
const NULL = "NULL";
const SIZE = 9;
const MIN = 0;
const MAX = 8;

export default class Game {
	squares;
	currentPlayer = playerX;
	stateChangedClosure = null;

	constructor() {
		this.squares = Array(SIZE).fill(null).map(n => new Square());
  	}

	static new() {
		return new Game();
	}

	/**
	 * @param {number} pos
	 */
	doNextMove(pos) {
		Game.validatePos(pos);
		if (!this.squares[pos].isEmpty()) {
			throw new Error ("this square is full");
		}
		console.log(JSON.stringify(this.squares));

		this.squares[pos].internalValue = this.currentPlayer;
		console.log(JSON.stringify(this.squares));

		if (typeof(this.stateChangedClosure) === "function") {
			this.stateChangedClosure();
		}

		console.log("value at pos " + pos + " is " + this.currentPlayer);
		this.changeCurrentPlayer();
	}

	changeCurrentPlayer() {
		if (this.currentPlayer === playerX) {
			console.log("changed player to 0");
			this.currentPlayer = playerO;
		} else {
			console.log("changed player to x");
			this.currentPlayer = playerX;
		}
	}

	static validatePos(pos) {
		if (pos > MAX || pos < MIN) {
			console.log("pos out of bounds: " + pos);
			throw new Error("pos out of bounds: " + pos);
		}
	}

	// contract with react
	getState() {
		return this.squares;
	}

	// contract with react
	setStateHasChangedClosure(closure) {
		this.stateChangedClosure = closure;
	}
}

class Square {
	internalValue = NULL;

	isX() {
		return this.internalValue === playerX;
	}

	isO() {
		return this.internalValue === playerO;
	}

	isEmpty() {
		return this.internalValue === NULL;
	}
}