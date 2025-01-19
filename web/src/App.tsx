import { useState } from "react";

import "./App.css";

function App() {
  const handleClick = () => {};

  return (
    <>
      <div className="absolute top-0 right-0 w-48 p-8 h-screen flex flex-col justify-around">
        <button
          className="h-20 w-20 rounded-lg bg-pink-400 transition-all hover:scale-110 hover:bg-pink-700"
          onClick={() => handleClick()}
        >
          Dab
        </button>
        <button
          className="h-20 w-20 rounded-lg bg-pink-400 transition-all hover:scale-110 hover:bg-pink-700"
          onClick={() => handleClick()}
        ></button>
        <button
          className="h-20 w-20 rounded-lg bg-pink-400 transition-all hover:scale-110 hover:bg-pink-700"
          onClick={() => handleClick()}
        >
          Hi
        </button>
        <button
          className="h-20 w-20 rounded-lg bg-pink-400 transition-all hover:scale-110 hover:bg-pink-700"
          onClick={() => handleClick()}
        >
          Hi
        </button>
      </div>
      {/* <button className="absolute bottom-16 right-12 h-20 w-20 rounded-lg  bg-pink-400 transition-all hover:scale-110 hover:bg-pink-700"
    onClick={() => handleClick(1)}>Hi</button>
    <button className="absolute bottom-40 right-12 h-20 w-20 rounded-lg  bg-pink-400 transition-all hover:scale-110 hover:bg-pink-700"
    onClick={() => handleClick(2)}>Hi</button>
    <button className="absolute bottom-64 right-12 h-20 w-20 rounded-lg  bg-pink-400 transition-all hover:scale-110 hover:bg-pink-700"
    onClick={() => handleClick(3)}>Hi</button>

    <button className="absolute top-60 right-12 h-20 w-20 rounded-lg  bg-pink-400 transition-all hover:scale-110 hover:bg-pink-700"
    onClick={() => handleClick(4)}>Hi</button>
    <button className="absolute top-36 right-12 h-20 w-20 rounded-lg  bg-pink-400 transition-all hover:scale-110 hover:bg-pink-700"
    onClick={() => handleClick(3)}>Hi</button> */}
      {/* <button
        className="absolute top-12 right-12 h-20 w-20 rounded-lg  bg-pink-400 transition-all hover:scale-110 hover:bg-pink-700"
        onClick={() => handleClick(4)}
      >
        Hi
      </button>

      <button
        className="absolute bottom-16 right-40 h-20 w-20 rounded-lg  bg-purple-300 transition-all hover:scale-110 hover:bg-blue-300"
        onClick={() => handleClick(2)}
      >
        Hi
      </button>
      <button
        className="absolute bottom-16 right-64 h-20 w-20 rounded-lg  bg-purple-300 transition-all hover:scale-110 hover:bg-blue-300"
        onClick={() => handleClick(3)}
      >
        Hi
      </button>
      <button
        className="absolute bottom-16 right-96 h-20 w-20 rounded-lg  bg-purple-300 transition-all hover:scale-110 hover:bg-blue-300"
        onClick={() => handleClick(4)}
      >
        Hi
      </button> */}
    </>
  );
}

export default App;
