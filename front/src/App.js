import  Square from "./Square"
import { useState } from 'react';
import Game from "./core";

export default function Board() {
  const game = Game.new();
  const [squares, setSquares] = useState(game.getState());
  game.setStateHasChangedClosure(() => {
	const nextSquares = game.squares.slice();
   	setSquares(nextSquares);
  });


  return (
          <>
                  <div className="board-row">
                    <Square value={squares[0]} onSquareClick={() => game.doNextMove(0)} />
                    <Square value={squares[1]} onSquareClick={() => game.doNextMove(1)} />
                    <Square value={squares[2]} onSquareClick={() => game.doNextMove(2)} />
                  </div>
                  <div className="board-row">
                    <Square value={squares[3]} onSquareClick={() => game.doNextMove(3)} />
                    <Square value={squares[4]} onSquareClick={() => game.doNextMove(4)} />
                    <Square value={squares[5]} onSquareClick={() => game.doNextMove(5)} />
                  </div>
                  <div className="board-row">
                    <Square value={squares[6]} onSquareClick={() => game.doNextMove(6)} />
                    <Square value={squares[7]} onSquareClick={() => game.doNextMove(7)} />
                    <Square value={squares[8]} onSquareClick={() => game.doNextMove(8)} />
                  </div>
          </>
  )
}
