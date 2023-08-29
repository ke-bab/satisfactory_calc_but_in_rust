import  Square from "./Square"
import { useState } from 'react';
import Game from "./core";

export default function Board() {
  const game = Game.new();
  const [squares, setSquares] = useState(game.getState());
//  game.setStateHasChangedClosure(() => {
//	const nextSquares = game.squares.slice();
//   	setSquares(nextSquares);
//  });


  return (
          <>
                  <div className="board-row">
                    <Square value={game.squares[0]} onSquareClick={() => game.doNextMove(0)} />
                    <Square value={game.squares[1]} onSquareClick={() => game.doNextMove(1)} />
                    <Square value={game.squares[2]} onSquareClick={() => game.doNextMove(2)} />
                  </div>
                  <div className="board-row">
                    <Square value={game.squares[3]} onSquareClick={() => game.doNextMove(3)} />
                    <Square value={game.squares[4]} onSquareClick={() => game.doNextMove(4)} />
                    <Square value={game.squares[5]} onSquareClick={() => game.doNextMove(5)} />
                  </div>
                  <div className="board-row">
                    <Square value={game.squares[6]} onSquareClick={() => game.doNextMove(6)} />
                    <Square value={game.squares[7]} onSquareClick={() => game.doNextMove(7)} />
                    <Square value={game.squares[8]} onSquareClick={() => game.doNextMove(8)} />
                  </div>
          </>
  )
}
