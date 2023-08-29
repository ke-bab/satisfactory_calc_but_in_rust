
export default function Square({value, onSquareClick}) {
	console.log(value);
  return <button onClick={onSquareClick}>{value.internalValue}</button>;
}